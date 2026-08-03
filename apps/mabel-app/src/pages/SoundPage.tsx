import { useDeviceData, useDeviceCommands } from "../providers/DeviceProvider";
import { SectionLabel, ListCard } from "../components/ui";
import { EQ_PRESETS } from "../lib/mock-data";

export default function SoundPage() {
  const state = useDeviceData();
  const { setEqualizer } = useDeviceCommands();

  const activePreset = state?.equalizer.preset ?? "Podcast";
  const bands = state?.equalizer.bands ?? EQ_PRESETS["Podcast"];

  const freqs = ["100", "200", "400", "800", "1.6k", "3.2k", "6.4k", "12.8k", "16k", "20k"];

  const handlePresetChange = (name: string) => {
    const presetBands = EQ_PRESETS[name];
    if (presetBands) {
      setEqualizer(name, presetBands);
    }
  };

  return (
    <div>
      <h2 className="text-[18px] font-semibold mb-5">Sound Effect</h2>

      {/* EQ Visualizer */}
      <div className="mb-6">
        <SectionLabel>Equalizer</SectionLabel>
        <ListCard>
          <div className="p-5">
            {/* Graph area */}
            <div className="relative h-[160px] flex items-center justify-between mb-2">
              {/* Center line */}
              <div className="absolute top-1/2 left-0 right-0 h-px bg-divider" />
              {/* Dots */}
              {(bands ?? []).map((val, i) => {
                const offset = ((val - 90) / 90) * -60;
                return (
                  <div
                    key={i}
                    className={`w-[14px] h-[14px] rounded-full border-[2.5px] border-accent z-10 transition-all ${
                      val !== 90 ? "bg-accent" : "bg-surface"
                    }`}
                    style={{ marginTop: `${offset}px` }}
                  />
                );
              })}
            </div>
            {/* Frequency labels */}
            <div className="flex justify-between text-[9px] text-text-muted px-1">
              {freqs.map((f) => <span key={f}>{f}</span>)}
            </div>
          </div>
        </ListCard>

        {/* Preset chips */}
        <div className="flex gap-2 flex-wrap">
          {Object.keys(EQ_PRESETS).map((name) => (
            <button
              key={name}
              onClick={() => handlePresetChange(name)}
              className={`px-3 py-1.5 rounded-full text-[12px] font-medium cursor-pointer transition-all ${
                activePreset === name
                  ? "bg-accent text-white"
                  : "bg-divider text-text-secondary hover:bg-accent-bg hover:text-accent-dark"
              }`}
            >
              {name}
            </button>
          ))}
        </div>
      </div>
    </div>
  );
}
