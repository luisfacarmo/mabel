import { useState, useEffect, useCallback } from "react";
import type { ConnectionStatus } from "../lib/types";

// Check if running inside Tauri (vs browser for dev)
const isTauri = typeof window !== "undefined" && "__TAURI_INTERNALS__" in window;

export function useConnection() {
  const [status, setStatus] = useState<ConnectionStatus>("disconnected");

  // Poll for device presence every 3 seconds
  useEffect(() => {
    if (!isTauri) {
      // In browser dev mode, default to connected (mock)
      setStatus("connected");
      return;
    }

    let active = true;

    const poll = async () => {
      try {
        const { invoke } = await import("@tauri-apps/api/core");
        const online = await invoke<boolean>("check_device_online");
        if (active) setStatus(online ? "connected" : "disconnected");
      } catch {
        if (active) setStatus("disconnected");
      }
    };

    poll(); // Initial check
    const interval = setInterval(poll, 3000);

    return () => {
      active = false;
      clearInterval(interval);
    };
  }, []);

  const connect = useCallback(async () => {
    setStatus("reconnecting");
    if (isTauri) {
      try {
        const { invoke } = await import("@tauri-apps/api/core");
        const online = await invoke<boolean>("check_device_online");
        setStatus(online ? "connected" : "disconnected");
      } catch {
        setStatus("disconnected");
      }
    } else {
      await new Promise((r) => setTimeout(r, 800));
      setStatus("connected");
    }
  }, []);

  const disconnect = useCallback(async () => {
    setStatus("disconnected");
  }, []);

  return { status, connect, disconnect };
}
