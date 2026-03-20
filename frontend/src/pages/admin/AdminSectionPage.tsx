import { useLocation } from "react-router-dom";

type AdminSectionPageProps = {
  title: string;
  permission?: string;
};

export default function AdminSectionPage({
  title,
  permission,
}: AdminSectionPageProps) {
  const location = useLocation();

  return (
    <section className="mx-auto flex max-w-4xl flex-col gap-6">
      <header className="space-y-2">
        <p className="text-xs font-semibold uppercase tracking-[0.3em] text-slate-400">
          Platform Admin
        </p>
        <h1 className="text-3xl font-semibold text-slate-900">{title}</h1>
        <p className="max-w-2xl text-sm text-slate-600">
          This route is now registered in the consolidated router. Replace this
          placeholder with the real {title.toLowerCase()} admin screen when that
          feature is ready.
        </p>
      </header>

      <div className="rounded-2xl border border-slate-200 bg-white p-6 shadow-sm">
        <dl className="grid gap-4 text-sm text-slate-600 sm:grid-cols-2">
          <div>
            <dt className="font-medium text-slate-900">Route</dt>
            <dd>{location.pathname}</dd>
          </div>
          <div>
            <dt className="font-medium text-slate-900">Permission</dt>
            <dd>{permission ?? "None declared"}</dd>
          </div>
        </dl>
      </div>
    </section>
  );
}
