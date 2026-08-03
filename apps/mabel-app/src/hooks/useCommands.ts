import { useCallback } from "react";
import type { AmbientSoundMode, NoiseCancelingMode } from "../lib/types";
import * as tauri from "../lib/tauri";

/**
 * Hook: command dispatch (setters that invoke Tauri backend).
 * In Tauri: calls real invoke() commands.
 * In browser: logs to console (UI development).
 */
export function useCommands() {
  const setSoundMode = useCallback(
    (mode: AmbientSoundMode, ncMode?: NoiseCancelingMode, level?: number, windNoise?: boolean) => {
      if (tauri.isTauri) {
        tauri.setSoundMode(mode, ncMode, level, windNoise).catch(console.error);
      } else {
        console.log("[mock] setSoundMode:", mode, ncMode, level, windNoise);
      }
    },
    []
  );

  const setEqualizer = useCallback((preset: string | null, bands: number[]) => {
    if (tauri.isTauri) {
      tauri.setEqualizer(preset, bands).catch(console.error);
    } else {
      console.log("[mock] setEqualizer:", { preset, bands });
    }
  }, []);

  const setLdac = useCallback((enabled: boolean) => {
    if (tauri.isTauri) {
      tauri.setLdac(enabled).catch(console.error);
    } else {
      console.log("[mock] setLdac:", enabled);
    }
  }, []);

  const setDolby = useCallback((enabled: boolean) => {
    if (tauri.isTauri) {
      tauri.setDolby(enabled).catch(console.error);
    } else {
      console.log("[mock] setDolby:", enabled);
    }
  }, []);

  const setSidetone = useCallback((enabled: boolean) => {
    if (tauri.isTauri) {
      tauri.setSidetone(enabled).catch(console.error);
    } else {
      console.log("[mock] setSidetone:", enabled);
    }
  }, []);

  const setAutoPowerOff = useCallback((minutes: number) => {
    if (tauri.isTauri) {
      tauri.setAutoPowerOff(minutes).catch(console.error);
    } else {
      console.log("[mock] setAutoPowerOff:", minutes);
    }
  }, []);

  const setModeCycle = useCallback((nc: boolean, transparency: boolean, normal: boolean) => {
    if (tauri.isTauri) {
      tauri.setModeCycle(nc, transparency, normal).catch(console.error);
    } else {
      console.log("[mock] setModeCycle:", { nc, transparency, normal });
    }
  }, []);

  return {
    setSoundMode,
    setEqualizer,
    setLdac,
    setDolby,
    setSidetone,
    setAutoPowerOff,
    setModeCycle,
  };
}
