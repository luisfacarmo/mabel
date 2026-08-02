import type { DeviceState } from "./types";

// Based on the OpenSCQ30 test vector from issue #194.
// Represents a realistic A3062 state for UI development.
export const MOCK_DEVICE_STATE: DeviceState = {
  battery: {
    level: 5,
    maxLevel: 10,
  },
  firmware: "03.37",
  serialNumber: "3062DB212C13E97C",
  soundModes: {
    ambientSoundMode: "transparency",
    noiseCancelingMode: "adaptive",
    adaptiveNcLevel: 5,
    customNcLevel: 3,
    customTransparency: 5,
    windNoiseReduction: false,
  },
  ambientSoundModeCycle: {
    noiseCanceling: true,
    transparency: true,
    normal: false,
  },
  equalizer: {
    preset: "Podcast",
    bands: [90, 140, 160, 160, 150, 140, 120, 100, 120, 90],
  },
  buttonConfig: {
    doublePressAction: "BassUp",
  },
  toggles: {
    dolbyAudio: true,
    ldac: false,
    sideTone: false,
    voicePrompt: true,
    lowBatteryPrompt: true,
  },
  autoPowerOff: 60,
  limitHighVolume: {
    enabled: false,
    dbLimit: 90,
  },
  dualConnections: {
    enabled: true,
    devices: [
      { name: "DESKTOP-PC", connected: true },
      { name: "iPhone 15", connected: false },
    ],
  },
};

// EQ presets — band values for each named preset
export const EQ_PRESETS: Record<string, number[]> = {
  "Soundcore Signature": [90, 90, 90, 90, 90, 90, 90, 90, 90, 90],
  "Bass Boost": [150, 140, 120, 100, 90, 90, 90, 90, 90, 90],
  Podcast: [90, 140, 160, 160, 150, 140, 120, 100, 120, 90],
  Classical: [90, 90, 90, 90, 90, 90, 120, 130, 140, 150],
  "Bass Reducer": [60, 70, 80, 90, 90, 90, 90, 90, 90, 90],
  "Treble Boost": [90, 90, 90, 90, 90, 100, 120, 140, 150, 160],
};
