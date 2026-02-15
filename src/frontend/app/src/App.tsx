/**
 * App: root router shell.
 * Spec: deterministic auth transitions.
 * Spec: UX-AUTH-01 — 401 treated as expected unauthenticated.
 * Spec: UX-AUTH-02 — setup-locked shows login only.
 */
import { useAuthState } from "./store/auth";
import { NotesProvider } from "./store/notes";
import { EditorProvider } from "./store/editor";
import { SetupView } from "./views/SetupView";
import { LoginView } from "./views/LoginView";
import { NotesLayout } from "./views/NotesLayout";

export function App() {
  const { phase } = useAuthState();

  switch (phase) {
    case "loading":
      return <LoadingScreen />;
    case "setup":
      return <SetupView />;
    case "login":
      return <LoginView />;
    case "authenticated":
      return (
        <NotesProvider>
          <EditorProvider>
            <NotesLayout />
          </EditorProvider>
        </NotesProvider>
      );
  }
}

function LoadingScreen() {
  return (
    <div style={styles.loading}>
      <span>Loading…</span>
    </div>
  );
}

const styles: Record<string, React.CSSProperties> = {
  loading: {
    display: "flex",
    alignItems: "center",
    justifyContent: "center",
    height: "100%",
    color: "#999",
  },
};
