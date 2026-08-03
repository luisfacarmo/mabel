//! A3062 command builders — construct outbound packets to change device settings.
//!
//! Each builder returns a `Packet` ready to serialize with `Packet::to_bytes()`.

use crate::framing::Packet;
use super::state::*;

/// Command bytes for state request/response.
pub const CMD_STATE: [u8; 2] = [0x01, 0x01];
/// Command bytes for sound mode change.
pub const CMD_SOUND_MODES: [u8; 2] = [0x06, 0x01];
/// Command bytes for equalizer change.
pub const CMD_EQUALIZER: [u8; 2] = [0x02, 0x01];
/// Command bytes for ANC mode cycle configuration.
pub const CMD_MODE_CYCLE: [u8; 2] = [0x06, 0x04];
/// Command bytes for button configuration.
pub const CMD_BUTTON_CONFIG: [u8; 2] = [0x04, 0x02];
/// Command bytes for auto power off.
pub const CMD_AUTO_POWER_OFF: [u8; 2] = [0x01, 0x0C];
/// Command bytes for LDAC toggle.
pub const CMD_LDAC: [u8; 2] = [0x06, 0x09];
/// Command bytes for Dolby toggle.
pub const CMD_DOLBY: [u8; 2] = [0x06, 0x0A];
/// Command bytes for side tone toggle.
pub const CMD_SIDETONE: [u8; 2] = [0x06, 0x0D];

/// Request the full device state. Device responds with command [0x01, 0x01].
pub fn request_state() -> Packet {
    Packet::outbound(CMD_STATE, vec![])
}

/// Set sound modes (ANC mode, NC level, transparency level, wind noise).
pub fn set_sound_modes(modes: &SoundModes) -> Packet {
    let ambient_byte = match modes.ambient_sound_mode {
        AmbientSoundMode::NoiseCanceling => 0u8,
        AmbientSoundMode::Transparency => 1,
        AmbientSoundMode::Normal => 2,
    };

    let packed_nc = (modes.custom_nc_level << 4) | modes.adaptive_nc_level;

    let nc_mode_byte = match modes.noise_canceling_mode {
        NoiseCancelingMode::Custom => 0u8,
        NoiseCancelingMode::Adaptive => 1,
    };

    let body = vec![
        ambient_byte,
        packed_nc,
        1, // transparency_mode (always 1 for A3062)
        nc_mode_byte,
        u8::from(modes.wind_noise_reduction),
        modes.custom_transparency,
    ];

    Packet::outbound(CMD_SOUND_MODES, body)
}

/// Set equalizer bands (10 values, 0-180 range).
pub fn set_equalizer(eq: &EqualizerConfig) -> Packet {
    let mut body = Vec::with_capacity(12);
    body.push(0); // profile ID (0 = custom)
    body.push(0); // unknown
    body.extend_from_slice(&eq.bands);
    Packet::outbound(CMD_EQUALIZER, body)
}

/// Set ambient sound mode cycle (which modes the button cycles through).
pub fn set_mode_cycle(cycle: &AmbientSoundModeCycle) -> Packet {
    let mut byte: u8 = 0;
    if cycle.noise_canceling { byte |= 0x01; }
    if cycle.transparency { byte |= 0x02; }
    if cycle.normal { byte |= 0x04; }
    Packet::outbound(CMD_MODE_CYCLE, vec![byte])
}

/// Set button double-press action.
pub fn set_button_config(config: &ButtonConfig) -> Packet {
    let byte = match config.double_press_action {
        Some(ButtonAction::BassUp) => 7u8,
        None => 0x0F,
    };
    Packet::outbound(CMD_BUTTON_CONFIG, vec![byte])
}

/// Set auto power off (0 = disabled, otherwise duration code).
pub fn set_auto_power_off(apo: &AutoPowerOff) -> Packet {
    let (enabled, duration) = if apo.minutes == 0 {
        (0u8, 0u8)
    } else {
        (1u8, encode_auto_power_off(apo.minutes))
    };
    Packet::outbound(CMD_AUTO_POWER_OFF, vec![enabled, duration])
}

/// Toggle LDAC codec.
pub fn set_ldac(enabled: bool) -> Packet {
    Packet::outbound(CMD_LDAC, vec![u8::from(enabled)])
}

/// Toggle Dolby Audio.
pub fn set_dolby(enabled: bool) -> Packet {
    Packet::outbound(CMD_DOLBY, vec![u8::from(enabled)])
}

