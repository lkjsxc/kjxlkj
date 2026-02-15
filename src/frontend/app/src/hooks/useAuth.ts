/**
 * Auth hook: login, register, logout actions.
 */
import { useCallback } from "react";
import { useAuthState, useAuthDispatch } from "../store/auth";
import {
  login as apiLogin,
  register as apiRegister,
  logout as apiLogout,
} from "../api/auth";
import type { Session } from "../types";

export function useAuth() {
  const state = useAuthState();
  const dispatch = useAuthDispatch();

  const login = useCallback(
    async (email: string, password: string) => {
      const session: Session = await apiLogin({ email, password });
      dispatch({ type: "authenticated", session });
    },
    [dispatch],
  );

  const register = useCallback(
    async (email: string, displayName: string, password: string) => {
      await apiRegister({ email, display_name: displayName, password });
      const session: Session = await apiLogin({ email, password });
      dispatch({ type: "authenticated", session });
    },
    [dispatch],
  );

  const logout = useCallback(async () => {
    await apiLogout();
    dispatch({ type: "logged_out" });
  }, [dispatch]);

  return { ...state, login, register, logout };
}
