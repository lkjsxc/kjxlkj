/**
 * Auth API calls.
 * Spec: GET /api/auth/session MAY return 401 before login —
 *       this MUST be treated as expected unauthenticated state.
 */
import type { Session } from "../types";
import { get, post, ApiClientError } from "./client";

/** Probe current session. Returns null if unauthenticated (401). */
export async function getSession(): Promise<Session | null> {
  try {
    return await get<Session>("/auth/session");
  } catch (err) {
    if (err instanceof ApiClientError && err.status === 401) {
      return null;
    }
    throw err;
  }
}

/** Check if setup is available. Returns true if 200, false if 409. */
export async function checkSetup(): Promise<boolean> {
  try {
    await get<unknown>("/setup/register");
    return true;
  } catch (err) {
    if (err instanceof ApiClientError && err.status === 409) {
      return false;
    }
    // Treat 405/404 as setup locked
    if (err instanceof ApiClientError && err.status >= 400) {
      return false;
    }
    throw err;
  }
}

export interface RegisterPayload {
  username: string;
  password: string;
}

export async function register(payload: RegisterPayload): Promise<Session> {
  return post<Session>("/setup/register", payload);
}

export interface LoginPayload {
  username: string;
  password: string;
}

export async function login(payload: LoginPayload): Promise<Session> {
  return post<Session>("/auth/login", payload);
}

export async function logout(): Promise<void> {
  await post<void>("/auth/logout");
}
