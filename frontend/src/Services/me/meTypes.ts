import type { ProjectRole } from "../../Stores/projectStore";

export type MeUser = {
  id: string;
  email: string;
  name: string;
  is_admin: boolean;
  is_verified: boolean;
};

export type MeProject = {
  project_id: string;
  name: string;
  role: ProjectRole;
};

export type BootstrapResponse = {
  user: MeUser;
  projects: MeProject[];
};
