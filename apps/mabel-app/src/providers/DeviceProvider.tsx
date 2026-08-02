import { createContext, useContext, type ReactNode } from "react";
import { useConnection } from "../hooks/useConnection";
import { useDeviceState } from "../hooks/useDeviceState";
import { useCommands } from "../hooks/useCommands";
import type { ConnectionStatus, DeviceState } from "../lib/types";

interface DeviceContextValue {
  connection: ReturnType<typeof useConnection>;
  state: DeviceState | null;
  commands: ReturnType<typeof useCommands>;
}

const DeviceContext = createContext<DeviceContextValue | null>(null);

export function DeviceProvider({ children }: { children: ReactNode }) {
  const connection = useConnection();
  const { state } = useDeviceState();
  const commands = useCommands();

  return (
    <DeviceContext.Provider value={{ connection, state, commands }}>
      {children}
    </DeviceContext.Provider>
  );
}

export function useDevice(): DeviceContextValue {
  const ctx = useContext(DeviceContext);
  if (!ctx) throw new Error("useDevice must be used within DeviceProvider");
  return ctx;
}

// Convenience re-exports for direct access
export function useDeviceConnection() {
  return useDevice().connection;
}

export function useDeviceData() {
  return useDevice().state;
}

export function useDeviceCommands() {
  return useDevice().commands;
}
