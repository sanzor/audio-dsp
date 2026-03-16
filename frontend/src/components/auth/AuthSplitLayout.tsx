import * as React from "react";

export default function AuthSplitLayout({
  title,
  subtitle,
  children
}: {
  title: string;
  subtitle: string;
  children: React.ReactNode;
}) {
  return (
    <div className="relative min-h-screen overflow-hidden">
      <div className="absolute inset-0 opacity-60">
        <div className="absolute left-[-20%] top-[-10%] h-[520px] w-[520px] rounded-full bg-[radial-gradient(circle,_rgba(56,189,248,0.35)_0%,_rgba(255,255,255,0)_60%)]" />
        <div className="absolute right-[-15%] bottom-[-20%] h-[560px] w-[560px] rounded-full bg-[radial-gradient(circle,_rgba(250,204,21,0.3)_0%,_rgba(255,255,255,0)_65%)]" />
      </div>

      <div className="relative mx-auto grid min-h-screen max-w-6xl grid-cols-1 gap-10 px-6 py-12 lg:grid-cols-[0.85fr_1.15fr]">
        <section className="flex animate-fade-up flex-col justify-between rounded-[32px] border border-white/60 bg-white/70 p-10 shadow-[0_30px_120px_rgba(15,23,42,0.12)] backdrop-blur">
          <div>
            <div className="mb-8 inline-flex items-center gap-3 rounded-full border border-slate-200 bg-white px-4 py-2 text-sm uppercase tracking-[0.2em] text-slate-500">
              Vigul Portal
            </div>
            <h1 className="text-4xl font-semibold text-slate-900 sm:text-5xl">
              Split onboarding for multi-tenant teams.
            </h1>
            <p className="mt-4 text-lg text-slate-600">
              Get the right people into the right organization with minimal friction. Handle self-service,
              invites, and multi-org switching without losing context.
            </p>
          </div>

          <div className="mt-10 flex flex-wrap items-center gap-4 text-sm text-slate-500">
            <span className="rounded-full bg-slate-900 px-3 py-1 text-xs font-semibold uppercase tracking-widest text-white">
              Fast permission checks
            </span>
            <span>Roles → Role Permissions → Memberships</span>
          </div>
        </section>

        <section className="flex flex-col justify-center">
          <div
            className="animate-fade-up rounded-[28px] border border-slate-200 bg-white/90 p-8 shadow-[0_40px_120px_rgba(15,23,42,0.15)]"
            style={{ animationDelay: "140ms" }}
          >
            <div className="flex items-center justify-between">
              <div>
                <h2 className="text-3xl font-semibold text-slate-900">{title}</h2>
                <p className="mt-2 text-sm text-slate-500">{subtitle}</p>
              </div>
            </div>
            {children}
          </div>
        </section>
      </div>
    </div>
  );
}
