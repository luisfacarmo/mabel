//! Device loop — connects to Soundcore headphone, processes packets, handles commands.
//!
//! Based on the proven baseus-desktop pattern: outer retry loop + inner select! loop.

use std::time::Duration;

use mabel_protocol::{
    models::a3062::{self, A3062State},
    Packet, PacketStream,
};
use mabel_transport::{RfcommTransport, WindowsRfcommTransport};
use serde::Serialize;
use tauri::{AppHandle, Emitter};
use tokio::sync::mpsc;
use tracing::{debug, info, warn};

const RETRY_DELAY: Duration = Duration::from_secs(5);
const READ_TIMEOUT: Duration = Duration::from_secs(15);

/// Commands sent from the frontend to the device loop.
#[derive(Debug)]
pub enum DeviceCommand {
    SetSoundModes(a3062::SoundModes),
    SetEqualizer(a3062::EqualizerConfig),
    SetModeCycle(a3062::AmbientSoundModeCycle),
    SetButtonConfig(a3062::ButtonConfig),
    SetAutoPowerOff(a3062::AutoPowerOff),
    SetLdac(bool),
    SetDolby(bool),
    SetSidetone(bool),
    Disconnect,
    Connect,
}

pub type CommandSender = mpsc::UnboundedSender<DeviceCommand>;
type CommandReceiver = mpsc::UnboundedReceiver<DeviceCommand>;

