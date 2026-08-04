//! Tauri commands — bridge between frontend invokes and the device loop.

use mabel_protocol::models::a3062;
use tauri::State;

use crate::device::{CommandSender, DeviceCommand};

#[tauri::command]
pub fn set_sound_mode(
    mode: String,
    nc_mode: Option<String>,
    level: Option<u8>,
    wind_noise: Option<bool>,
    cmd_tx: State<CommandSender>,
) -> Result<(), String> {
    let ambient = match mode.as_str() {
        "noiseCanceling" => a3062::AmbientSoundMode::NoiseCanceling,
        "transparency" => a3062::AmbientSoundMode::Transparency,
        "normal" => a3062::AmbientSoundMode::Normal,
        other => return Err(format!("unknown mode: {other}")),
    };

    let nc = match nc_mode.as_deref().unwrap_or("adaptive") {
        "adaptive" => a3062::NoiseCancelingMode::Adaptive,
        "custom" => a3062::NoiseCancelingMode::Custom,
        _ => a3062::NoiseCancelingMode::Adaptive,
    };

    let modes = a3062::SoundModes {
        ambient_sound_mode: ambient,
        noise_canceling_mode: nc,
        adaptive_nc_level: level.unwrap_or(3),
        custom_nc_level: level.unwrap_or(3),
        custom_transparency: level.unwrap_or(3),
        wind_noise_reduction: wind_noise.unwrap_or(false),
    };

    cmd_tx
        .send(DeviceCommand::SetSoundModes(modes))
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn set_equalizer(preset: Option<String>, bands: Vec<u8>, cmd_tx: State<CommandSender>) -> Result<(), String> {
    let eq_preset = preset.and_then(|p| match p.as_str() {
        "Soundcore Signature" => Some(a3062::EqPreset::SoundcoreSignature),
        "Bass Boost" => Some(a3062::EqPreset::BassBoost),
        "Podcast" => Some(a3062::EqPreset::Podcast),
        "Classical" => Some(a3062::EqPreset::Classical),
        "Bass Reducer" => Some(a3062::EqPreset::BassReducer),
        "Treble Boost" => Some(a3062::EqPreset::TrebleBoost),
        _ => None,
    });

    let mut band_array = [90u8; 10];
    for (i, &v) in bands.iter().take(10).enumerate() {
        band_array[i] = v;
    }

    cmd_tx
        .send(DeviceCommand::SetEqualizer(a3062::EqualizerConfig {
            preset: eq_preset,
            bands: band_array,
        }))
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn set_ldac(enabled: bool, cmd_tx: State<CommandSender>) -> Result<(), String> {
    cmd_tx.send(DeviceCommand::SetLdac(enabled)).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn set_dolby(enabled: bool, cmd_tx: State<CommandSender>) -> Result<(), String> {
    cmd_tx.send(DeviceCommand::SetDolby(enabled)).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn set_sidetone(enabled: bool, cmd_tx: State<CommandSender>) -> Result<(), String> {
    cmd_tx.send(DeviceCommand::SetSidetone(enabled)).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn set_auto_power_off(minutes: u16, cmd_tx: State<CommandSender>) -> Result<(), String> {
    cmd_tx
        .send(DeviceCommand::SetAutoPowerOff(a3062::AutoPowerOff { minutes }))
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn set_mode_cycle(nc: bool, transparency: bool, normal: bool, cmd_tx: State<CommandSender>) -> Result<(), String> {
    cmd_tx
        .send(DeviceCommand::SetModeCycle(a3062::AmbientSoundModeCycle {
            noise_canceling: nc,
            transparency,
            normal,
        }))
        .map_err(|e| e.to_string())
}


// --- Settings commands ---

#[tauri::command]
pub fn get_settings() -> crate::settings::Settings {
    crate::settings::load()
}

#[tauri::command]
pub fn save_settings(settings: crate::settings::Settings) -> Result<(), String> {
    crate::settings::save(&settings)
}


#[tauri::command]
pub fn disconnect_device(cmd_tx: State<CommandSender>) -> Result<(), String> {
    cmd_tx.send(DeviceCommand::Disconnect).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn connect_device(cmd_tx: State<CommandSender>) -> Result<(), String> {
    cmd_tx.send(DeviceCommand::Connect).map_err(|e| e.to_string())
}
