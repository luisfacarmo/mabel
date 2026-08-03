/**
 * Tauri IPC bridge — typed wrappers for invoke + event listeners.
 * Centralizes all communication with the Rust backend.
 */

import { invoke } from "@tauri-apps/api/core";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import type { DeviceState } from "./types";

// --- Check if running inside Tauri ---
export const isTauri =
  typeof window !== "undefined" && "__TAURI_INTERNALS__" in window;

// --- Connection state events ---
export type ConnectionState = "connecting" | "connected" | "disconnected";

export function onConnectionState(
  cb: (state: ConnectionState) => void
): Promise<UnlistenFn> {
  return listen<ConnectionState>("connection-state", (e) => cb(e.payload));
}

// --- Device state events ---
export function onDeviceState(
  cb: (state: DeviceState) => void
): Promise<UnlistenFn> {
  return listen<DeviceState>("device-state", (e) => cb(e.payload));
}

// --- Commands (invoke Tauri backend) ---

export function setSoundMode(
  mode: string,
  ncMode?: string,
  level?: number,
  windNoise?: boolean
): Promise<void> {
  return invoke("set_sound_mode", {
    mode,
    ncMode: ncMode ?? null,
    level: level ?? null,
    windNoise: windNoise ?? null,
  });
}

export function setEqualizer(
  preset: string | null,
  bands: number[]
): Promise<void> {
  return invoke("set_equalizer", { preset, bands });
}

export function setLdac(enabled: boolean): Promise<void> {
  return invoke("set_ldac", { enabled });
}

export function setDolby(enabled: boolean): Promise<void> {
  return invoke("set_dolby", { enabled });
}

export function setSidetone(enabled: boolean): Promise<void> {
  return invoke("set_sidetone", { enabled });
}

export function setAutoPowerOff(minutes: number): Promise<void> {
  return invoke("set_auto_power_off", { minutes });
}

export function setModeCycle(
  nc: boolean,
  transparency: boolean,
  normal: boolean
): Promise<void> {
  return invoke("set_mode_cycle", { nc, transparency, normal });
}
