// Root application component.
// Deterministic pre-auth routing per UX-AUTH-01/02/03.
// Phase-based rendering: loading → setup | login → authenticated.

import { useAuth } from './hooks/useAuth';
import SetupPage from './views/SetupPage';
import LoginPage from './views/LoginPage';
import NotesLayout from './views/NotesLayout';

export default function App() {
  const { phase, session, error, refresh } = useAuth();

  if (phase === 'loading') {
    return (
      <div className="auth-page">
        <p>Loading…</p>
      </div>
    );
  }

  if (phase === 'setup') {
    return <SetupPage onComplete={refresh} />;
  }

  if (phase === 'login') {
    return (
      <div>
        {error && (
          <div className="auth-error" role="alert">
            {error}
          </div>
        )}
        <LoginPage onSuccess={refresh} />
      </div>
    );
  }

  // phase === 'authenticated'
  return <NotesLayout session={session!} />;
}
