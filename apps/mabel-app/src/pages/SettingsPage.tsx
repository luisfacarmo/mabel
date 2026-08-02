import { useState } from "react";
import { useDeviceData } from "../providers/DeviceProvider";
import { SectionLabel, ListCard, ToggleRow, LinkRow } from "../components/ui";

export default function SettingsPage() {
  const state = useDeviceData();
  const [sideTone, setSideTone] = useState(state?.toggles.sideTone ?? false);
  const [lowBattery, setLowBattery] = useState(state?.toggles.lowBatteryPrompt ?? true);
  const [voicePrompt, setVoicePrompt] = useState(state?.toggles.voicePrompt ?? true);
  const [dualConn, setDualConn] = useState(state?.dualConnections.enabled ?? true);
  const [limitVol, setLimitVol] = useState(state?.limitHighVolume.enabled ?? false);

  return (
    <div>
      <h2 className="text-[18px] font-semibold mb-5">Settings</h2>

      {/* Audio */}
      <div className="mb-6">
        <SectionLabel>Audio</SectionLabel>
        <ListCard>
          <ToggleRow label="Side Tone" checked={sideTone} onChange={setSideTone} />
          <ToggleRow label="Low Battery Prompt" checked={lowBattery} onChange={setLowBattery} />
          <ToggleRow label="Voice Prompt" checked={voicePrompt} onChange={setVoicePrompt} />
        </ListCard>
      </div>

      {/* Connections */}
      <div className="mb-6">
        <SectionLabel>Connections</SectionLabel>
        <ListCard>
          <ToggleRow label="Dual Connections" checked={dualConn} onChange={setDualConn} />
          <LinkRow label="Sound Mode" subtitle="Preferred audio quality" />
        </ListCard>
      </div>

      {/* Power */}
      <div className="mb-6">
        <SectionLabel>Power</SectionLabel>
        <ListCard>
          <LinkRow label="Auto Power Off" value={`${state?.autoPowerOff ?? 60} Min`} />
          <ToggleRow label="Limit High Volume" checked={limitVol} onChange={setLimitVol} />
        </ListCard>
      </div>

      {/* Device */}
      <div className="mb-6">
        <SectionLabel>Device</SectionLabel>
        <ListCard>
          <LinkRow label="Device Name" value="Space One Pro" />
          <LinkRow label="Firmware" value={state?.firmware ?? "—"} />
          <LinkRow label="Serial Number" value={state?.serialNumber ?? "—"} />
        </ListCard>
      </div>
    </div>
  );
}
