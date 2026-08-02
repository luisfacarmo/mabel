interface Props {
  icon: string;
  label: string;
  active: boolean;
  onClick: () => void;
}

/** A selectable mode card (circle icon + label below). Used for ANC modes. */
export default function ModeCard({ icon, label, active, onClick }: Props) {
  return (
    <button
      onClick={onClick}
      className={`
        flex-1 flex flex-col items-center gap-2 p-5 rounded-[var(--radius-lg)] border-2 cursor-pointer transition-all
        ${active
          ? "border-accent bg-accent-bg"
          : "border-border bg-surface hover:border-accent hover:-translate-y-0.5"
        }
      `}
      style={{ boxShadow: "var(--shadow-md)" }}
    >
      <div
        className={`
          w-[44px] h-[44px] rounded-full flex items-center justify-center text-[20px] transition-colors
          ${active ? "bg-accent text-white" : "bg-[#f3f4f6]"}
        `}
      >
        {icon}
      </div>
      <div
        className={`text-[12px] font-medium text-center leading-tight ${
          active ? "text-accent-dark font-semibold" : "text-text-secondary"
        }`}
        dangerouslySetInnerHTML={{ __html: label }}
      />
    </button>
  );
}
