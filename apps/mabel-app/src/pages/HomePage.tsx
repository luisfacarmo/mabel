import { useDeviceData, useDeviceConnection } from "../providers/DeviceProvider";
import DeviceHero from "../components/home/DeviceHero";
import BatteryRing from "../components/home/BatteryRing";
import QuickStatus from "../components/home/QuickStatus";

export default function HomePage() {
  const state = useDeviceData();
  const { status } = useDeviceConnection();

  if (!state) {
    return (
      <div className="flex items-center justify-center h-full">
        <p className="text-text-secondary">No device connected</p>
      </div>
    );
  }

  return (
    <div className="flex flex-col items-center justify-center h-full gap-8">
      <DeviceHero
        deviceName="Space One Pro"
        connectionStatus={status}
      />

      <BatteryRing
        level={state.battery.level}
        maxLevel={state.battery.maxLevel}
      />

      <QuickStatus
        ancMode={state.soundModes.ambientSoundMode}
        eqPreset={state.equalizer.preset}
        ldac={state.toggles.ldac}
      />
    </div>
  );
}
