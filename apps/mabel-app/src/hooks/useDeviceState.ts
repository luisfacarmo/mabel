import { useState } from "react";
import type { DeviceState } from "../lib/types";
import { MOCK_DEVICE_STATE } from "../lib/mock-data";

// Hook: readonly device state (populated from events).
// Mock implementation — swap for Tauri event listener in Phase E.

export function useDeviceState() {
  const [state] = useState<DeviceState | null>(MOCK_DEVICE_STATE);

  return { state };
}
