import AuthSplitLayout from "@/components/auth/AuthSplitLayout";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { useRegister } from "@/hooks/auth/useRegister";
import { useState } from "react";
import { Link, useNavigate } from "react-router-dom";

export default function SignUp() {
  const { signUp, isSigningUp, registerError, isRegistered } = useRegister();
  const navigate = useNavigate();
  const [name, setName] = useState("");
  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");

  if (isRegistered) {
    navigate("/onboarding");
  }

  function handleSubmit(e: React.FormEvent) {
    e.preventDefault();
    signUp({ name, email, password_hash: password });
  }

  return (
    <AuthSplitLayout title="Create an account" subtitle="Get started for free">
      <form onSubmit={handleSubmit} className="mt-6 flex flex-col gap-4">
        <div className="flex flex-col gap-1.5">
          <Label htmlFor="name">Name</Label>
          <Input
            id="name"
            type="text"
            placeholder="Your name"
            value={name}
            onChange={(e) => setName(e.target.value)}
            required
          />
        </div>
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
        {registerError && (
          <p className="text-sm text-red-600">{registerError.message}</p>
        )}
        <Button type="submit" disabled={isSigningUp} className="w-full">
          {isSigningUp ? "Creating account…" : "Sign up"}
        </Button>
        <p className="text-center text-sm text-slate-500">
          Already have an account?{" "}
          <Link to="/login" className="font-medium text-slate-900 underline underline-offset-4">
            Sign in
          </Link>
        </p>
      </form>
    </AuthSplitLayout>
  );
}
