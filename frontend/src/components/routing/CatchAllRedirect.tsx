import { Navigate } from "react-router-dom";
import { useAuthStore } from "@/Stores/authStore";
import { getDefaultRoute } from "./routeDefaults";

export default function CatchAllRedirect() {
  const user = useAuthStore((state) => state.user);
  return <Navigate to={getDefaultRoute(user)} replace />;
}
