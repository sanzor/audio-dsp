import type * as React from "react";
import { Check, Loader2, Zap, Shield, Building2, Crown } from "lucide-react";
import { Button } from "@/components/ui/button";
import {
  Card,
  CardContent,
  CardFooter,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { useMySubscriptions, useActiveSubscription } from "@/hooks/subscriptions/useSubscriptions";
import { useTierConfigs } from "@/hooks/tierConfigs/useTierConfigs";
import type { TierConfig } from "@/Services/tierConfigs/tierConfigsService";

// Tier metadata keyed by the PascalCase string the backend sends
const TIER_META: Record<
  string,
  { label: string; icon: React.ReactNode; highlight: boolean; accentClass: string }
> = {
  Free:      { label: "Free",       icon: <Zap className="w-5 h-5" />,       highlight: false, accentClass: "border-border" },
  Standard:  { label: "Standard",   icon: <Shield className="w-5 h-5" />,     highlight: false, accentClass: "border-border" },
  Premium:   { label: "Premium",    icon: <Crown className="w-5 h-5" />,      highlight: true,  accentClass: "border-primary" },
  BankLevel: { label: "Enterprise", icon: <Building2 className="w-5 h-5" />,  highlight: false, accentClass: "border-border" },
};

function formatStorage(bytes: number) {
  if (bytes >= 1_073_741_824) return `${(bytes / 1_073_741_824).toFixed(0)} GB`;
  if (bytes >= 1_048_576)     return `${(bytes / 1_048_576).toFixed(0)} MB`;
  return `${bytes} B`;
}

function tierLabel(tier: string) {
  return TIER_META[tier]?.label ?? tier;
}

function formatDate(iso: string | null) {
  if (!iso) return "—";
  try {
    return new Date(iso).toLocaleDateString("en-US", {
      year: "numeric",
      month: "short",
      day: "numeric",
    });
  } catch {
    return iso;
  }
}

function TierCard({ config, isCurrent }: { config: TierConfig; isCurrent: boolean }) {
  const meta = TIER_META[config.tier] ?? {
    label: config.tier,
    icon: <Zap className="w-5 h-5" />,
    highlight: false,
    accentClass: "border-border",
  };

  return (
    <Card
      className={`relative flex flex-col border-2 transition-shadow ${meta.accentClass} ${
        meta.highlight ? "shadow-lg shadow-primary/10" : ""
      } ${isCurrent ? "ring-2 ring-primary ring-offset-2" : ""}`}
    >
      {meta.highlight && (
        <div className="absolute -top-3 left-1/2 -translate-x-1/2">
          <span className="rounded-full bg-primary px-3 py-1 text-xs font-semibold text-primary-foreground">
            Popular
          </span>
        </div>
      )}

      <CardHeader className="space-y-3 pb-4 pt-8">
        <div
          className={`flex h-10 w-10 items-center justify-center rounded-xl ${
            meta.highlight ? "bg-primary text-primary-foreground" : "bg-muted text-foreground"
          }`}
        >
          {meta.icon}
        </div>
        <CardTitle className="text-xl">{meta.label}</CardTitle>
      </CardHeader>

      <CardContent className="flex-1 space-y-3">
        <ul className="space-y-2">
          <li className="flex items-center gap-2 text-sm text-slate-600">
            <Check className="h-3.5 w-3.5 shrink-0 text-primary" />
            {config.max_projects === -1 ? "Unlimited projects" : `${config.max_projects} projects`}
          </li>
          <li className="flex items-center gap-2 text-sm text-slate-600">
            <Check className="h-3.5 w-3.5 shrink-0 text-primary" />
            {config.max_tracks_per_project === -1
              ? "Unlimited tracks per project"
              : `${config.max_tracks_per_project} tracks / project`}
          </li>
          <li className="flex items-center gap-2 text-sm text-slate-600">
            <Check className="h-3.5 w-3.5 shrink-0 text-primary" />
            {config.max_storage_bytes === -1
              ? "Unlimited storage"
              : `${formatStorage(config.max_storage_bytes)} storage`}
          </li>
        </ul>
      </CardContent>

      <CardFooter className="pt-4">
        {isCurrent ? (
          <Button variant="outline" className="w-full" disabled>
            Current plan
          </Button>
        ) : (
          <Button
            className="w-full"
            variant={meta.highlight ? "default" : "outline"}
            disabled
          >
            {config.tier === "Free" ? "Downgrade" : "Upgrade"}
          </Button>
        )}
      </CardFooter>
    </Card>
  );
}

export default function AccountSubscriptions() {
  const { data: active, isLoading: subLoading } = useActiveSubscription();
  const { data: allSubs } = useMySubscriptions();
  const { data: configs, isLoading: configsLoading } = useTierConfigs();

  // Backend serialises Tier as PascalCase ("Free", "Standard", etc.)
  const currentTier = active?.tier ?? "Free";
  const isLoading = subLoading || configsLoading;

  return (
    <section className="mx-auto flex max-w-4xl flex-col gap-8">
      <header className="space-y-1">
        <p className="text-xs font-semibold uppercase tracking-[0.3em] text-slate-400">Account</p>
        <h1 className="text-3xl font-semibold text-slate-900">Subscriptions</h1>
        <p className="text-slate-500">Choose the plan that fits your work.</p>
      </header>

      {isLoading ? (
        <div className="flex items-center gap-2 text-sm text-slate-500">
          <Loader2 className="h-4 w-4 animate-spin" />
          Loading…
        </div>
      ) : (
        <div className="grid gap-5 sm:grid-cols-2 lg:grid-cols-4">
          {(configs ?? []).map((cfg) => (
            <TierCard key={cfg.tier} config={cfg} isCurrent={currentTier === cfg.tier} />
          ))}
        </div>
      )}

      {allSubs && allSubs.length > 0 && (
        <div className="space-y-3">
          <h2 className="text-sm font-semibold uppercase tracking-wider text-slate-400">
            History
          </h2>
          <div className="overflow-hidden rounded-xl border border-slate-100">
            <table className="w-full text-sm">
              <thead className="bg-slate-50">
                <tr>
                  {["Tier", "Active", "Started", "Expires"].map((h) => (
                    <th
                      key={h}
                      className="px-4 py-2.5 text-left text-xs font-semibold uppercase tracking-wider text-slate-400"
                    >
                      {h}
                    </th>
                  ))}
                </tr>
              </thead>
              <tbody className="divide-y divide-slate-50">
                {allSubs.map((s) => (
                  <tr key={s.id}>
                    <td className="px-4 py-2.5 font-medium text-slate-900">{tierLabel(s.tier)}</td>
                    <td className="px-4 py-2.5">
                      <span
                        className={`rounded-full px-2 py-0.5 text-xs font-medium ${
                          s.is_active
                            ? "bg-emerald-100 text-emerald-700"
                            : "bg-slate-100 text-slate-500"
                        }`}
                      >
                        {s.is_active ? "Active" : "Inactive"}
                      </span>
                    </td>
                    <td className="px-4 py-2.5 text-slate-500">{formatDate(s.started_at)}</td>
                    <td className="px-4 py-2.5 text-slate-500">{formatDate(s.expires_at)}</td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        </div>
      )}
    </section>
  );
}
