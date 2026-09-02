import { useMutation } from "@tanstack/react-query";
import { login } from "../../Services/auth/authService";
import { bootstrap } from "../../Services/me/meService";
import { useAuthStore } from "../../Stores/authStore";
import { useProjectStore } from "../../Stores/projectStore";

export function useLogin() {
  const setSession = useAuthStore((state) => state.setSession);
  const setProjects = useProjectStore((state) => state.setProjects);
  const setActiveProject = useProjectStore((state) => state.setActiveProject);
  const clearProject = useProjectStore((state) => state.clearProject);

  const loginMutation = useMutation({
    mutationFn: login,
    onSuccess: async (result) => {
      setSession(result.user, result.token);
      clearProject();

      const { workspaces } = await bootstrap();
      const projects = workspaces.map((workspace) => ({
        project_id: workspace.workspace_id,
        name: workspace.name,
        role: workspace.role,
      }));

      setProjects(projects);

      if (projects.length > 0) {
        setActiveProject(projects[0]);
      }
    },
  });

  return {
    signIn: loginMutation.mutateAsync,
    isSigningIn: loginMutation.isPending,
    loginError: loginMutation.error,
  };
}
