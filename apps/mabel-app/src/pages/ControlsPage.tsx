import { useState } from "react";
import { useDeviceData } from "../providers/DeviceProvider";
import { SectionLabel, ListCard, LinkRow } from "../components/ui";
import * as Select from "@radix-ui/react-select";
import { ChevronDown, Check } from "lucide-react";

const DOUBLE_PRESS_OPTIONS = [
  { value: "bassUp", label: "Bass Up" },
  { value: "none", label: "None" },
];

export default function ControlsPage() {
  const state = useDeviceData();
  const [doublePressAction, setDoublePressAction] = useState(
    state?.buttonConfig.doublePressAction ?? "bassUp"
  );

  return (
    <div>
      <h2 className="text-[18px] font-semibold mb-5">Controls</h2>

      {/* Button controls */}
      <div className="mb-6">
        <SectionLabel>Audio</SectionLabel>
        <ListCard>
          <div className="px-[18px] py-[14px]">
            <div className="flex items-center justify-between">
              <div>
                <div className="text-[14px] text-text">Double Press</div>
                <div className="text-[12px] text-text-muted mt-0.5">
                  Left or right earcup
                </div>
              </div>
              <Select.Root value={doublePressAction} onValueChange={setDoublePressAction}>
                <Select.Trigger className="inline-flex items-center gap-1.5 px-3 py-1.5 rounded-lg bg-surface-hover text-[13px] text-text cursor-pointer border border-border hover:border-accent transition-colors outline-none">
                  <Select.Value />
                  <Select.Icon>
                    <ChevronDown size={14} className="text-text-muted" />
                  </Select.Icon>
                </Select.Trigger>
                <Select.Portal>
                  <Select.Content
                    className="bg-surface border border-border rounded-lg shadow-lg overflow-hidden z-50"
                    position="popper"
                    sideOffset={4}
                  >
                    <Select.Viewport className="p-1">
                      {DOUBLE_PRESS_OPTIONS.map((opt) => (
                        <Select.Item
                          key={opt.value}
                          value={opt.value}
                          className="flex items-center gap-2 px-3 py-2 rounded-md text-[13px] text-text cursor-pointer outline-none data-[highlighted]:bg-accent-bg data-[highlighted]:text-accent-dark transition-colors"
                        >
                          <Select.ItemIndicator className="w-4">
                            <Check size={14} className="text-accent" />
                          </Select.ItemIndicator>
                          <Select.ItemText>{opt.label}</Select.ItemText>
                        </Select.Item>
                      ))}
                    </Select.Viewport>
                  </Select.Content>
                </Select.Portal>
              </Select.Root>
            </div>
          </div>
        </ListCard>
      </div>

      {/* Call controls (not adjustable for A3062) */}
      <div className="mb-6">
        <SectionLabel>Call</SectionLabel>
        <ListCard>
          <LinkRow
            label="Answer / Hang Up"
            subtitle="Double press either earcup"
            value="Fixed"
          />
          <LinkRow
            label="Reject Incoming Call"
            subtitle="Long press 2 seconds"
            value="Fixed"
          />
        </ListCard>
        <p className="text-[11px] text-text-muted mt-2 px-1">
          Call controls are hardware-defined and cannot be customized.
        </p>
      </div>

      {/* Touch controls info */}
      <div className="mb-6">
        <SectionLabel>Touch Gestures</SectionLabel>
        <ListCard>
          <LinkRow
            label="Single Tap"
            subtitle="Play / Pause"
            value="Fixed"
          />
          <LinkRow
            label="Swipe Up / Down"
            subtitle="Volume Up / Down"
            value="Fixed"
          />
          <LinkRow
            label="Long Press"
            subtitle="Cycle ANC Mode"
            value="Fixed"
          />
        </ListCard>
      </div>
    </div>
  );
}
