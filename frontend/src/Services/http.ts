import { useAuthStore } from "../Stores/authStore";
import { useProjectStore } from "../Stores/projectStore";

export const API_BASE_URL = import.meta.env.VITE_API_BASE_URL || "http://localhost:3080";

export function projectApiPath(path: string): string {
  const projectId = useProjectStore.getState().activeProject?.project_id;
  if (projectId == null) throw new Error("An active project is required for this request.");
  return `/v1/projects/${projectId}${path}`;
}

type HttpMethod = "GET" | "POST" | "PUT" | "PATCH" | "DELETE";

type RequestOptions<TInput> = {
  method: HttpMethod;
  path: string;
  body?: TInput;
};

async function request<TOutput, TInput = undefined>({
  method,
  path,
  body,
}: RequestOptions<TInput>): Promise<TOutput> {
  const token = useAuthStore.getState().token ?? undefined;
  const activeProjectId = useProjectStore.getState().activeProject?.project_id;

  const response = await fetch(`${API_BASE_URL}${path}`, {
    method,
    headers: {
      "Content-Type": "application/json",
      ...(token ? { Authorization: `Bearer ${token}` } : {}),
      ...(activeProjectId != null ? { "X-Project-Id": String(activeProjectId) } : {}),
    },
    body: body ? JSON.stringify(body) : undefined,
  });

  if (response.status === 401) {
    useAuthStore.getState().clearSession();
    useProjectStore.getState().clearProject();
    window.location.href = "/login";
    throw new Error("Session expired. Please log in again.");
  }

  if (!response.ok) {
    const message = await response.text();
    throw new Error(message || `Request failed: ${response.status}`);
  }

  if (response.status === 204) {
    return undefined as TOutput;
  }

  return (await response.json()) as TOutput;
}

export const http = {
  get<TOutput>(path: string) {
    return request<TOutput>({ method: "GET", path });
  },
  post<TOutput, TInput>(path: string, body: TInput) {
    return request<TOutput, TInput>({ method: "POST", path, body });
  },
  put<TOutput, TInput>(path: string, body: TInput) {
    return request<TOutput, TInput>({ method: "PUT", path, body });
  },
  patch<TOutput, TInput>(path: string, body: TInput) {
    return request<TOutput, TInput>({ method: "PATCH", path, body });
  },
  delete<TOutput>(path: string) {
    return request<TOutput>({ method: "DELETE", path });
  },
};