pub fn command_channel() -> (CommandSender, CommandReceiver) {
    mpsc::unbounded_channel()
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceStateEvent {
    pub battery: BatteryEvent,
    pub firmware: String,
    pub serial_number: String,
    pub sound_modes: SoundModesEvent,
    pub equalizer: EqualizerEvent,
    pub button_config: ButtonConfigEvent,
    pub ambient_sound_mode_cycle: ModeCycleEvent,
    pub toggles: TogglesEvent,
    pub auto_power_off: u16,
    pub limit_high_volume: LimitHighVolumeEvent,
    pub dual_connections: DualConnectionsEvent,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BatteryEvent { pub level: u8, pub max_level: u8, pub is_charging: bool }

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SoundModesEvent {
    pub ambient_sound_mode: String,
    pub noise_canceling_mode: String,
    pub adaptive_nc_level: u8,
    pub custom_nc_level: u8,
    pub custom_transparency: u8,
    pub wind_noise_reduction: bool,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EqualizerEvent { pub preset: Option<String>, pub bands: Vec<u8> }

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ButtonConfigEvent { pub double_press_action: Option<String> }

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ModeCycleEvent { pub noise_canceling: bool, pub transparency: bool, pub normal: bool }

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TogglesEvent {
    pub dolby_audio: bool,
    pub ldac: bool,
    pub side_tone: bool,
    pub voice_prompt: bool,
    pub low_battery_prompt: bool,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LimitHighVolumeEvent { pub enabled: bool, pub db_limit: u8 }

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DualConnectionsEvent {
    pub enabled: bool,
    pub devices: Vec<DualConnectionDeviceEvent>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DualConnectionDeviceEvent {
    pub name: String,
    pub connected: bool,
}

/// Main device loop. Runs forever, retrying on disconnect.
pub async fn run_loop(app: AppHandle, mut cmd_rx: CommandReceiver) {
    let transport = WindowsRfcommTransport::default();

    loop {
        let _ = app.emit("connection-state", "connecting");

        match find_and_connect(&transport).await {
            Ok((mut rx, connection)) => {
                info!("device connected, entering notification loop");
                let _ = app.emit("connection-state", "connected");

                // Request initial state
                let state_req = a3062::request_state().to_bytes();
                if let Err(e) = connection.write(&state_req).await {
                    warn!("failed to request state: {e}");
                }

                let mut stream = PacketStream::new();
                let mut state_poll = tokio::time::interval(Duration::from_secs(10));

                loop {
                    tokio::select! {
                        result = tokio::time::timeout(READ_TIMEOUT, rx.recv()) => {
                            match result {
                                Ok(Some(data)) => {
                                    stream.push(&data);
                                    while let Some(packet) = stream.next_packet() {
                                        handle_inbound_packet(&app, &packet);
                                    }
                                }
                                Ok(None) => {
                                    info!("read channel closed, device disconnected");
                                    break;
                                }
                                Err(_timeout) => {
                                    let status = connection.connection_status();
                                    if *status.borrow() == mabel_transport::ConnectionStatus::Disconnected {
                                        info!("device disconnected (timeout + status check)");
                                        break;
                                    }
                                }
                            }
                        }
                        Some(cmd) = cmd_rx.recv() => {
                            match cmd {
                                DeviceCommand::Disconnect => {
                                    info!("disconnect requested by user");
                                    break;
                                }
                                DeviceCommand::Connect => {
                                    // Already connected, ignore
                                }
                                _ => {
                                    debug!("executing command: {cmd:?}");
                                    let packet_bytes = build_command_packet(&cmd);
                                    if let Err(e) = connection.write(&packet_bytes).await {
                                        warn!("failed to send command: {e}");
                                        break;
                                    }
                                }
                            }
                        }
                        _ = state_poll.tick() => {
                            let state_req = a3062::request_state().to_bytes();
                            if let Err(e) = connection.write(&state_req).await {
                                warn!("failed to poll state: {e}");
                                break;
                            }
                        }
                    }
                }

                let _ = app.emit("connection-state", "disconnected");
            }
            Err(e) => {
                debug!("connect failed: {e}");
                let _ = app.emit("connection-state", "disconnected");
            }
        }

        tokio::time::sleep(RETRY_DELAY).await;
    }
}

async fn find_and_connect(
    transport: &WindowsRfcommTransport,
) -> Result<(mpsc::Receiver<Vec<u8>>, Box<dyn mabel_transport::RfcommConnection>), String> {
    let devices = transport.discover().await.map_err(|e| e.to_string())?;

    let soundcore = devices
        .iter()
        .find(|d| d.name.contains("soundcore") || d.name.contains("Space One"))
        .ok_or_else(|| "no Soundcore device found".to_string())?;

    info!("found: {} ({})", soundcore.name, soundcore.mac_address);

    let connection = transport
        .connect(&soundcore.mac_address)
        .await
        .map_err(|e| e.to_string())?;

    let rx = connection.read_channel();
    Ok((rx, connection))
}

fn handle_inbound_packet(app: &AppHandle, packet: &Packet) {
    if packet.command == [0x01, 0x01] {
        // Log raw bytes for debugging
        debug!("state update raw body ({} bytes): {:?}", packet.body.len(), &packet.body[..packet.body.len().min(30)]);
        if packet.body.len() > 30 {
            debug!("  ... bytes [30..]: {:?}", &packet.body[30..]);
        }

        match a3062::parse_state_update(&packet.body) {
            Ok(state) => {
                debug!("  parsed: battery={}/{}, fw={}, anc={:?}, dolby={}, ldac={}",
                    state.battery.level, state.battery.max_level,
                    state.firmware, state.sound_modes.ambient_sound_mode,
                    state.toggles.dolby_audio, state.toggles.ldac);
                let event = state_to_event(&state);

                // Battery alert check
                let settings = crate::settings::load();
                if settings.low_battery_alerts {
                    let pct = (state.battery.level as u16 * 100) / state.battery.max_level.max(1) as u16;
                    if pct <= settings.low_battery_threshold as u16 && pct > 0 {
                        let _ = app.emit("low-battery", pct as u8);
                    }
                }

                let _ = app.emit("device-state", &event);
                debug!("emitted device-state event");
            }
            Err(e) => warn!("failed to parse state update: {e}"),
        }
    } else {
        debug!("unhandled command: [{:#04x}, {:#04x}]", packet.command[0], packet.command[1]);
    }
}

fn state_to_event(state: &A3062State) -> DeviceStateEvent {
    DeviceStateEvent {
        battery: BatteryEvent { level: state.battery.level, max_level: state.battery.max_level, is_charging: state.battery.is_charging },
        firmware: state.firmware.clone(),
        serial_number: state.serial_number.clone(),
        sound_modes: SoundModesEvent {
            ambient_sound_mode: match state.sound_modes.ambient_sound_mode {
                a3062::AmbientSoundMode::NoiseCanceling => "noiseCanceling".into(),
                a3062::AmbientSoundMode::Transparency => "transparency".into(),
                a3062::AmbientSoundMode::Normal => "normal".into(),
            },
            noise_canceling_mode: match state.sound_modes.noise_canceling_mode {
                a3062::NoiseCancelingMode::Adaptive => "adaptive".into(),
                a3062::NoiseCancelingMode::Custom => "custom".into(),
            },
            adaptive_nc_level: state.sound_modes.adaptive_nc_level,
            custom_nc_level: state.sound_modes.custom_nc_level,
            custom_transparency: state.sound_modes.custom_transparency,
            wind_noise_reduction: state.sound_modes.wind_noise_reduction,
        },
        equalizer: EqualizerEvent {
            preset: state.equalizer.preset.map(|p| match p {
                a3062::EqPreset::SoundcoreSignature => "Soundcore Signature".into(),
                a3062::EqPreset::BassBoost => "Bass Boost".into(),
                a3062::EqPreset::Podcast => "Podcast".into(),
                a3062::EqPreset::Classical => "Classical".into(),
                a3062::EqPreset::BassReducer => "Bass Reducer".into(),
                a3062::EqPreset::TrebleBoost => "Treble Boost".into(),
            }),
            bands: state.equalizer.bands.to_vec(),
        },
        button_config: ButtonConfigEvent {
            double_press_action: state.button_config.double_press_action.map(|a| match a {
                a3062::ButtonAction::BassUp => "bassUp".into(),
            }),
        },
        ambient_sound_mode_cycle: ModeCycleEvent {
            noise_canceling: state.ambient_sound_mode_cycle.noise_canceling,
            transparency: state.ambient_sound_mode_cycle.transparency,
            normal: state.ambient_sound_mode_cycle.normal,
        },
        toggles: TogglesEvent {
            dolby_audio: state.toggles.dolby_audio,
            ldac: state.toggles.ldac,
            side_tone: state.toggles.side_tone,
            voice_prompt: state.toggles.voice_prompt,
            low_battery_prompt: state.toggles.low_battery_prompt,
        },
        auto_power_off: state.auto_power_off.minutes,
        limit_high_volume: LimitHighVolumeEvent {
            enabled: state.limit_high_volume.enabled,
            db_limit: state.limit_high_volume.db_limit,
        },
        dual_connections: DualConnectionsEvent {
            enabled: state.dual_connections,
            devices: vec![],
        },
    }
}

fn build_command_packet(cmd: &DeviceCommand) -> Vec<u8> {
    match cmd {
        DeviceCommand::SetSoundModes(modes) => a3062::set_sound_modes(modes).to_bytes(),
        DeviceCommand::SetEqualizer(eq) => a3062::set_equalizer(eq).to_bytes(),
        DeviceCommand::SetModeCycle(cycle) => a3062::set_mode_cycle(cycle).to_bytes(),
        DeviceCommand::SetButtonConfig(config) => a3062::set_button_config(config).to_bytes(),
        DeviceCommand::SetAutoPowerOff(apo) => a3062::set_auto_power_off(apo).to_bytes(),
        DeviceCommand::SetLdac(enabled) => a3062::set_ldac(*enabled).to_bytes(),
        DeviceCommand::SetDolby(enabled) => a3062::set_dolby(*enabled).to_bytes(),
        DeviceCommand::SetSidetone(enabled) => a3062::set_sidetone(*enabled).to_bytes(),
        DeviceCommand::Disconnect | DeviceCommand::Connect => vec![], // handled in select! loop
    }
}
