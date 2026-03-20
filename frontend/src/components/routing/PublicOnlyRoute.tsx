import { Navigate, Outlet } from "react-router-dom";
import { useAuthStore } from "@/Stores/authStore";
import { getDefaultRoute } from "./routeDefaults";

export default function PublicOnlyRoute() {
  const user = useAuthStore((state) => state.user);

  if (user) {
    return <Navigate to={getDefaultRoute(user)} replace />;
  }

  return <Outlet />;
}
