import { Music2 } from "lucide-react";
import { useProjectStore, type ActiveProject } from "@/Stores/projectStore";

export function ProjectsPanel() {
  const { projects, activeProject, setActiveProject } = useProjectStore();

  return (
    <>
      <div className="panel-header">Projects</div>
      <div className="panel-content">
        {projects.map((project) => (
          <ProjectCard
            key={project.project_id}
            project={project}
            isActive={activeProject?.project_id === project.project_id}
            onClick={() => setActiveProject(project)}
          />
        ))}
        {projects.length === 0 && (
          <p className="text-sm" style={{ color: "var(--text-muted)" }}>
            No projects yet.
          </p>
        )}
      </div>
    </>
  );
}

function ProjectCard({
  project,
  isActive,
  onClick,
}: {
  project: ActiveProject;
  isActive: boolean;
  onClick: () => void;
}) {
  return (
    <div className={`project-card${isActive ? " active" : ""}`} onClick={onClick}>
      <div
        className="w-10 h-10 rounded flex items-center justify-center flex-shrink-0"
        style={{ backgroundColor: isActive ? "rgba(255,255,255,0.2)" : "rgba(255,255,255,0.05)" }}
      >
        <Music2 className="w-5 h-5" style={{ color: isActive ? "#fff" : "var(--text-muted)" }} />
      </div>
      <div className="min-w-0">
        <div
          className="font-medium truncate"
          style={{ color: isActive ? "#fff" : "var(--text-main)" }}
        >
          {project.name}
        </div>
        <div
          className="text-xs flex items-center gap-1 mt-1"
          style={{ color: isActive ? "rgba(255,255,255,0.7)" : "var(--text-muted)" }}
        >
          {isActive ? (
            <>
              <span className="w-2 h-2 rounded-full bg-green-400 inline-block" />
              Active
            </>
          ) : (
            <span className="capitalize">{project.role}</span>
          )}
        </div>
      </div>
    </div>
  );
}
