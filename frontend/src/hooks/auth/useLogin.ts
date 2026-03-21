import { useMutation } from "@tanstack/react-query";
import { useNavigate } from "react-router-dom";
import { login } from "../../Services/auth/authService";
import { bootstrap } from "../../Services/me/meService";
import { useAuthStore } from "../../Stores/authStore";
import { useProjectStore } from "../../Stores/projectStore";
import {
  DEFAULT_APP_PATH,
  DEFAULT_SUPER_ADMIN_PATH,
} from "../../components/layout/app/routes";

export function useLogin() {
  const setSession = useAuthStore((state) => state.setSession);
  const setProjects = useProjectStore((state) => state.setProjects);
  const setActiveProject = useProjectStore((state) => state.setActiveProject);
  const clearProject = useProjectStore((state) => state.clearProject);
  const navigate = useNavigate();

  const loginMutation = useMutation({
    mutationFn: login,
    onSuccess: async (result) => {
      setSession(result.user, result.token);
      clearProject();

      const { user, projects } = await bootstrap();

      if (user.is_admin) {
        navigate(DEFAULT_SUPER_ADMIN_PATH);
        return;
      }

      if (projects.length === 0) {
        navigate("/onboarding");
        return;
      }

      setProjects(projects);

      // Auto-select the first project — user can switch from the app shell
      setActiveProject(projects[0]);

      navigate(DEFAULT_APP_PATH);
    },
  });

  return {
    signIn: loginMutation.mutateAsync,
    isSigningIn: loginMutation.isPending,
    loginError: loginMutation.error,
  };
}
