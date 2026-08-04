//! A3062 device state — all typed structs for the Space One Pro.

use serde::{Deserialize, Serialize};

/// Full device state for the Soundcore Space One Pro (A3062).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct A3062State {
    pub battery: Battery,
    pub firmware: String,
    pub serial_number: String,
    pub sound_modes: SoundModes,
    pub equalizer: EqualizerConfig,
    pub button_config: ButtonConfig,
    pub ambient_sound_mode_cycle: AmbientSoundModeCycle,
    pub toggles: DeviceToggles,
    pub auto_power_off: AutoPowerOff,
    pub limit_high_volume: LimitHighVolume,
    pub dual_connections: bool,
}

/// Battery level (0-10 scale, maxLevel always 10 for A3062).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Battery {
    /// Current level (1-10, with +1 offset applied from raw wire value).
    pub level: u8,
    /// Maximum level (always 10 for A3062).
    pub max_level: u8,
    /// Whether the device is currently charging.
    pub is_charging: bool,
}

/// Active sound/ANC modes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SoundModes {
    pub ambient_sound_mode: AmbientSoundMode,
    pub noise_canceling_mode: NoiseCancelingMode,
    /// Adaptive NC intensity (1-5).
    pub adaptive_nc_level: u8,
    /// Custom NC intensity (1-5).
    pub custom_nc_level: u8,
    /// Custom transparency intensity (1-5).
    pub custom_transparency: u8,
    pub wind_noise_reduction: bool,
}

/// Top-level ambient sound mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AmbientSoundMode {
    NoiseCanceling,
    Transparency,
    Normal,
}

/// Sub-mode within Noise Canceling.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NoiseCancelingMode {
    Adaptive,
    Custom,
}

/// 10-band equalizer configuration.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EqualizerConfig {
    /// Named preset, or None if custom.
    pub preset: Option<EqPreset>,
    /// 10 band values (0-180 range, 90 = flat).
    pub bands: [u8; 10],
}

/// Known EQ presets for A3062.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EqPreset {
    SoundcoreSignature,
    BassBoost,
    Podcast,
    Classical,
    BassReducer,
    TrebleBoost,
}

/// Button/gesture configuration.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ButtonConfig {
    pub double_press_action: Option<ButtonAction>,
}

/// Possible double-press actions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ButtonAction {
    BassUp,
}

/// Which modes the physical button cycles through.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AmbientSoundModeCycle {
    pub noise_canceling: bool,
    pub transparency: bool,
    pub normal: bool,
}

/// On/off toggles for various features.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeviceToggles {
    pub dolby_audio: bool,
    pub ldac: bool,
    pub side_tone: bool,
    pub voice_prompt: bool,
    pub low_battery_prompt: bool,
}

/// Auto power off configuration.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AutoPowerOff {
    /// Minutes until auto power off. 0 = disabled.
    pub minutes: u16,
}

/// Volume limiter configuration.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LimitHighVolume {
    pub enabled: bool,
    /// dB limit when enabled.
    pub db_limit: u8,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_state_serializes_to_json() {
        let state = A3062State {
            battery: Battery { level: 7, max_level: 10, is_charging: false },
            firmware: "03.37".into(),
            serial_number: "3062DB212C13E97C".into(),
            sound_modes: SoundModes {
                ambient_sound_mode: AmbientSoundMode::NoiseCanceling,
                noise_canceling_mode: NoiseCancelingMode::Adaptive,
                adaptive_nc_level: 3,
                custom_nc_level: 3,
                custom_transparency: 3,
                wind_noise_reduction: false,
            },
            equalizer: EqualizerConfig {
                preset: Some(EqPreset::Podcast),
                bands: [90, 140, 160, 160, 150, 140, 120, 100, 120, 0],
            },
            button_config: ButtonConfig {
                double_press_action: Some(ButtonAction::BassUp),
            },
            ambient_sound_mode_cycle: AmbientSoundModeCycle {
                noise_canceling: true,
                transparency: true,
                normal: true,
            },
            toggles: DeviceToggles {
                dolby_audio: true,
                ldac: false,
                side_tone: false,
                voice_prompt: true,
                low_battery_prompt: true,
            },
            auto_power_off: AutoPowerOff { minutes: 60 },
            limit_high_volume: LimitHighVolume { enabled: false, db_limit: 90 },
            dual_connections: true,
        };

        let json = serde_json::to_string_pretty(&state).unwrap();
        assert!(json.contains("03.37"));
        assert!(json.contains("3062DB212C13E97C"));
        assert!(json.contains("NoiseCanceling"));

        // Round-trip
        let deserialized: A3062State = serde_json::from_str(&json).unwrap();
        assert_eq!(state, deserialized);
    }

    #[test]
    fn test_eq_preset_values() {
        let config = EqualizerConfig {
            preset: None,
            bands: [90; 10], // flat
        };
        assert!(config.preset.is_none());
        assert_eq!(config.bands, [90; 10]);
    }
}
