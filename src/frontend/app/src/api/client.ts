/**
 * HTTP client wrapper for the kjxlkj API.
 * All requests go to same-origin /api prefix.
 */
import type { ApiError } from "../types";

export class ApiClientError extends Error {
  constructor(
    public status: number,
    public body: ApiError,
  ) {
    super(body.message);
    this.name = "ApiClientError";
  }
}

/** CSRF token cached from session response. */
let csrfToken: string | null = null;

/** Set CSRF token for mutating requests. */
export function setCsrfToken(token: string | null): void {
  csrfToken = token;
}

async function request<T>(
  method: string,
  path: string,
  body?: unknown,
): Promise<T> {
  const headers: Record<string, string> = {
    "Content-Type": "application/json",
  };
  // Per /docs/spec/security/csrf.md: include x-csrf-token on
  // mutating methods.
  if (csrfToken && method !== "GET" && method !== "HEAD") {
    headers["x-csrf-token"] = csrfToken;
  }
  const opts: RequestInit = {
    method,
    credentials: "same-origin",
    headers,
  };
  if (body !== undefined) {
    opts.body = JSON.stringify(body);
  }
  const res = await fetch(`/api${path}`, opts);
  if (res.status === 204) return undefined as T;
  const json = (await res.json()) as T | ApiError;
  if (!res.ok) {
    throw new ApiClientError(res.status, json as ApiError);
  }
  return json as T;
}

export function get<T>(path: string): Promise<T> {
  return request<T>("GET", path);
}

export function post<T>(path: string, body?: unknown): Promise<T> {
  return request<T>("POST", path, body);
}

export function put<T>(path: string, body?: unknown): Promise<T> {
  return request<T>("PUT", path, body);
}

export function patch<T>(path: string, body?: unknown): Promise<T> {
  return request<T>("PATCH", path, body);
}

export function del<T>(path: string): Promise<T> {
  return request<T>("DELETE", path);
}
