import { useCallback } from "react";
import type { AmbientSoundMode, NoiseCancelingMode } from "../lib/types";

// Hook: command dispatch (setters that invoke backend).
// Mock implementation — swap for Tauri invoke() in Phase E.
// Each function logs the command for dev visibility.

export function useCommands() {
  const setSoundMode = useCallback((mode: AmbientSoundMode) => {
    console.log("[mock] setSoundMode:", mode);
  }, []);

  const setNoiseCancelingMode = useCallback((mode: NoiseCancelingMode) => {
    console.log("[mock] setNoiseCancelingMode:", mode);
  }, []);

  const setAdaptiveNcLevel = useCallback((level: number) => {
    console.log("[mock] setAdaptiveNcLevel:", level);
  }, []);

  const setCustomNcLevel = useCallback((level: number) => {
    console.log("[mock] setCustomNcLevel:", level);
  }, []);

  const setCustomTransparency = useCallback((level: number) => {
    console.log("[mock] setCustomTransparency:", level);
  }, []);

  const setWindNoiseReduction = useCallback((enabled: boolean) => {
    console.log("[mock] setWindNoiseReduction:", enabled);
  }, []);

  const setEqualizer = useCallback((preset: string | null, bands: number[]) => {
    console.log("[mock] setEqualizer:", { preset, bands });
  }, []);

  const setToggle = useCallback((name: string, value: boolean) => {
    console.log("[mock] setToggle:", name, value);
  }, []);

  const setAutoPowerOff = useCallback((minutes: number) => {
    console.log("[mock] setAutoPowerOff:", minutes);
  }, []);

  const setButtonConfig = useCallback((action: string | null) => {
    console.log("[mock] setButtonConfig:", action);
  }, []);

  return {
    setSoundMode,
    setNoiseCancelingMode,
    setAdaptiveNcLevel,
    setCustomNcLevel,
    setCustomTransparency,
    setWindNoiseReduction,
    setEqualizer,
    setToggle,
    setAutoPowerOff,
    setButtonConfig,
  };
}
