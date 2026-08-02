interface Props {
  label: string;
  subtitle?: string;
  value?: string;
  icon?: string;
  onClick?: () => void;
}

/** A navigable row with label, optional subtitle, value and chevron. */
export default function LinkRow({ label, subtitle, value, icon, onClick }: Props) {
  return (
    <div
      onClick={onClick}
      className="flex items-center justify-between px-[18px] py-[14px] cursor-pointer hover:bg-surface-hover transition-colors [&+&]:border-t [&+&]:border-divider"
    >
      <div className="flex items-center gap-3">
        {icon && <span className="text-[18px] w-6 text-center">{icon}</span>}
        <div>
          <div className="text-[14px] text-text">{label}</div>
          {subtitle && <div className="text-[12px] text-text-muted mt-0.5">{subtitle}</div>}
        </div>
      </div>
      <div className="flex items-center gap-2">
        {value && <span className="text-[13px] text-text-muted">{value}</span>}
        <span className="text-[16px] text-text-muted">›</span>
      </div>
    </div>
  );
}
