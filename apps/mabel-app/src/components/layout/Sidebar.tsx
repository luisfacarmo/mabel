import { NavLink } from "react-router-dom";
import { Home, Headphones, AudioLines, Sliders, Settings } from "lucide-react";
import * as Tooltip from "@radix-ui/react-tooltip";

const NAV_ITEMS = [
  { to: "/", icon: Home, label: "Home" },
  { to: "/anc", icon: Headphones, label: "ANC" },
  { to: "/sound", icon: AudioLines, label: "Sound" },
  { to: "/controls", icon: Sliders, label: "Controls" },
  { to: "/settings", icon: Settings, label: "Settings" },
];

export default function Sidebar() {
  return (
    <Tooltip.Provider delayDuration={300}>
      <nav className="flex flex-col items-center w-16 shrink-0 border-r border-border bg-[#0a0a0c] py-4 gap-1">
        {NAV_ITEMS.map(({ to, icon: Icon, label }) => (
          <Tooltip.Root key={to}>
            <Tooltip.Trigger asChild>
              <NavLink
                to={to}
                className={({ isActive }) =>
                  `flex items-center justify-center w-10 h-10 rounded-lg transition-colors ${
                    isActive
                      ? "bg-accent/15 text-accent"
                      : "text-text-secondary hover:text-text-primary hover:bg-surface-hover"
                  }`
                }
              >
                <Icon size={20} />
              </NavLink>
            </Tooltip.Trigger>
            <Tooltip.Portal>
              <Tooltip.Content
                side="right"
                sideOffset={8}
                className="bg-surface border border-border rounded-md px-2.5 py-1.5 text-xs text-text-primary shadow-lg"
              >
                {label}
                <Tooltip.Arrow className="fill-surface" />
              </Tooltip.Content>
            </Tooltip.Portal>
          </Tooltip.Root>
        ))}

        <div className="flex-1" />

        {/* Connection indicator */}
        <div className="flex items-center justify-center w-10 h-10">
          <div className="w-2.5 h-2.5 rounded-full bg-success shadow-[0_0_6px_var(--color-success)]" />
        </div>
      </nav>
    </Tooltip.Provider>
  );
}
