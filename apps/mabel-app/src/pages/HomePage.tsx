import { useDeviceData, useDeviceCommands, useDeviceConnection } from "../providers/DeviceProvider";
import { SectionLabel, ListCard, ToggleRow, LinkRow, ModeCard } from "../components/ui";
import { ShieldOff, Headphones, Ear, Bluetooth, BluetoothOff } from "lucide-react";
import type { AmbientSoundMode } from "../lib/types";
import * as tauri from "../lib/tauri";
import headsetImg from "../assets/headset.png";

const MODES: Array<{ key: AmbientSoundMode; icon: React.ReactNode; label: string }> = [
  { key: "noiseCanceling", icon: <ShieldOff size={22} />, label: "Noise<br>Cancellation" },
  { key: "normal", icon: <Headphones size={22} />, label: "Normal" },
  { key: "transparency", icon: <Ear size={22} />, label: "Transparency<br>Mode" },
];

export default function HomePage() {
  const state = useDeviceData();
  const { status } = useDeviceConnection();
  const { setSoundMode, setLdac, setDolby } = useDeviceCommands();

  const isConnected = status === "connected" && state !== null;

  if (!isConnected) {
    return (
      <div className="flex flex-col items-center justify-center h-full gap-6 py-16">
        <div className="w-[120px] h-[120px] rounded-full bg-surface flex items-center justify-center border border-border overflow-hidden p-4">
          <img src={headsetImg} alt="Space One Pro" className="w-full h-full object-contain opacity-50" />
        </div>
        <div className="text-center">
          <h2 className="text-[18px] font-semibold text-text mb-1">Space One Pro</h2>
          <p className="text-[13px] text-text-muted">
            {status === "reconnecting" ? "Connecting..." : "Not connected"}
          </p>
        </div>
        <button
          onClick={() => { if (tauri.isTauri) tauri.connectDevice().catch(console.error); }}
          disabled={status === "reconnecting"}
          className="flex items-center gap-2 px-5 py-2.5 rounded-lg bg-accent text-white text-[13px] font-medium cursor-pointer hover:bg-accent-dark transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
        >
          <Bluetooth size={16} />
          {status === "reconnecting" ? "Connecting..." : "Connect"}
        </button>
      </div>
    );
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
        <button
          onClick={() => { if (tauri.isTauri) tauri.disconnectDevice().catch(console.error); }}
          className="mt-3 flex items-center gap-1.5 px-3 py-1.5 rounded-md text-[11px] text-text-muted border border-border hover:border-error hover:text-error transition-colors cursor-pointer"
        >
          <BluetoothOff size={12} />
          Disconnect
        </button>
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
          <LinkRow label="Sound Effect" subtitle={state.equalizer.preset ?? "Custom"} />
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
