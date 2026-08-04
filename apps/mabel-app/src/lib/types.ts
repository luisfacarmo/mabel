// A3062 device state — mirrors the Rust struct shape exactly.
// When real IPC replaces mocks, this interface stays unchanged.

export type AmbientSoundMode = "noiseCanceling" | "transparency" | "normal";
export type NoiseCancelingMode = "adaptive" | "custom";
export type ConnectionStatus = "connected" | "disconnected" | "reconnecting";

export interface DeviceState {
  battery: {
    level: number; // 1-10
    maxLevel: number; // always 10 for A3062
    isCharging: boolean;
  };
  firmware: string;
  serialNumber: string;
  soundModes: {
    ambientSoundMode: AmbientSoundMode;
    noiseCancelingMode: NoiseCancelingMode;
    adaptiveNcLevel: number; // 1-5
    customNcLevel: number; // 1-5
    customTransparency: number; // 1-5
    windNoiseReduction: boolean;
  };
  ambientSoundModeCycle: {
    noiseCanceling: boolean;
    transparency: boolean;
    normal: boolean;
  };
  equalizer: {
    preset: string | null; // null = custom
    bands: number[]; // 10 values, 0-180 range
  };
  buttonConfig: {
    doublePressAction: string | null;
  };
  toggles: {
    dolbyAudio: boolean;
    ldac: boolean;
    sideTone: boolean;
    voicePrompt: boolean;
    lowBatteryPrompt: boolean;
  };
  autoPowerOff: number; // minutes, 0 = disabled
  limitHighVolume: {
    enabled: boolean;
    dbLimit: number;
  };
  dualConnections: {
    enabled: boolean;
    devices: Array<{ name: string; connected: boolean }>;
  };
}
