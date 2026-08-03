import { useDeviceData, useDeviceCommands } from "../providers/DeviceProvider";
import { SectionLabel, ListCard, ToggleRow, LinkRow } from "../components/ui";

export default function SettingsPage() {
  const state = useDeviceData();
  const { setSidetone, setLdac, setDolby } = useDeviceCommands();

  const sideTone = state?.toggles.sideTone ?? false;
  const ldac = state?.toggles.ldac ?? false;
  const dolby = state?.toggles.dolbyAudio ?? true;
  const lowBattery = state?.toggles.lowBatteryPrompt ?? true;
  const voicePrompt = state?.toggles.voicePrompt ?? true;

  return (
    <div>
      <h2 className="text-[18px] font-semibold mb-5">Settings</h2>

      {/* Audio */}
      <div className="mb-6">
        <SectionLabel>Audio</SectionLabel>
        <ListCard>
          <ToggleRow label="Side Tone" checked={sideTone} onChange={setSidetone} />
          <ToggleRow label="Dolby Audio" checked={dolby} onChange={setDolby} />
          <ToggleRow label="LDAC" checked={ldac} onChange={setLdac} />
        </ListCard>
      </div>

      {/* Status (read-only from device) */}
      <div className="mb-6">
        <SectionLabel>Status</SectionLabel>
        <ListCard>
          <LinkRow label="Low Battery Prompt" value={lowBattery ? "On" : "Off"} />
          <LinkRow label="Voice Prompt" value={voicePrompt ? "On" : "Off"} />
          <LinkRow label="Dual Connections" value={state?.dualConnections.enabled ? "On" : "Off"} />
        </ListCard>
      </div>

      {/* Power */}
      <div className="mb-6">
        <SectionLabel>Power</SectionLabel>
        <ListCard>
          <LinkRow label="Auto Power Off" value={`${state?.autoPowerOff ?? 60} Min`} />
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
