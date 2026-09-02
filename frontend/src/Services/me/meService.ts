import { http } from "../http";
import type { BootstrapResponse } from "./meTypes";

type CreateWorkspaceResponse = {
  workspace_id: number;
  name: string;
};

export async function bootstrap(): Promise<BootstrapResponse> {
  return http.get<BootstrapResponse>("/v1/me/bootstrap");
}

export async function createWorkspace(name: string): Promise<CreateWorkspaceResponse> {
  return http.post<CreateWorkspaceResponse, { name: string }>("/v1/me/workspaces", { name });
}
