import { useDeviceData, useDeviceCommands } from "../providers/DeviceProvider";
import { SectionLabel, ListCard, ToggleRow, LinkRow, ModeCard } from "../components/ui";
import type { AmbientSoundMode } from "../lib/types";
import headsetImg from "../assets/headset.png";

const MODES: Array<{ key: AmbientSoundMode; icon: string; label: string }> = [
  { key: "noiseCanceling", icon: "🔇", label: "Noise<br>Cancellation" },
  { key: "normal", icon: "🙉", label: "Normal" },
  { key: "transparency", icon: "🌬️", label: "Transparency<br>Mode" },
];

export default function HomePage() {
  const state = useDeviceData();
  const { setSoundMode, setLdac, setDolby } = useDeviceCommands();

  if (!state) {
    return <p className="text-text-secondary">No device connected</p>;
  }

  const activeMode = state.soundModes.ambientSoundMode;
  const windNoise = state.soundModes.windNoiseReduction;
  const dolby = state.toggles.dolbyAudio;
  const ldac = state.toggles.ldac;
  const batteryPct = Math.round((state.battery.level / state.battery.maxLevel) * 100);

  const handleModeChange = (mode: AmbientSoundMode) => {
    setSoundMode(mode, undefined, undefined, windNoise);
  };

  return (
    <div>
      {/* Device hero */}
      <div className="flex flex-col items-center py-8">
        <div className="w-[140px] h-[140px] rounded-full bg-gradient-to-br from-accent-bg to-[#b2ebf2] flex items-center justify-center mb-4 overflow-hidden p-4">
          <img src={headsetImg} alt="Space One Pro" className="w-full h-full object-contain drop-shadow-lg" />
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
        <SectionLabel>Ambient Sound</SectionLabel>
        <div className="flex gap-3 mb-4">
          {MODES.map(({ key, icon, label }) => (
            <ModeCard
              key={key}
              icon={icon}
              label={label}
              active={activeMode === key}
              onClick={() => handleModeChange(key)}
            />
          ))}
        </div>
        <ListCard>
          <LinkRow label="Mode" value={state.soundModes.noiseCancelingMode === "adaptive" ? "Adaptive ANC" : "Custom ANC"} />
          <ToggleRow label="Wind Noise Reduction" checked={windNoise} onChange={(v) => setSoundMode(activeMode, undefined, undefined, v)} />
        </ListCard>
      </div>

      {/* Sound Effect */}
      <div className="mb-6">
        <ListCard>
          <LinkRow icon="🎵" label="Sound Effect" subtitle={state.equalizer.preset ?? "Custom"} />
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
