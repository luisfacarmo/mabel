import { useState, useEffect } from "react";
import type { ConnectionStatus } from "../lib/types";
import { isTauri, onConnectionState, type ConnectionState } from "../lib/tauri";

/**
 * Hook: connection status from the device loop.
 * In Tauri: listens to "connection-state" events from the Rust backend.
 * In browser: defaults to "connected" (mock mode).
 */
export function useConnection() {
  const [status, setStatus] = useState<ConnectionStatus>("disconnected");

  useEffect(() => {
    if (!isTauri) {
      setStatus("connected");
      return;
    }

    let unlisten: (() => void) | undefined;

    onConnectionState((state: ConnectionState) => {
      const mapped: ConnectionStatus =
        state === "connected"
          ? "connected"
          : state === "connecting"
            ? "reconnecting"
            : "disconnected";
      setStatus(mapped);
    }).then((fn) => {
      unlisten = fn;
    });

    return () => {
      unlisten?.();
    };
  }, []);

  return { status };
}
