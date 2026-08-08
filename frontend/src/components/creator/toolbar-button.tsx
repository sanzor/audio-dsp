import type { ReactNode } from "react";
import { creatorToolbarColors } from "./creatorToolbarColors";

type ToolbarButtonVariant = "play" | "stop" | "save" | "compile" | "bypass" | "publish" | "validate";

interface ToolbarButtonProps {
  variant: ToolbarButtonVariant;
  onClick: () => void;
  disabled?: boolean;
  title?: string;
  /** When true, overrides text color to var(--text-muted) while keeping the variant's border color — used for the Bypass toggle's "Bypassed" (active) state. */
  muted?: boolean;
  children: ReactNode;
}

const variantColor: Record<ToolbarButtonVariant, (typeof creatorToolbarColors)[keyof typeof creatorToolbarColors]> = {
  play: creatorToolbarColors.green,
  stop: creatorToolbarColors.red,
  save: creatorToolbarColors.green,
  compile: creatorToolbarColors.blue,
  bypass: creatorToolbarColors.blue,
  publish: creatorToolbarColors.pink,
  validate: creatorToolbarColors.blue,
};

export function ToolbarButton({ variant, onClick, disabled, title, muted, children }: ToolbarButtonProps) {
  const token = variantColor[variant];
  return (
    <button
      onClick={onClick}
      disabled={disabled}
      title={title}
      className="font-mono font-bold px-2.5 py-0.5 rounded text-[10px] transition-colors"
      style={{
        color: muted ? "var(--text-muted)" : token.color,
        border: token.border,
        opacity: disabled ? 0.5 : 1,
      }}
    >
      {children}
    </button>
  );
}
