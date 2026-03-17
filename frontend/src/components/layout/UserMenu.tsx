import * as React from "react";
import { useNavigate } from "react-router-dom";
import { UserAvatar } from "../ui/user-avatar";
import { useAuthStore } from "@/Stores/authStore";
import { useProjectStore } from "@/Stores/projectStore";
import { logout } from "@/Services/auth/authService";

function getInitials(name: string): string {
  return name
    .split(" ")
    .map((n) => n[0])
    .slice(0, 2)
    .join("")
    .toUpperCase();
}

export function UserMenu() {
  const user = useAuthStore((state) => state.user);
  const clearSession = useAuthStore((state) => state.clearSession);
  const clearProject = useProjectStore((state) => state.clearProject);
  const navigate = useNavigate();
  const [open, setOpen] = React.useState(false);
  const ref = React.useRef<HTMLDivElement>(null);

  React.useEffect(() => {
    if (!open) return;
    function handler(e: PointerEvent) {
      if (ref.current && !ref.current.contains(e.target as Node)) {
        setOpen(false);
      }
    }
    document.addEventListener("pointerdown", handler);
    return () => document.removeEventListener("pointerdown", handler);
  }, [open]);

  if (!user) return null;

  const initials = getInitials(user.name);

  async function handleLogout() {
    await logout().catch(() => {});
    clearSession();
    clearProject();
    navigate("/login", { replace: true });
  }

  return (
    <div ref={ref} className="relative">
      <button
        type="button"
        onClick={() => setOpen((o) => !o)}
        className="rounded-full transition hover:opacity-80"
      >
        <UserAvatar initials={initials} />
      </button>

      {open && (
        <div className="absolute right-0 top-11 z-50 w-64 rounded-2xl border border-slate-200 bg-white shadow-xl">
          {/* Profile header */}
          <div className="flex items-center gap-3 border-b border-slate-100 px-4 py-3">
            <UserAvatar initials={initials} />
            <div className="min-w-0">
              <p className="truncate text-sm font-semibold text-slate-900">{user.name}</p>
              <p className="truncate text-xs text-slate-500">{user.email}</p>
            </div>
          </div>

          {/* Menu items */}
          <div className="p-1.5">
            <button
              type="button"
              onClick={() => setOpen(false)}
              className="w-full rounded-lg px-3 py-2 text-left text-sm text-slate-700 hover:bg-slate-50"
            >
              Profile
            </button>
            <button
              type="button"
              onClick={() => setOpen(false)}
              className="w-full rounded-lg px-3 py-2 text-left text-sm text-slate-700 hover:bg-slate-50"
            >
              Terms of service
            </button>
            <button
              type="button"
              onClick={() => setOpen(false)}
              className="w-full rounded-lg px-3 py-2 text-left text-sm text-slate-700 hover:bg-slate-50"
            >
              Help center
            </button>
            <div className="my-1 border-t border-slate-100" />
            <button
              type="button"
              onClick={handleLogout}
              className="w-full rounded-lg px-3 py-2 text-left text-sm text-slate-700 hover:bg-slate-50"
            >
              Log out
            </button>
          </div>
        </div>
      )}
    </div>
  );
}
