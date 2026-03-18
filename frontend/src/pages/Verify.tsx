import AuthSplitLayout from "@/components/auth/AuthSplitLayout";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { useVerify } from "@/hooks/auth/useVerify";
import { useState } from "react";
import { useSearchParams } from "react-router-dom";

export default function Verify() {
  const [searchParams] = useSearchParams();
  const { verify, isVerifying, verifyError, verifySuccess } = useVerify();
  const [token, setToken] = useState(searchParams.get("token") ?? "");

  function handleSubmit(e: React.FormEvent) {
    e.preventDefault();
    verify({ token });
  }

  if (verifySuccess) {
    return (
      <AuthSplitLayout title="Email verified" subtitle="Your account is now active.">
        <p className="mt-4 text-sm text-slate-600">
          You can now <a href="/dashboard" className="underline">go to the dashboard</a>.
        </p>
      </AuthSplitLayout>
    );
  }

  return (
    <AuthSplitLayout title="Verify your email" subtitle="Enter the token from your verification email.">
      <form onSubmit={handleSubmit} className="mt-6 flex flex-col gap-4">
        <div className="flex flex-col gap-1.5">
          <Label htmlFor="token">Verification token</Label>
          <Input
            id="token"
            type="text"
            value={token}
            onChange={(e) => setToken(e.target.value)}
            required
          />
        </div>
        {verifyError && (
          <p className="text-sm text-red-600">{verifyError.message}</p>
        )}
        <Button type="submit" disabled={isVerifying} className="w-full">
          {isVerifying ? "Verifying…" : "Verify"}
        </Button>
      </form>
    </AuthSplitLayout>
  );
}