/// Toggle side tone.
pub fn set_sidetone(enabled: bool) -> Packet {
    Packet::outbound(CMD_SIDETONE, vec![u8::from(enabled)])
}

fn encode_auto_power_off(minutes: u16) -> u8 {
    match minutes {
        0 => 0,
        1..=5 => 1,
        6..=10 => 2,
        11..=15 => 3,
        16..=30 => 4,
        31..=60 => 5,
        _ => 6,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::framing::Packet;

    #[test]
    fn test_request_state_roundtrip() {
        let pkt = request_state();
        assert_eq!(pkt.command, CMD_STATE);
        assert!(pkt.body.is_empty());
        let parsed = Packet::parse(&pkt.to_bytes()).unwrap();
        assert_eq!(parsed, pkt);
    }

    #[test]
    fn test_set_sound_modes_transparency() {
        let modes = SoundModes {
            ambient_sound_mode: AmbientSoundMode::Transparency,
            noise_canceling_mode: NoiseCancelingMode::Adaptive,
            adaptive_nc_level: 3,
            custom_nc_level: 5,
            custom_transparency: 4,
            wind_noise_reduction: false,
        };
        let pkt = set_sound_modes(&modes);
        assert_eq!(pkt.body, &[1, 0x53, 1, 1, 0, 4]);
    }

    #[test]
    fn test_set_sound_modes_nc_custom() {
        let modes = SoundModes {
            ambient_sound_mode: AmbientSoundMode::NoiseCanceling,
            noise_canceling_mode: NoiseCancelingMode::Custom,
            adaptive_nc_level: 0,
            custom_nc_level: 4,
            custom_transparency: 3,
            wind_noise_reduction: true,
        };
        let pkt = set_sound_modes(&modes);
        assert_eq!(pkt.body, &[0, 0x40, 1, 0, 1, 3]);
    }

    #[test]
    fn test_set_equalizer_flat() {
        let eq = EqualizerConfig { preset: None, bands: [90; 10] };
        let pkt = set_equalizer(&eq);
        assert_eq!(pkt.body.len(), 12);
        assert_eq!(&pkt.body[2..], &[90u8; 10]);
    }

    #[test]
    fn test_set_mode_cycle() {
        let all = AmbientSoundModeCycle { noise_canceling: true, transparency: true, normal: true };
        assert_eq!(set_mode_cycle(&all).body, &[0x07]);

        let nc_only = AmbientSoundModeCycle { noise_canceling: true, transparency: false, normal: false };
        assert_eq!(set_mode_cycle(&nc_only).body, &[0x01]);
    }

    #[test]
    fn test_set_button_config() {
        let bass = ButtonConfig { double_press_action: Some(ButtonAction::BassUp) };
        assert_eq!(set_button_config(&bass).body, &[7]);

        let none = ButtonConfig { double_press_action: None };
        assert_eq!(set_button_config(&none).body, &[0x0F]);
    }

    #[test]
    fn test_set_auto_power_off() {
        assert_eq!(set_auto_power_off(&AutoPowerOff { minutes: 0 }).body, &[0, 0]);
        assert_eq!(set_auto_power_off(&AutoPowerOff { minutes: 60 }).body, &[1, 5]);
    }

    #[test]
    fn test_toggles() {
        assert_eq!(set_ldac(true).body, &[1]);
        assert_eq!(set_ldac(false).body, &[0]);
        assert_eq!(set_dolby(true).body, &[1]);
        assert_eq!(set_sidetone(false).body, &[0]);
    }

    #[test]
    fn test_all_commands_roundtrip() {
        let packets = vec![
            request_state(),
            set_sound_modes(&SoundModes {
                ambient_sound_mode: AmbientSoundMode::Normal,
                noise_canceling_mode: NoiseCancelingMode::Adaptive,
                adaptive_nc_level: 3, custom_nc_level: 3, custom_transparency: 3,
                wind_noise_reduction: false,
            }),
            set_equalizer(&EqualizerConfig { preset: None, bands: [90; 10] }),
            set_mode_cycle(&AmbientSoundModeCycle { noise_canceling: true, transparency: true, normal: false }),
            set_button_config(&ButtonConfig { double_press_action: Some(ButtonAction::BassUp) }),
            set_auto_power_off(&AutoPowerOff { minutes: 30 }),
            set_ldac(true),
            set_dolby(false),
            set_sidetone(true),
        ];
        for pkt in packets {
            let parsed = Packet::parse(&pkt.to_bytes()).unwrap();
            assert_eq!(parsed, pkt);
        }
    }
}
