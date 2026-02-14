// Auth state management per UX-AUTH-01, UX-AUTH-02, UX-AUTH-03.
// GET /api/auth/session 401 = expected unauthenticated state.
// 409 from setup probe = setup locked, show login only.

import { useCallback, useEffect, useState } from 'react';
import { api, ApiError, setCsrf } from '../api';
import type { SessionInfo } from '../types';

export type AuthPhase =
  | 'loading'
  | 'setup'      // first-run owner registration available
  | 'login'      // setup locked, need login
  | 'authenticated';

export interface AuthState {
  phase: AuthPhase;
  session: SessionInfo | null;
  error: string | null;
}

export function useAuth() {
  const [state, setState] = useState<AuthState>({
    phase: 'loading',
    session: null,
    error: null,
  });

  const probe = useCallback(async () => {
    setState((s) => ({ ...s, phase: 'loading', error: null }));
    try {
      const info = await api.get<SessionInfo>('/api/auth/session');
      setCsrf(info.csrf_token);
      setState({ phase: 'authenticated', session: info, error: null });
    } catch (err) {
      if (err instanceof ApiError && err.status === 401) {
        // UX-AUTH-01: expected unauthenticated state
        // Now check whether setup is available
        try {
          await api.get('/api/setup/status');
          setState({ phase: 'setup', session: null, error: null });
        } catch (setupErr) {
          if (setupErr instanceof ApiError && setupErr.status === 409) {
            // UX-AUTH-02: setup locked → login only
            setState({ phase: 'login', session: null, error: null });
          } else {
            // If setup status endpoint doesn't exist yet, default to login
            setState({ phase: 'login', session: null, error: null });
          }
        }
      } else {
        setState({
          phase: 'login',
          session: null,
          error: err instanceof Error ? err.message : 'Unknown error',
        });
      }
    }
  }, []);

  useEffect(() => {
    void probe();
  }, [probe]);

  const refresh = useCallback(() => {
    void probe();
  }, [probe]);

  return { ...state, refresh };
}
