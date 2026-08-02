import { useState, useCallback } from "react";
import type { ConnectionStatus } from "../lib/types";

// Hook: connection lifecycle (connect, disconnect, status).
// Mock implementation — swap internals for Tauri IPC in Phase E.

export function useConnection() {
  const [status, setStatus] = useState<ConnectionStatus>("connected");

  const connect = useCallback(async () => {
    setStatus("reconnecting");
    // Mock: simulate connection delay
    await new Promise((r) => setTimeout(r, 800));
    setStatus("connected");
  }, []);

  const disconnect = useCallback(async () => {
    setStatus("disconnected");
  }, []);

  return { status, connect, disconnect };
}
