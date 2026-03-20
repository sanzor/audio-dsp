import type { AuthUser } from "@/Stores/authStore";
import {
  DEFAULT_APP_PATH,
  DEFAULT_SUPER_ADMIN_PATH,
} from "@/components/layout/app/routes";

export function getDefaultRoute(user: AuthUser | null) {
  if (!user) {
    return "/login";
  }

  return user.is_admin ? DEFAULT_SUPER_ADMIN_PATH : DEFAULT_APP_PATH;
}
