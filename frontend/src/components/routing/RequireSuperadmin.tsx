import { Navigate, Outlet } from "react-router-dom";
import { useAuthStore } from "@/Stores/authStore";

export default function RequireSuperadmin() {
  const user = useAuthStore((state) => state.user);
  if (!user) return <Navigate to="/login" replace />;
  if (!user.is_admin) return <Navigate to="/dashboard" replace />;
  return <Outlet />;
}
