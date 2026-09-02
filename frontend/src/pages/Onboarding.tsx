import AuthSplitLayout from "@/components/auth/AuthSplitLayout";
import { Button } from "@/components/ui/button";
import { bootstrap, createWorkspace } from "@/Services/me/meService";
import { useAuthStore } from "@/Stores/authStore";
import { useProjectStore } from "@/Stores/projectStore";
import { useState } from "react";
import { useNavigate } from "react-router-dom";

export default function Onboarding() {
  const navigate = useNavigate();
  const user = useAuthStore((state) => state.user);
  const setProjects = useProjectStore((state) => state.setProjects);
  const setActiveProject = useProjectStore((state) => state.setActiveProject);
  const [isCreatingWorkspace, setIsCreatingWorkspace] = useState(false);
  const [error, setError] = useState<string | null>(null);

  async function continueToDashboard() {
    setIsCreatingWorkspace(true);
    setError(null);

    try {
      const workspaceName = user?.name?.trim()
        ? `${user.name.trim()}'s workspace`
        : "My workspace";
      const workspace = await createWorkspace(workspaceName);
      const { workspaces } = await bootstrap();
      const projects = workspaces.map((workspace) => ({
        project_id: workspace.workspace_id,
        name: workspace.name,
        role: workspace.role,
      }));
      const activeProject = projects.find(
        (project) => project.project_id === workspace.workspace_id,
      );

      if (!activeProject) {
        throw new Error("Your workspace was created, but could not be loaded.");
      }

      setProjects(projects);
      setActiveProject(activeProject);
      navigate("/dashboard", { replace: true });
    } catch (err) {
      setError(err instanceof Error ? err.message : "Could not create your workspace.");
    } finally {
      setIsCreatingWorkspace(false);
    }
  }

  return (
    <AuthSplitLayout title="You're all set" subtitle="Your account has been created.">
      <div className="mt-6">
        <Button className="w-full" onClick={continueToDashboard} disabled={isCreatingWorkspace}>
          {isCreatingWorkspace ? "Setting up workspace…" : "Go to dashboard"}
        </Button>
        {error && <p className="mt-3 text-sm text-red-600">{error}</p>}
      </div>
    </AuthSplitLayout>
  );
}
