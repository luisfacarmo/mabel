//! A3062 state parser — parses the body of a command [0x01, 0x01] response into A3062State.
//!
//! Byte layout verified against OpenSCQ30 source code (structures.rs, state_update.rs).
//! Field order: Battery, Firmware, Serial, EQ, 2 unknown, HearID, 2 unknown,
//! ButtonConfig, ModeCycle, SoundModes, 1 unknown, LowBattery, Dolby, LDAC,
//! DualConn, AutoPowerOff, LimitHighVolume, SideTone, VoicePrompt.

use crate::error::{ProtocolError, Result};
use super::state::*;

/// Known EQ preset band values for matching.
const PRESET_BANDS: &[(&str, EqPreset, [u8; 10])] = &[
    ("Soundcore Signature", EqPreset::SoundcoreSignature, [90, 90, 90, 90, 90, 90, 90, 90, 90, 90]),
    ("Bass Boost", EqPreset::BassBoost, [150, 140, 120, 90, 90, 90, 90, 90, 90, 90]),
    ("Podcast", EqPreset::Podcast, [90, 140, 160, 160, 150, 140, 120, 100, 120, 0]),
    ("Classical", EqPreset::Classical, [120, 90, 90, 90, 90, 90, 90, 100, 120, 140]),
    ("Bass Reducer", EqPreset::BassReducer, [60, 70, 80, 90, 90, 90, 90, 90, 90, 90]),
    ("Treble Boost", EqPreset::TrebleBoost, [90, 90, 90, 90, 90, 100, 120, 140, 150, 160]),
];

