import AuthSplitLayout from "@/components/auth/AuthSplitLayout";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { useLogin } from "@/hooks/auth/useLogin";
import { useState } from "react";
import { Link } from "react-router-dom";

export default function SignIn() {
  const { signIn, isSigningIn, loginError } = useLogin();
  const [email, setEmail] = useState("test@gmail.com");
  const [password, setPassword] = useState("test");

  async function handleSubmit(e: React.FormEvent) {
    e.preventDefault();
    try {
      await signIn({ email, password_hash: password });
    } catch {
      // The mutation error is rendered from loginError.
    }
  }

  return (
    <AuthSplitLayout title="Welcome back" subtitle="Sign in to your account">
      <form onSubmit={handleSubmit} className="mt-6 flex flex-col gap-4">
        <div className="flex flex-col gap-1.5">
          <Label htmlFor="email">Email</Label>
          <Input
            id="email"
            type="email"
            placeholder="you@example.com"
            value={email}
            onChange={(e) => setEmail(e.target.value)}
            required
          />
        </div>
        <div className="flex flex-col gap-1.5">
          <Label htmlFor="password">Password</Label>
          <Input
            id="password"
            type="password"
            value={password}
            onChange={(e) => setPassword(e.target.value)}
            required
          />
        </div>
        {loginError && (
          <p className="text-sm text-red-600">{loginError.message}</p>
        )}
        <Button type="submit" disabled={isSigningIn} className="w-full">
          {isSigningIn ? "Signing in…" : "Sign in"}
        </Button>
        <p className="text-center text-sm text-slate-500">
          No account?{" "}
          <Link to="/register" className="font-medium text-slate-900 underline underline-offset-4">
            Sign up
          </Link>
        </p>
      </form>
    </AuthSplitLayout>
  );
}
