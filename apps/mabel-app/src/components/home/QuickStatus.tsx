import { useNavigate } from "react-router-dom";
import { Headphones, AudioLines, Radio } from "lucide-react";
import { motion } from "framer-motion";
import type { AmbientSoundMode } from "../../lib/types";

interface Props {
  ancMode: AmbientSoundMode;
  eqPreset: string | null;
  ldac: boolean;
}

const ANC_LABELS: Record<AmbientSoundMode, string> = {
  noiseCanceling: "Noise Canceling",
  transparency: "Transparency",
  normal: "Normal",
};

export default function QuickStatus({ ancMode, eqPreset, ldac }: Props) {
  const navigate = useNavigate();

  const chips = [
    {
      icon: Headphones,
      label: ANC_LABELS[ancMode],
      onClick: () => navigate("/anc"),
    },
    {
      icon: AudioLines,
      label: eqPreset ?? "Custom EQ",
      onClick: () => navigate("/sound"),
    },
    {
      icon: Radio,
      label: ldac ? "LDAC" : "SBC",
      onClick: () => navigate("/settings"),
    },
  ];

  return (
    <motion.div
      className="flex flex-wrap justify-center gap-2"
      initial={{ opacity: 0 }}
      animate={{ opacity: 1 }}
      transition={{ delay: 0.6, duration: 0.4 }}
    >
      {chips.map(({ icon: Icon, label, onClick }) => (
        <button
          key={label}
          onClick={onClick}
          className="flex items-center gap-1.5 px-3 py-1.5 rounded-full bg-surface border border-border text-xs text-text-secondary hover:text-accent hover:border-accent/30 transition-colors cursor-pointer"
        >
          <Icon size={12} />
          <span>{label}</span>
        </button>
      ))}
    </motion.div>
  );
}
