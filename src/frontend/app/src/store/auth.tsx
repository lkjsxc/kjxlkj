/**
 * Auth store: session state, setup-lock detection.
 * Spec: UX-AUTH-01, UX-AUTH-02, UX-AUTH-03.
 */
import {
  createContext,
  useReducer,
  useContext,
  useEffect,
  type ReactNode,
} from "react";
import type { Session } from "../types";
import { getSession, checkSetup } from "../api/auth";

type AuthPhase = "loading" | "setup" | "login" | "authenticated";

interface AuthState {
  phase: AuthPhase;
  session: Session | null;
  setupAvailable: boolean;
}

type AuthAction =
  | { type: "loaded"; session: Session | null; setupAvailable: boolean }
  | { type: "authenticated"; session: Session }
  | { type: "logged_out" };

function reducer(_state: AuthState, action: AuthAction): AuthState {
  switch (action.type) {
    case "loaded": {
      if (action.session) {
        return {
          phase: "authenticated",
          session: action.session,
          setupAvailable: false,
        };
      }
      return {
        phase: action.setupAvailable ? "setup" : "login",
        session: null,
        setupAvailable: action.setupAvailable,
      };
    }
    case "authenticated":
      return { phase: "authenticated", session: action.session, setupAvailable: false };
    case "logged_out":
      return { phase: "login", session: null, setupAvailable: false };
  }
}

const initial: AuthState = {
  phase: "loading",
  session: null,
  setupAvailable: false,
};

type AuthDispatch = React.Dispatch<AuthAction>;

const AuthStateCtx = createContext<AuthState>(initial);
const AuthDispatchCtx = createContext<AuthDispatch>(() => {});

export function AuthProvider({ children }: { children: ReactNode }) {
  const [state, dispatch] = useReducer(reducer, initial);

  useEffect(() => {
    void (async () => {
      const session = await getSession();
      const setupAvailable = session ? false : await checkSetup();
      dispatch({ type: "loaded", session, setupAvailable });
    })();
  }, []);

  return (
    <AuthStateCtx.Provider value={state}>
      <AuthDispatchCtx.Provider value={dispatch}>
        {children}
      </AuthDispatchCtx.Provider>
    </AuthStateCtx.Provider>
  );
}

export function useAuthState(): AuthState {
  return useContext(AuthStateCtx);
}

export function useAuthDispatch(): AuthDispatch {
  return useContext(AuthDispatchCtx);
}
