import { Loader2 } from "lucide-react";
import { useProducts } from "@/hooks/products/useProducts";

function formatPrice(cents: number, currency: string) {
  if (cents === 0) return "Free";
  return new Intl.NumberFormat("en-US", {
    style: "currency",
    currency: currency.toUpperCase(),
    minimumFractionDigits: 0,
  }).format(cents / 100);
}

export default function AccountShop() {
  const { data: products, isLoading, isError } = useProducts(true);

  return (
    <section className="mx-auto flex max-w-4xl flex-col gap-8">
      <header className="space-y-1">
        <p className="text-xs font-semibold uppercase tracking-[0.3em] text-slate-400">Account</p>
        <h1 className="text-3xl font-semibold text-slate-900">Shop</h1>
        <p className="text-slate-500">Expand your capacity with a project pack.</p>
      </header>

      {isLoading && (
        <div className="flex items-center gap-2 text-sm text-slate-500">
          <Loader2 className="h-4 w-4 animate-spin" />
          Loading…
        </div>
      )}

      {isError && (
        <p className="text-sm text-red-600">Failed to load products. Please try again.</p>
      )}

      {!isLoading && !isError && (products ?? []).length === 0 && (
        <p className="text-sm text-slate-400">No products available right now.</p>
      )}

      {!isLoading && !isError && (products ?? []).length > 0 && (
        <div className="grid gap-5 sm:grid-cols-2 lg:grid-cols-3">
          {(products ?? []).map((product) => (
            <div
              key={product.id}
              className="flex flex-col rounded-2xl border border-slate-200 bg-white p-6 shadow-sm transition hover:shadow-md"
            >
              <span className="mb-1 text-xs font-semibold uppercase tracking-wider text-slate-400">
                {product.tier}
              </span>
              <h3 className="mb-1 text-lg font-semibold text-slate-900">{product.name}</h3>
              {product.description && (
                <p className="mb-4 flex-1 text-sm text-slate-500">{product.description}</p>
              )}
              {!product.description && <div className="flex-1" />}
              <div className="mb-5 text-2xl font-semibold text-slate-900">
                {formatPrice(product.price_cents, product.currency)}
              </div>
              <button
                type="button"
                disabled
                className="w-full cursor-not-allowed rounded-xl bg-slate-900 py-2.5 text-sm font-semibold text-white opacity-50"
              >
                Buy now
              </button>
            </div>
          ))}
        </div>
      )}
    </section>
  );
}
