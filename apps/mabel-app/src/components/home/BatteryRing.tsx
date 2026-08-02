import { motion } from "framer-motion";

interface Props {
  level: number; // 0-10
  maxLevel: number; // 10
}

const RADIUS = 54;
const CIRCUMFERENCE = 2 * Math.PI * RADIUS;

function getColor(pct: number): string {
  if (pct > 50) return "var(--color-success)";
  if (pct > 20) return "var(--color-warning)";
  return "var(--color-error)";
}

export default function BatteryRing({ level, maxLevel }: Props) {
  const pct = Math.round((level / maxLevel) * 100);
  const offset = CIRCUMFERENCE * (1 - pct / 100);
  const color = getColor(pct);

  return (
    <div className="relative w-40 h-40 flex items-center justify-center">
      <svg width="160" height="160" viewBox="0 0 160 160" className="rotate-[-90deg]">
        {/* Track */}
        <circle
          cx="80"
          cy="80"
          r={RADIUS}
          fill="none"
          stroke="var(--color-border)"
          strokeWidth="8"
        />
        {/* Fill */}
        <motion.circle
          cx="80"
          cy="80"
          r={RADIUS}
          fill="none"
          stroke={color}
          strokeWidth="8"
          strokeLinecap="round"
          strokeDasharray={CIRCUMFERENCE}
          initial={{ strokeDashoffset: CIRCUMFERENCE }}
          animate={{ strokeDashoffset: offset }}
          transition={{ duration: 1.2, ease: "easeOut" }}
          style={{
            filter: `drop-shadow(0 0 8px ${color})`,
          }}
        />
      </svg>
      {/* Center text */}
      <div className="absolute inset-0 flex flex-col items-center justify-center">
        <motion.span
          className="text-4xl font-bold text-text-primary tabular-nums"
          initial={{ opacity: 0, scale: 0.8 }}
          animate={{ opacity: 1, scale: 1 }}
          transition={{ delay: 0.3, duration: 0.5 }}
        >
          {pct}
        </motion.span>
        <span className="text-xs text-text-secondary mt-0.5">%</span>
      </div>
    </div>
  );
}
