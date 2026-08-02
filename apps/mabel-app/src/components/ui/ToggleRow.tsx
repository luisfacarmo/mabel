import * as Switch from "@radix-ui/react-switch";

interface Props {
  label: string;
  checked: boolean;
  onChange: (checked: boolean) => void;
}

/** A row with a label on the left and an iOS-style toggle on the right. */
export default function ToggleRow({ label, checked, onChange }: Props) {
  return (
    <div className="flex items-center justify-between px-[18px] py-[14px] hover:bg-surface-hover transition-colors [&+&]:border-t [&+&]:border-divider">
      <span className="text-[14px] text-text">{label}</span>
      <Switch.Root
        checked={checked}
        onCheckedChange={onChange}
        className="w-[44px] h-[24px] rounded-full bg-border data-[state=checked]:bg-accent transition-colors cursor-pointer relative"
      >
        <Switch.Thumb className="block w-[20px] h-[20px] rounded-full bg-white shadow-[0_1px_3px_rgba(0,0,0,0.15)] transition-transform translate-x-[2px] data-[state=checked]:translate-x-[22px]" />
      </Switch.Root>
    </div>
  );
}
