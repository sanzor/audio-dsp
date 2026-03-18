import { NavLink } from "react-router-dom";
import { superAdminNavItems, type NavItem } from "../app/routes";
import { cn } from "@/lib/utils";

export default function AdminSidebar() {
  const visibleItems = superAdminNavItems;

  return (
    <aside className="flex h-screen w-64 flex-col border-r border-slate-200 bg-white/70 px-4 py-6 backdrop-blur">
      <div className="mb-6 px-1">
        <p className="text-xs font-semibold uppercase tracking-[0.3em] text-slate-400">
          Platform Admin
        </p>
      </div>

      <nav className="flex flex-1 flex-col gap-1">
        {visibleItems.map((item) => (
          <NavLink
            key={item.path}
            to={item.path}
            className={({ isActive }) =>
              cn(
                "rounded-lg px-3 py-2 text-sm font-medium transition",
                isActive
                  ? "bg-brand-600 text-white"
                  : "text-slate-600 hover:bg-brand-50 hover:text-brand-700"
              )
            }
          >
            {item.label}
          </NavLink>
        ))}
      </nav>
    </aside>
  );
}
