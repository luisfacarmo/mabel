import type { ReactNode } from "react";

interface Props {
  children: ReactNode;
}

export default function SectionLabel({ children }: Props) {
  return (
    <div className="text-[11px] font-semibold text-text-muted uppercase tracking-wide mb-2 px-1 flex items-center gap-2">
      {children}
    </div>
  );
}
