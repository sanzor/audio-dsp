import type { ReactNode } from "react";

interface PositionedMenuProps {
  x: number;
  y: number;
  onClose: () => void;
  children: ReactNode;
}

export function PositionedMenu({ x, y, onClose, children }: PositionedMenuProps) {
  return (
    <>
      {/* Full-screen dismiss backdrop */}
      <div
        style={{ position: "fixed", inset: 0, zIndex: 999 }}
        onMouseDown={onClose}
      />

      {/* Menu */}
      <div
        style={{
          position: "fixed",
          top: y,
          left: x,
          zIndex: 1000,
          backgroundColor: "var(--bg-dark)",
          border: "1px solid rgba(255,255,255,0.1)",
          borderRadius: 6,
          minWidth: 160,
          padding: "4px 0",
          boxShadow: "0 8px 24px rgba(0,0,0,0.5)",
        }}
      >
        {children}
      </div>
    </>
  );
}

interface MenuItemProps {
  onClick: () => void;
  disabled?: boolean;
  children: ReactNode;
}

export function MenuItem({ onClick, disabled = false, children }: MenuItemProps) {
  return (
    <button
      disabled={disabled}
      onMouseDown={(e) => {
        e.stopPropagation(); // don't trigger backdrop
        if (!disabled) onClick();
      }}
      style={{
        display: "block",
        width: "100%",
        textAlign: "left",
        padding: "6px 12px",
        fontSize: "0.875rem",
        background: "none",
        border: "none",
        cursor: disabled ? "default" : "pointer",
        color: disabled ? "var(--text-muted)" : "var(--text-main)",
        borderRadius: 0,
      }}
      onMouseEnter={(e) => {
        if (!disabled) (e.currentTarget as HTMLButtonElement).style.backgroundColor = "var(--accent-blue)";
      }}
      onMouseLeave={(e) => {
        (e.currentTarget as HTMLButtonElement).style.backgroundColor = "transparent";
      }}
    >
      {children}
    </button>
  );
}
