/**
 * Auth API calls.
 * Spec: GET /api/auth/session MAY return 401 before login —
 *       this MUST be treated as expected unauthenticated state.
 */
import type { Session } from "../types";
import { get, post, setCsrfToken, ApiClientError } from "./client";

/** Probe current session. Returns null if unauthenticated (401). */
export async function getSession(): Promise<Session | null> {
  try {
    const session = await get<Session>("/auth/session");
    setCsrfToken(session.csrf_token);
    return session;
  } catch (err) {
    if (err instanceof ApiClientError && err.status === 401) {
      setCsrfToken(null);
      return null;
    }
    throw err;
  }
}

/** Check if setup is available from setup-status probe. */
export async function checkSetup(): Promise<boolean> {
  const res = await get<{ setup_available?: boolean }>("/setup/register");
  return res.setup_available === true;
}

export interface RegisterPayload {
  email: string;
  display_name: string;
  password: string;
}

export interface RegisterResult {
  user_id: string;
  request_id: string;
}

export async function register(payload: RegisterPayload): Promise<RegisterResult> {
  return post<RegisterResult>("/setup/register", payload);
}

export interface LoginPayload {
  email: string;
  password: string;
}

export async function login(payload: LoginPayload): Promise<Session> {
  const session = await post<Session>("/auth/login", payload);
  setCsrfToken(session.csrf_token);
  return session;
}

export async function logout(): Promise<void> {
  await post<void>("/auth/logout");
  setCsrfToken(null);
}
