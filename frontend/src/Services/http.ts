import { useAuthStore } from "../Stores/authStore";
import { useProjectStore } from "../Stores/projectStore";

type HttpMethod = "GET" | "POST" | "PUT" | "PATCH" | "DELETE";

export class HttpError extends Error {
  status: number;
  url: string;
  body: unknown;

  constructor(opts: { status: number; url: string; body: unknown; message?: string }) {
    super(opts.message ?? `HTTP ${opts.status}`);
    this.name = "HttpError";
    this.status = opts.status;
    this.url = opts.url;
    this.body = opts.body;
  }
}

function joinUrl(base: string, path: string) {
  if (!base) return path;
  if (/^https?:\/\//i.test(path)) return path;
  const b = base.endsWith("/") ? base.slice(0, -1) : base;
  const p = path.startsWith("/") ? path : `/${path}`;
  return `${b}${p}`;
}

async function readBody(res: Response): Promise<unknown> {
  const ct = res.headers.get("content-type") ?? "";
  if (res.status === 204) return undefined;
  const text = await res.text();
  if (!text) return undefined;
  if (ct.includes("application/json")) {
    try { return JSON.parse(text); } catch { return text; }
  }
  return text;
}

async function request<TRes>(
  method: HttpMethod,
  path: string,
  init?: RequestInit
): Promise<TRes> {
  const url = joinUrl(import.meta.env.VITE_API_BASE_URL ?? "", path);

  const activeProjectId = useProjectStore.getState().activeProject?.project_id;
  const projectHeader: HeadersInit = activeProjectId
    ? { "X-Project-ID": activeProjectId }
    : {};

  const res = await fetch(url, {
    ...init,
    method,
    credentials: "include",
    headers: { ...(init?.headers ?? {}), ...projectHeader },
  });

  if (res.status === 401) {
    useAuthStore.getState().clearSession();
    useProjectStore.getState().clearProject();
    window.location.href = "/login";
    throw new HttpError({ status: 401, url, body: null, message: "Session expired" });
  }

  const body = await readBody(res);
  if (!res.ok) {
    throw new HttpError({
      status: res.status,
      url,
      body,
      message: typeof body === "string" && body ? body : `Request failed (${res.status})`,
    });
  }
  return body as TRes;
}

function jsonHeaders(extra?: HeadersInit): HeadersInit {
  return { "Content-Type": "application/json", ...extra };
}

export const http = {
  get<TRes>(path: string, init?: RequestInit) {
    return request<TRes>("GET", path, init);
  },
  delete<TRes>(path: string, init?: RequestInit) {
    return request<TRes>("DELETE", path, init);
  },
  post<TRes, TBody>(path: string, body: TBody, init?: RequestInit) {
    return request<TRes>("POST", path, {
      ...init,
      headers: jsonHeaders(init?.headers),
      body: JSON.stringify(body),
    });
  },
  put<TRes, TBody>(path: string, body: TBody, init?: RequestInit) {
    return request<TRes>("PUT", path, {
      ...init,
      headers: jsonHeaders(init?.headers),
      body: JSON.stringify(body),
    });
  },
  patch<TRes, TBody>(path: string, body: TBody, init?: RequestInit) {
    return request<TRes>("PATCH", path, {
      ...init,
      headers: jsonHeaders(init?.headers),
      body: JSON.stringify(body),
    });
  },
};
