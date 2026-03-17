// import * as React from "react";
// import { useNavigate, useLocation } from "react-router-dom";
// import { ChevronDown, LayoutDashboard, ShieldCheck } from "lucide-react";
// import { useMemberships } from "@/hooks/useMemberships";
// import { useOrgStore } from "@/store/orgStore";
// import { orgPath } from "@/utils/orgPaths";
// import { cn } from "@/lib/utils";

// type Mode = "app" | "admin";

// const MODE_META: Record<Mode, { label: string; description: string; Icon: React.ElementType; color: string }> = {
//   app: {
//     label: "App",
//     description: "Tenant workspace",
//     Icon: LayoutDashboard,
//     color: "bg-brand-600",
//   },
//   admin: {
//     label: "Admin",
//     description: "Platform control",
//     Icon: ShieldCheck,
//     color: "bg-slate-800",
//   },
// };

// export function ModeSwitcher() {
//   const navigate = useNavigate();
//   const location = useLocation();
//   const activeMembership = useOrgStore((state) => state.activeMembership);
//   const { data } = useMemberships();
//   const memberships = data?.memberships ?? [];
//   const isSuperAdmin = memberships.some((m) => m.roles.includes("superadmin"));

//   const [open, setOpen] = React.useState(false);
//   const ref = React.useRef<HTMLDivElement>(null);

//   const currentMode: Mode = location.pathname.startsWith("/admin") ? "admin" : "app";
//   const { label, Icon, color } = MODE_META[currentMode];

//   // Close on outside click
//   React.useEffect(() => {
//     function onPointerDown(e: PointerEvent) {
//       if (ref.current && !ref.current.contains(e.target as Node)) {
//         setOpen(false);
//       }
//     }
//     if (open) {
//       document.addEventListener("pointerdown", onPointerDown);
//     }
//     return () => document.removeEventListener("pointerdown", onPointerDown);
//   }, [open]);

//   function handleSelect(mode: Mode) {
//     setOpen(false);
//     if (mode === "admin") {
//       navigate("/admin/products");
//     } else {
//       const slug = activeMembership?.org_slug;
//       navigate(slug ? orgPath(slug, "/dashboard") : "/dashboard");
//     }
//   }

//   const visibleModes: Mode[] = isSuperAdmin ? ["admin"] : ["app"];

//   return (
//     <div ref={ref} className="relative mb-6">
//       {/* Trigger button */}
//       <button
//         type="button"
//         onClick={() => setOpen((o) => !o)}
//         className={cn(
//           "flex w-full items-center gap-2.5 rounded-xl px-2 py-1.5 text-left transition",
//           "hover:bg-slate-100 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-brand-500"
//         )}
//         aria-haspopup="listbox"
//         aria-expanded={open}
//       >
//         {/* Mode icon pill */}
//         <span className={cn("flex h-7 w-7 shrink-0 items-center justify-center rounded-lg text-white", color)}>
//           <Icon size={14} />
//         </span>

//         <span className="flex flex-1 flex-col leading-none">
//           <span className="text-[11px] font-semibold uppercase tracking-widest text-slate-400">
//             {currentMode === "admin" ? "Platform" : "Workspace"}
//           </span>
//           <span className="text-sm font-semibold text-slate-900">{label}</span>
//         </span>

//         <ChevronDown
//           size={14}
//           className={cn("shrink-0 text-slate-400 transition-transform", open && "rotate-180")}
//         />
//       </button>

//       {/* Dropdown */}
//       {open && (
//         <div
//           role="listbox"
//           className="absolute left-0 top-full z-50 mt-1 w-full overflow-hidden rounded-xl border border-slate-200 bg-white shadow-xl"
//         >
//           {visibleModes.map((mode) => {
//             const meta = MODE_META[mode];
//             const active = mode === currentMode;
//             return (
//               <button
//                 key={mode}
//                 role="option"
//                 aria-selected={active}
//                 type="button"
//                 onClick={() => handleSelect(mode)}
//                 className={cn(
//                   "flex w-full items-center gap-3 px-3 py-2.5 text-left transition",
//                   active ? "bg-slate-50" : "hover:bg-slate-50"
//                 )}
//               >
//                 <span
//                   className={cn(
//                     "flex h-7 w-7 shrink-0 items-center justify-center rounded-lg text-white",
//                     meta.color
//                   )}
//                 >
//                   <meta.Icon size={14} />
//                 </span>
//                 <span className="flex flex-col">
//                   <span className="text-sm font-semibold text-slate-900">{meta.label}</span>
//                   <span className="text-xs text-slate-400">{meta.description}</span>
//                 </span>
//                 {active && (
//                   <span className="ml-auto h-1.5 w-1.5 rounded-full bg-brand-600" />
//                 )}
//               </button>
//             );
//           })}
//         </div>
//       )}
//     </div>
//   );
// }
