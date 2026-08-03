import { useDeviceData, useDeviceCommands } from "../providers/DeviceProvider";
import { SectionLabel, ListCard, ToggleRow, ModeCard } from "../components/ui";
import type { AmbientSoundMode } from "../lib/types";

const MODES: Array<{ key: AmbientSoundMode; icon: string; label: string }> = [
  { key: "noiseCanceling", icon: "🔇", label: "Noise<br>Cancellation" },
  { key: "normal", icon: "🙉", label: "Normal" },
  { key: "transparency", icon: "🌬️", label: "Transparency<br>Mode" },
];

export default function AncPage() {
  const state = useDeviceData();
  const { setSoundMode } = useDeviceCommands();

  const activeMode = state?.soundModes.ambientSoundMode ?? "noiseCanceling";
  const ncLevel = state?.soundModes.customNcLevel ?? 3;
  const windNoise = state?.soundModes.windNoiseReduction ?? false;

  const handleModeChange = (mode: AmbientSoundMode) => {
    setSoundMode(mode, undefined, undefined, windNoise);
  };

  const handleLevelChange = (level: number) => {
    setSoundMode(activeMode, "custom", level, windNoise);
  };

  const handleWindNoise = (enabled: boolean) => {
    setSoundMode(activeMode, undefined, undefined, enabled);
  };

  return (
    <div>
      <h2 className="text-[18px] font-semibold mb-5">Noise Control</h2>

      {/* Mode selector */}
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
      </div>

      {/* Level selector */}
      <div className="mb-6">
        <ListCard>
          <div className="px-[18px] py-[14px]">
            <div className="text-[13px] text-text-secondary mb-3">Level</div>
            <div className="flex items-center gap-2 justify-center">
              <span className="text-[11px] text-text-muted">Min</span>
              {[1, 2, 3, 4, 5].map((n) => (
                <button
                  key={n}
                  onClick={() => handleLevelChange(n)}
                  className={`w-8 h-8 rounded-lg flex items-center justify-center text-[13px] font-semibold cursor-pointer transition-all ${
                    ncLevel === n
                      ? "bg-accent text-white shadow-[0_2px_8px_rgba(77,208,225,0.3)]"
                      : "bg-border text-text-secondary hover:bg-accent-bg"
                  }`}
                >
                  {n}
                </button>
              ))}
              <span className="text-[11px] text-text-muted">Max</span>
            </div>
          </div>
        </ListCard>
      </div>

      {/* Wind noise */}
      <div className="mb-6">
        <ListCard>
          <ToggleRow label="Wind Noise Reduction" checked={windNoise} onChange={handleWindNoise} />
        </ListCard>
      </div>
    </div>
  );
}
