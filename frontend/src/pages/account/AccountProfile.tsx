import { useAuthStore } from "@/Stores/authStore";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Separator } from "@/components/ui/separator";

export default function AccountProfile() {
  const user = useAuthStore((s) => s.user);

  if (!user) return null;

  return (
    <section className="mx-auto flex max-w-2xl flex-col gap-6">
      <header className="space-y-1">
        <p className="text-xs font-semibold uppercase tracking-[0.3em] text-slate-400">Account</p>
        <h1 className="text-3xl font-semibold text-slate-900">Profile</h1>
      </header>

      <Card>
        <CardHeader>
          <CardTitle>Personal information</CardTitle>
        </CardHeader>
        <CardContent className="space-y-4">
          <Row label="Name" value={user.name} />
          <Separator />
          <Row label="Email" value={user.email} />
          <Separator />
          <Row
            label="Status"
            value={
              <span
                className={`rounded-full px-2 py-0.5 text-xs font-medium ${
                  user.is_verified
                    ? "bg-emerald-100 text-emerald-700"
                    : "bg-amber-100 text-amber-700"
                }`}
              >
                {user.is_verified ? "Verified" : "Unverified"}
              </span>
            }
          />
          {user.is_admin && (
            <>
              <Separator />
              <Row
                label="Role"
                value={
                  <span className="rounded-full bg-violet-100 px-2 py-0.5 text-xs font-medium text-violet-700">
                    Admin
                  </span>
                }
              />
            </>
          )}
        </CardContent>
      </Card>
    </section>
  );
}

function Row({ label, value }: { label: string; value: React.ReactNode }) {
  return (
    <div className="flex items-center justify-between">
      <span className="text-sm font-medium text-slate-500">{label}</span>
      <span className="text-sm text-slate-900">{value}</span>
    </div>
  );
}