/// Parse the body of a state update packet (command [0x01, 0x01]) into A3062State.
///
/// The body bytes start AFTER the packet framing (direction + command + length already stripped).
pub fn parse_state_update(body: &[u8]) -> Result<A3062State> {
    if body.len() < 86 {
        return Err(ProtocolError::ParseError(format!(
            "state update body too short: {} bytes (need at least 86)",
            body.len()
        )));
    }

    let mut pos = 0;

    // --- SingleBattery (2 bytes) ---
    let battery_level = body[pos];
    pos += 1;
    let _battery_charging = body[pos];
    pos += 1;

    // --- FirmwareVersion (5 bytes ASCII) ---
    let firmware = parse_ascii(body, pos, 5)?;
    pos += 5;

    // --- SerialNumber (16 bytes ASCII) ---
    let serial_number = parse_ascii(body, pos, 16)?;
    pos += 16;

    // --- EqualizerConfiguration: profile_id (1) + unknown (1) + 10 bands ---
    let _eq_profile_id = body[pos];
    pos += 1;
    let _eq_unknown = body[pos];
    pos += 1;
    let mut bands = [0u8; 10];
    bands.copy_from_slice(&body[pos..pos + 10]);
    pos += 10;

    // --- 2 unknown bytes ---
    pos += 2;

    // --- CustomHearId (27 bytes for A3062 with music_genre_at_end) ---
    pos += 27;

    // --- 2 unknown bytes ---
    pos += 2;

    // --- ButtonConfiguration (1 byte): BassUp = 7 ---
    let button_byte = body[pos];
    pos += 1;

    // --- AmbientSoundModeCycle (1 byte bitmask): bit0=NC, bit1=Transp, bit2=Normal ---
    let cycle_byte = body[pos];
    pos += 1;

    // --- SoundModes (6 bytes) ---
    let ambient_mode_byte = body[pos];
    pos += 1;
    let packed_nc = body[pos]; // (custom_nc << 4) | adaptive_nc
    pos += 1;
    let _transparency_mode = body[pos]; // always 1
    pos += 1;
    let nc_mode_byte = body[pos]; // Custom=0, Adaptive=1
    pos += 1;
    let wind_noise_byte = body[pos];
    pos += 1;
    let custom_transparency = body[pos];
    pos += 1;

    // --- 1 unknown byte ---
    pos += 1;

    // --- LowBatteryPrompt (1 byte bool) ---
    let low_battery_prompt = body[pos] != 0;
    pos += 1;

    // --- DolbyAudio (1 byte bool) ---
    let dolby_audio = body[pos] != 0;
    pos += 1;

    // --- Ldac (1 byte bool) ---
    let ldac = body[pos] != 0;
    pos += 1;

    // --- DualConnections (1 byte bool) ---
    let dual_connections = body[pos] != 0;
    pos += 1;

    // --- AutoPowerOff (2 bytes: is_enabled + duration) ---
    let auto_power_off_enabled = body[pos] != 0;
    pos += 1;
    let auto_power_off_duration = body[pos];
    pos += 1;

    // --- LimitHighVolume (3 bytes: enabled + db_limit + refresh_rate) ---
    let limit_vol_enabled = body[pos] != 0;
    pos += 1;
    let limit_vol_db = body[pos];
    pos += 1;
    let _limit_vol_refresh = body[pos];
    pos += 1;

    // --- SideTone (1 byte bool) ---
    let side_tone = if pos < body.len() { body[pos] != 0 } else { false };
    pos += 1;

    // --- AmbientSoundModeVoicePrompt (1 byte bool) ---
    let voice_prompt = if pos < body.len() { body[pos] != 0 } else { true };
    let _pos = pos + 1;

    // --- Build state ---

    let ambient_sound_mode = match ambient_mode_byte {
        0 => AmbientSoundMode::NoiseCanceling,
        1 => AmbientSoundMode::Transparency,
        2 => AmbientSoundMode::Normal,
        _ => AmbientSoundMode::Normal,
    };

    let noise_canceling_mode = match nc_mode_byte {
        0 => NoiseCancelingMode::Custom,
        1 => NoiseCancelingMode::Adaptive,
        _ => NoiseCancelingMode::Adaptive,
    };

    let adaptive_nc_level = packed_nc & 0x0F;
    let custom_nc_level = packed_nc >> 4;

    let preset = detect_preset(&bands);

    let double_press_action = match button_byte {
        7 => Some(ButtonAction::BassUp),
        _ => None,
    };

    let auto_power_off_minutes = if auto_power_off_enabled {
        decode_auto_power_off_duration(auto_power_off_duration)
    } else {
        0
    };

    Ok(A3062State {
        battery: Battery {
            level: battery_level.min(10),
            max_level: 10,
        },
        firmware,
        serial_number,
        sound_modes: SoundModes {
            ambient_sound_mode,
            noise_canceling_mode,
            adaptive_nc_level,
            custom_nc_level,
            custom_transparency,
            wind_noise_reduction: wind_noise_byte != 0,
        },
        equalizer: EqualizerConfig { preset, bands },
        button_config: ButtonConfig {
            double_press_action,
        },
        ambient_sound_mode_cycle: AmbientSoundModeCycle {
            noise_canceling: cycle_byte & 0x01 != 0,
            transparency: cycle_byte & 0x02 != 0,
            normal: cycle_byte & 0x04 != 0,
        },
        toggles: DeviceToggles {
            dolby_audio,
            ldac,
            side_tone,
            voice_prompt,
            low_battery_prompt,
        },
        auto_power_off: AutoPowerOff {
            minutes: auto_power_off_minutes,
        },
        limit_high_volume: LimitHighVolume {
            enabled: limit_vol_enabled,
            db_limit: limit_vol_db,
        },
        dual_connections,
    })
}

fn parse_ascii(body: &[u8], offset: usize, len: usize) -> Result<String> {
    if offset + len > body.len() {
        return Err(ProtocolError::ParseError(format!(
            "not enough bytes for ASCII at offset {}: need {}, have {}",
            offset, len, body.len() - offset
        )));
    }
    Ok(String::from_utf8_lossy(&body[offset..offset + len]).into_owned())
}

