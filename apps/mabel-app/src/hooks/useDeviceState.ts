import { useState, useEffect, useRef } from "react";
import type { DeviceState } from "../lib/types";
import { MOCK_DEVICE_STATE } from "../lib/mock-data";
import { isTauri, onDeviceState } from "../lib/tauri";

/**
 * Hook: readonly device state (populated from backend events).
 * In Tauri: listens to "device-state" events emitted by the device loop.
 * The device loop polls state every 10s, so the frontend always catches up.
 * In browser: returns static mock data for UI development.
 */
export function useDeviceState() {
  const [state, setState] = useState<DeviceState | null>(
    isTauri ? null : MOCK_DEVICE_STATE
  );
  const listenerReady = useRef(false);

  useEffect(() => {
    if (!isTauri) return;

    let unlisten: (() => void) | undefined;

    onDeviceState((deviceState) => {
      setState(deviceState);
    }).then((fn) => {
      unlisten = fn;
      listenerReady.current = true;
    });

    return () => {
      unlisten?.();
    };
  }, []);

  return { state };
}
