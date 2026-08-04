import { motion } from "framer-motion";
import type { ReactNode } from "react";

interface Props {
  icon: ReactNode;
  label: string;
  active: boolean;
  onClick: () => void;
}

/** A selectable mode card (circle icon + label below). Used for ANC modes. */
export default function ModeCard({ icon, label, active, onClick }: Props) {
  return (
    <motion.button
      onClick={onClick}
      whileHover={{ scale: 1.02 }}
      whileTap={{ scale: 0.97 }}
      transition={{ type: "spring", stiffness: 400, damping: 25 }}
      className={`
        flex-1 flex flex-col items-center gap-2 p-5 rounded-[var(--radius-lg)] border-2 cursor-pointer transition-all
        ${active
          ? "border-accent bg-accent-bg"
          : "border-border bg-surface hover:border-accent"
        }
      `}
      style={{ boxShadow: "var(--shadow-md)" }}
    >
      <div
        className={`
          w-[44px] h-[44px] rounded-full flex items-center justify-center transition-colors
          ${active ? "bg-accent text-white" : "bg-surface-hover text-text-secondary"}
        `}
      >
        {icon}
      </div>
      <div
        className={`text-[12px] font-medium text-center leading-tight ${
          active ? "text-accent-dark font-semibold" : "text-text-secondary"
        }`}
        dangerouslySetInnerHTML={{ __html: label }}
      />
    </motion.button>
  );
}
