import { useEffect } from "react";
import { useNavigate } from "react-router-dom";
import { useAuth } from "@/Auth/UseAuth";

export function AuthCallback() {
  const { user, loading } = useAuth();
  const navigate = useNavigate();

  useEffect(() => {
    if (loading) return; // wait until AuthProvider finishes

    if (user) {
      navigate("/dashboard");
    } else {
      navigate("/login");
    }
  }, [loading, user, navigate]);

  return <div>Logging you in...</div>;
}