import { motion } from "framer-motion";
import type { ConnectionStatus } from "../../lib/types";
import headsetImg from "../../assets/headset.png";

interface Props {
  deviceName: string;
  connectionStatus: ConnectionStatus;
}

const STATUS_CONFIG: Record<ConnectionStatus, { label: string; color: string; pulse: boolean }> = {
  connected: { label: "Connected", color: "bg-success", pulse: false },
  disconnected: { label: "Disconnected", color: "bg-error", pulse: false },
  reconnecting: { label: "Reconnecting...", color: "bg-warning", pulse: true },
};

export default function DeviceHero({ deviceName, connectionStatus }: Props) {
  const { label, color, pulse } = STATUS_CONFIG[connectionStatus];

  return (
    <motion.div
      className="flex flex-col items-center gap-4"
      initial={{ opacity: 0, y: 20 }}
      animate={{ opacity: 1, y: 0 }}
      transition={{ duration: 0.5 }}
    >
      {/* Device image */}
      <div className="w-32 h-32 rounded-full bg-surface flex items-center justify-center border border-border overflow-hidden p-3">
        <img
          src={headsetImg}
          alt={deviceName}
          className="w-full h-full object-contain drop-shadow-lg"
        />
      </div>

      {/* Device name */}
      <h1 className="text-xl font-semibold text-text-primary">{deviceName}</h1>

      {/* Connection badge */}
      <div className="flex items-center gap-2">
        <div className={`relative w-2.5 h-2.5 rounded-full ${color}`}>
          {pulse && (
            <div className={`absolute inset-0 rounded-full ${color} animate-ping opacity-75`} />
          )}
        </div>
        <span className="text-xs text-text-secondary">{label}</span>
      </div>
    </motion.div>
  );
}
