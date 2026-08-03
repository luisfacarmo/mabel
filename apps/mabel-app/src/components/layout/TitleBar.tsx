import { getCurrentWindow } from "@tauri-apps/api/window";
import { Minus, Square, X } from "lucide-react";

const appWindow = getCurrentWindow();

export default function TitleBar() {
  return (
    <div
      data-tauri-drag-region
      className="h-9 flex items-center justify-between bg-surface border-b border-border select-none shrink-0"
    >
      {/* App title */}
      <div data-tauri-drag-region className="flex items-center gap-2 pl-4 text-[12px] text-text-muted font-medium">
        <span className="w-4 h-4 rounded bg-accent flex items-center justify-center text-[9px] text-white font-bold">
          M
        </span>
        Mabel
      </div>

      {/* Window controls */}
      <div className="flex items-center h-full">
        <button
          onClick={() => appWindow.minimize()}
          className="h-full w-11 flex items-center justify-center text-text-muted hover:bg-surface-hover hover:text-text transition-colors"
          aria-label="Minimize"
        >
          <Minus size={14} />
        </button>
        <button
          onClick={() => appWindow.toggleMaximize()}
          className="h-full w-11 flex items-center justify-center text-text-muted hover:bg-surface-hover hover:text-text transition-colors"
          aria-label="Maximize"
        >
          <Square size={12} />
        </button>
        <button
          onClick={() => appWindow.close()}
          className="h-full w-11 flex items-center justify-center text-text-muted hover:bg-error hover:text-white transition-colors"
          aria-label="Close"
        >
          <X size={14} />
        </button>
      </div>
    </div>
  );
}
