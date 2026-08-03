import { useState, useEffect } from "react";
import type { DeviceState } from "../lib/types";
import { MOCK_DEVICE_STATE } from "../lib/mock-data";
import { isTauri, onDeviceState } from "../lib/tauri";

/**
 * Hook: readonly device state (populated from backend events).
 * In Tauri: listens to "device-state" events emitted by the device loop.
 * In browser: returns static mock data for UI development.
 */
export function useDeviceState() {
  const [state, setState] = useState<DeviceState | null>(
    isTauri ? null : MOCK_DEVICE_STATE
  );

  useEffect(() => {
    if (!isTauri) return;

    let unlisten: (() => void) | undefined;

    onDeviceState((deviceState) => {
      setState(deviceState);
    }).then((fn) => {
      unlisten = fn;
    });

    return () => {
      unlisten?.();
    };
  }, []);

  return { state };
}
