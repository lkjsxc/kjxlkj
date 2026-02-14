// API client per /docs/spec/api/http.md
// All fetch calls go through this module for uniform error handling.

export interface ErrorBody {
  code: string;
  message: string;
  details?: unknown;
  request_id: string;
}

export class ApiError extends Error {
  constructor(
    public readonly status: number,
    public readonly body: ErrorBody,
  ) {
    super(body.message);
    this.name = 'ApiError';
  }
}

let csrfToken = '';

export function setCsrf(token: string): void {
  csrfToken = token;
}

async function request<T>(
  method: string,
  path: string,
  body?: unknown,
): Promise<T> {
  const headers: Record<string, string> = {
    'Content-Type': 'application/json',
  };
  // Attach CSRF token for mutating methods per /docs/spec/security/csrf.md
  if (['POST', 'PUT', 'PATCH', 'DELETE'].includes(method) && csrfToken) {
    headers['X-CSRF-Token'] = csrfToken;
  }
  const resp = await fetch(path, {
    method,
    headers,
    body: body != null ? JSON.stringify(body) : undefined,
    credentials: 'same-origin',
  });
  if (!resp.ok) {
    const err: ErrorBody = await resp.json().catch(() => ({
      code: 'UNKNOWN',
      message: resp.statusText,
      request_id: '',
    }));
    throw new ApiError(resp.status, err);
  }
  if (resp.status === 204) return undefined as T;
  return resp.json() as Promise<T>;
}

export const api = {
  get: <T>(path: string) => request<T>('GET', path),
  post: <T>(path: string, body?: unknown) => request<T>('POST', path, body),
  put: <T>(path: string, body?: unknown) => request<T>('PUT', path, body),
  patch: <T>(path: string, body?: unknown) => request<T>('PATCH', path, body),
  del: <T>(path: string) => request<T>('DELETE', path),
};
