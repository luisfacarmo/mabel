import { NavLink } from "react-router-dom";
import { Home, Headphones, AudioLines, SlidersHorizontal, Settings } from "lucide-react";
import { motion } from "framer-motion";
import { useDeviceConnection } from "../../providers/DeviceProvider";

const NAV_ITEMS = [
  { to: "/", icon: Home, label: "Home" },
  { to: "/anc", icon: Headphones, label: "Noise Control" },
  { to: "/sound", icon: AudioLines, label: "Sound Effect" },
  { to: "/controls", icon: SlidersHorizontal, label: "Controls" },
  { to: "/settings", icon: Settings, label: "Settings" },
];

export default function Sidebar() {
  const { status } = useDeviceConnection();

  const dotColor =
    status === "connected"
      ? "bg-success shadow-[0_0_4px_var(--color-success)]"
      : "bg-error shadow-[0_0_4px_var(--color-error)]";

  const statusLabel =
    status === "connected" ? "Connected" : status === "reconnecting" ? "Scanning..." : "Offline";

  return (
    <aside className="w-[220px] shrink-0 bg-surface border-r border-border flex flex-col py-5">
      <div className="flex items-center gap-3 px-5 pb-5 border-b border-border mb-2">
        <div className="w-8 h-8 rounded-lg bg-accent flex items-center justify-center text-white text-sm font-bold">
          M
        </div>
        <span className="text-[16px] font-semibold text-text">Mabel</span>
      </div>

      <nav className="flex flex-col gap-0.5 px-2">
        {NAV_ITEMS.map(({ to, icon: Icon, label }) => (
          <NavLink
            key={to}
            to={to}
            className={({ isActive }) =>
              `flex items-center gap-3 px-3 py-[10px] rounded-lg text-[13px] font-medium transition-colors ${
                isActive
                  ? "bg-accent-bg text-accent-dark font-semibold border-r-[3px] border-accent"
                  : "text-text-secondary hover:bg-bg hover:text-text"
              }`
            }
          >
            {({ isActive }) => (
              <motion.div
                className="flex items-center gap-3 w-full"
                whileTap={{ scale: 0.97 }}
                transition={{ type: "spring", stiffness: 400, damping: 25 }}
              >
                <Icon size={18} className={isActive ? "text-accent" : ""} />
                {label}
              </motion.div>
            )}
          </NavLink>
        ))}
      </nav>

      <div className="flex-1" />

      <div className="flex items-center gap-2 px-5 text-[11px] text-text-muted">
        <motion.div
          className={`w-[7px] h-[7px] rounded-full ${dotColor}`}
          animate={status === "connected" ? { scale: [1, 1.2, 1] } : {}}
          transition={{ repeat: Infinity, duration: 2, ease: "easeInOut" }}
        />
        {statusLabel}
      </div>
    </aside>
  );
}
