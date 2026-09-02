import type { ProjectRole } from "../../Stores/projectStore";

export type MeUser = {
  id: number;
  email: string;
  name: string;
  is_admin: boolean;
  is_verified: boolean;
};

export type MeWorkspace = {
  workspace_id: number;
  name: string;
  role: ProjectRole;
};

export type BootstrapResponse = {
  user: MeUser;
  workspaces: MeWorkspace[];
};
