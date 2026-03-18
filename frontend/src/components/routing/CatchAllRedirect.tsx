import { Navigate } from "react-router-dom";
import { useAuthStore } from "@/Stores/authStore";

export default function CatchAllRedirect() {
  const user = useAuthStore((state) => state.user);
  return <Navigate to={user ? "/dashboard" : "/login"} replace />;
}
