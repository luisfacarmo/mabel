import type { ReactNode } from "react";

interface Props {
  children: ReactNode;
}

/** White rounded card container with shadow. Rows go inside. */
export default function ListCard({ children }: Props) {
  return (
    <div className="bg-surface rounded-[var(--radius-lg)] shadow-[var(--shadow-md)] overflow-hidden mb-4">
      {children}
    </div>
  );
}
