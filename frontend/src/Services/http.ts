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

function buildHeaders(extra?: HeadersInit): Headers {
  const headers = new Headers(extra);
  const token = useAuthStore.getState().token;
  const activeProjectId = useProjectStore.getState().activeProject?.project_id;

  if (token && !headers.has("Authorization")) {
    headers.set("Authorization", `Bearer ${token}`);
  }

  if (activeProjectId != null && !headers.has("X-Project-ID")) {
    headers.set("X-Project-ID", String(activeProjectId));
  }

  return headers;
}

function clearClientSession() {
  useAuthStore.getState().clearSession();
  useProjectStore.getState().clearProject();
}

async function refreshSession(): Promise<boolean> {
  const refreshUrl = joinUrl(import.meta.env.VITE_API_BASE_URL ?? "", "/auth/refresh");
  const res = await fetch(refreshUrl, {
    method: "POST",
    credentials: "include",
  });

  if (!res.ok) {
    return false;
  }

  const body = await readBody(res);
  const nextToken =
    body && typeof body === "object" && "token" in body && typeof body.token === "string"
      ? body.token
      : null;

  if (!nextToken) {
    return false;
  }

  useAuthStore.getState().setToken(nextToken);
  return true;
}

function redirectToLogin() {
  if (window.location.pathname !== "/login") {
    window.location.href = "/login";
  }
}

async function authFetch(path: string, init?: RequestInit, allowRetry = true): Promise<Response> {
  const url = path;
  const requestInit: RequestInit = {
    ...init,
    credentials: "include",
    headers: buildHeaders(init?.headers),
  };

  let res = await fetch(url, requestInit);

  if (res.status === 401 && allowRetry && !url.endsWith("/auth/refresh")) {
    const refreshed = await refreshSession();
    if (refreshed) {
      res = await fetch(url, {
        ...init,
        credentials: "include",
        headers: buildHeaders(init?.headers),
      });
    }
  }

  if (res.status === 401) {
    clearClientSession();
    redirectToLogin();
    throw new HttpError({ status: 401, url, body: null, message: "Session expired" });
  }

  return res;
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
  const res = await authFetch(path, { ...init, method });

  const body = await readBody(res);
  if (!res.ok) {
    throw new HttpError({
      status: res.status,
      url: path,
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

export { authFetch };
