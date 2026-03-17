import { useMutation } from "@tanstack/react-query";
import { useNavigate } from "react-router-dom";
import { login } from "../../Services/auth/authService";
import { bootstrap } from "../../Services/me/meService";
import { useAuthStore } from "../../Stores/authStore";
import { useProjectStore } from "../../Stores/projectStore";

export function useLogin() {
  const setSession = useAuthStore((state) => state.setSession);
  const setProjects = useProjectStore((state) => state.setProjects);
  const setActiveProject = useProjectStore((state) => state.setActiveProject);
  const clearProject = useProjectStore((state) => state.clearProject);
  const navigate = useNavigate();

  const loginMutation = useMutation({
    mutationFn: login,
    onSuccess: async (result) => {
      setSession(result.user);
      clearProject();

      const { user, projects } = await bootstrap();

      if (user.is_admin) {
        navigate("/admin");
        return;
      }

      setProjects(projects);

      // Auto-select the first project — user can switch from the app shell
      if (projects.length > 0) {
        setActiveProject(projects[0]);
      }

      navigate("/dashboard");
    },
  });

  return {
    signIn: loginMutation.mutateAsync,
    isSigningIn: loginMutation.isPending,
    loginError: loginMutation.error,
  };
}
