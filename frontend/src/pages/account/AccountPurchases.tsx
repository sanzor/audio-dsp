import { Loader2 } from "lucide-react";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { usePurchases } from "@/hooks/purchasedProducts/usePurchases";

function formatDate(iso: string) {
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

export default function AccountPurchases() {
  const { data: purchases, isLoading, isError } = usePurchases();

  return (
    <section className="mx-auto flex max-w-4xl flex-col gap-8">
      <header className="space-y-1">
        <p className="text-xs font-semibold uppercase tracking-[0.3em] text-slate-400">Account</p>
        <h1 className="text-3xl font-semibold text-slate-900">Purchases</h1>
      </header>

      <Card>
        <CardHeader>
          <CardTitle>Purchase history</CardTitle>
        </CardHeader>
        <CardContent>
          {isLoading && (
            <div className="flex items-center gap-2 text-sm text-slate-500">
              <Loader2 className="h-4 w-4 animate-spin" />
              Loading…
            </div>
          )}
          {isError && <p className="text-sm text-red-600">Failed to load purchases.</p>}
          {!isLoading && !isError && (
            <table className="w-full text-sm">
              <thead>
                <tr className="border-b border-slate-100 text-left text-xs font-semibold uppercase tracking-wider text-slate-400">
                  <th className="pb-2 pr-4">ID</th>
                  <th className="pb-2 pr-4">Product ID</th>
                  <th className="pb-2">Purchased at</th>
                </tr>
              </thead>
              <tbody className="divide-y divide-slate-50">
                {(purchases ?? []).map((p) => (
                  <tr key={p.id}>
                    <td className="py-2.5 pr-4 text-slate-400">{p.id}</td>
                    <td className="py-2.5 pr-4 text-slate-700">{p.product_id}</td>
                    <td className="py-2.5 text-slate-500">{formatDate(p.purchased_at)}</td>
                  </tr>
                ))}
                {(purchases ?? []).length === 0 && (
                  <tr>
                    <td colSpan={3} className="py-8 text-center text-sm text-slate-400">
                      No purchases yet.
                    </td>
                  </tr>
                )}
              </tbody>
            </table>
          )}
        </CardContent>
      </Card>
    </section>
  );
}
