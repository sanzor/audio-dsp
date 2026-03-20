import { Outlet } from "react-router-dom";
import AccountSidebar from "./AccountSidebar";
import { UserMenu } from "../UserMenu";

export default function AccountShell() {
  return (
    <div className="flex min-h-screen">
      <AccountSidebar />
      <div className="flex min-w-0 flex-1 flex-col">
        <header className="relative z-10 flex h-14 flex-shrink-0 items-center justify-end border-b border-slate-200 bg-white/70 px-6 backdrop-blur">
          <UserMenu />
        </header>
        <main className="flex-1 overflow-auto px-8 py-8">
          <Outlet />
        </main>
      </div>
    </div>
  );
}
