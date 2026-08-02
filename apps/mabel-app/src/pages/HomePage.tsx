import { useState } from "react";
import { useDeviceData } from "../providers/DeviceProvider";
import { SectionLabel, ListCard, ToggleRow, LinkRow, ModeCard } from "../components/ui";
import type { AmbientSoundMode } from "../lib/types";

const MODES: Array<{ key: AmbientSoundMode; icon: string; label: string }> = [
  { key: "noiseCanceling", icon: "🔇", label: "Noise<br>Cancellation" },
  { key: "normal", icon: "🙉", label: "Normal" },
  { key: "transparency", icon: "🌬️", label: "Transparency<br>Mode" },
];

export default function HomePage() {
  const state = useDeviceData();
  const [activeMode, setActiveMode] = useState<AmbientSoundMode>(
    state?.soundModes.ambientSoundMode ?? "noiseCanceling"
  );
  const [windNoise, setWindNoise] = useState(state?.soundModes.windNoiseReduction ?? false);
  const [dolby, setDolby] = useState(state?.toggles.dolbyAudio ?? true);
  const [ldac, setLdac] = useState(state?.toggles.ldac ?? false);

  if (!state) {
    return <p className="text-text-secondary">No device connected</p>;
  }

  const batteryPct = Math.round((state.battery.level / state.battery.maxLevel) * 100);

  return (
    <div>
      {/* Device hero */}
      <div className="flex flex-col items-center py-8">
        <div className="w-[140px] h-[140px] rounded-full bg-gradient-to-br from-accent-bg to-[#b2ebf2] flex items-center justify-center text-[56px] mb-4">
          🎧
        </div>
        <h1 className="text-[20px] font-semibold text-text mb-1">Space One Pro</h1>
        <div className="flex items-center gap-2 text-[13px] text-text-secondary">
          <div className="w-[60px] h-[5px] bg-border rounded-full overflow-hidden">
            <div className="h-full rounded-full bg-gradient-to-r from-accent to-accent-dark" style={{ width: `${batteryPct}%` }} />
          </div>
          {batteryPct}%
        </div>
      </div>

      {/* Ambient Sound */}
      <div className="mb-6">
        <SectionLabel>🔊 Ambient Sound</SectionLabel>
        <div className="flex gap-3 mb-4">
          {MODES.map(({ key, icon, label }) => (
            <ModeCard
              key={key}
              icon={icon}
              label={label}
              active={activeMode === key}
              onClick={() => setActiveMode(key)}
            />
          ))}
        </div>
        <ListCard>
          <LinkRow label="Mode" value="Adaptive ANC" />
          <ToggleRow label="Wind Noise Reduction" checked={windNoise} onChange={setWindNoise} />
        </ListCard>
      </div>

      {/* Sound Effect */}
      <div className="mb-6">
        <ListCard>
          <LinkRow icon="🎵" label="Sound Effect" subtitle="soundcore Signature" />
        </ListCard>
      </div>

      {/* Toggles */}
      <div className="mb-6">
        <ListCard>
          <ToggleRow label="Dolby Audio" checked={dolby} onChange={setDolby} />
          <ToggleRow label="LDAC" checked={ldac} onChange={setLdac} />
        </ListCard>
      </div>
    </div>
  );
}
