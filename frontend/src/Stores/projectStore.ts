import { create } from "zustand";
import { persist } from "zustand/middleware";

export type ProjectRole = "owner" | "editor" | "viewer";

export interface ActiveProject {
  project_id: string;
  name: string;
  role: ProjectRole;
}

interface ProjectState {
  projects: ActiveProject[];
  activeProject: ActiveProject | null;
  setProjects: (projects: ActiveProject[]) => void;
  setActiveProject: (project: ActiveProject) => void;
  clearProject: () => void;
}

export const useProjectStore = create<ProjectState>()(
  persist(
    (set) => ({
      projects: [],
      activeProject: null,
      setProjects: (projects) => set({ projects }),
      setActiveProject: (project) => set({ activeProject: project }),
      clearProject: () => set({ activeProject: null, projects: [] }),
    }),
    { name: "audio-dsp-project" }
  )
);