fn detect_preset(bands: &[u8; 10]) -> Option<EqPreset> {
    for &(_, preset, ref preset_bands) in PRESET_BANDS {
        if bands == preset_bands {
            return Some(preset);
        }
    }
    None
}

fn decode_auto_power_off_duration(byte: u8) -> u16 {
    match byte {
        0 => 0,
        1 => 5,
        2 => 10,
        3 => 15,
        4 => 30,
        5 => 60,
        6 => 120,
        _ => byte as u16 * 10,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test vector from OpenSCQ30 (A3062 state update body, 94 bytes).
    const TEST_VECTOR: &[u8] = &[
        4, 255, 48, 51, 46, 51, 55, 51, 48, 54, 50, 68, 66, 50, 49, 50, 67, 49, 51,
        69, 57, 55, 67, 5, 0, 90, 140, 160, 160, 150, 140, 120, 100, 120, 0, 30, 255,
        0, 255, 255, 255, 255, 255, 255, 255, 255, 255, 0, 0, 0, 0, 0, 0, 255, 255,
        255, 255, 255, 255, 255, 255, 255, 0, 0, 4, 4, 7, 3, 1, 80, 1, 1, 0, 5, 49,
        1, 1, 0, 1, 1, 1, 0, 90, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0,
    ];

    #[test]
    fn test_parse_battery() {
        let state = parse_state_update(TEST_VECTOR).unwrap();
        assert_eq!(state.battery.level, 4);
        assert_eq!(state.battery.max_level, 10);
    }

    #[test]
    fn test_parse_firmware() {
        let state = parse_state_update(TEST_VECTOR).unwrap();
        assert_eq!(state.firmware, "03.37");
    }

    #[test]
    fn test_parse_serial_number() {
        let state = parse_state_update(TEST_VECTOR).unwrap();
        assert_eq!(state.serial_number, "3062DB212C13E97C");
    }

    #[test]
    fn test_parse_eq_bands() {
        let state = parse_state_update(TEST_VECTOR).unwrap();
        assert_eq!(state.equalizer.bands, [90, 140, 160, 160, 150, 140, 120, 100, 120, 0]);
        assert_eq!(state.equalizer.preset, Some(EqPreset::Podcast));
    }

    #[test]
    fn test_parse_sound_modes() {
        let state = parse_state_update(TEST_VECTOR).unwrap();
        assert_eq!(state.sound_modes.ambient_sound_mode, AmbientSoundMode::Transparency);
        assert_eq!(state.sound_modes.noise_canceling_mode, NoiseCancelingMode::Adaptive);
        assert_eq!(state.sound_modes.custom_nc_level, 5);
        assert_eq!(state.sound_modes.custom_transparency, 5);
    }

    #[test]
    fn test_parse_button_config() {
        let state = parse_state_update(TEST_VECTOR).unwrap();
        assert_eq!(state.button_config.double_press_action, Some(ButtonAction::BassUp));
    }

    #[test]
    fn test_parse_mode_cycle() {
        let state = parse_state_update(TEST_VECTOR).unwrap();
        assert!(state.ambient_sound_mode_cycle.noise_canceling);
        assert!(state.ambient_sound_mode_cycle.transparency);
        assert!(!state.ambient_sound_mode_cycle.normal);
    }

    #[test]
    fn test_parse_toggles() {
        let state = parse_state_update(TEST_VECTOR).unwrap();
        assert!(state.toggles.low_battery_prompt);
        assert!(state.toggles.dolby_audio);
        assert!(!state.toggles.ldac);
    }

    #[test]
    fn test_parse_dual_connections() {
        let state = parse_state_update(TEST_VECTOR).unwrap();
        assert!(state.dual_connections);
    }

    #[test]
    fn test_parse_auto_power_off() {
        let state = parse_state_update(TEST_VECTOR).unwrap();
        assert_eq!(state.auto_power_off.minutes, 5);
    }

    #[test]
    fn test_too_short_body_fails() {
        let short = &[0u8; 10];
        assert!(parse_state_update(short).is_err());
    }
}
