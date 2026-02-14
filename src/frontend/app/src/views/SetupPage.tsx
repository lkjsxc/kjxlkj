// UX-AUTH-02: setup page shown ONLY when setup is available.
// First-run owner registration form.

import { type FormEvent, useState } from 'react';
import { api, ApiError, setCsrf } from '../api';
import type { SessionInfo } from '../types';

interface Props {
  onComplete: () => void;
}

export default function SetupPage({ onComplete }: Props) {
  const [email, setEmail] = useState('');
  const [displayName, setDisplayName] = useState('');
  const [password, setPassword] = useState('');
  const [error, setError] = useState('');
  const [submitting, setSubmitting] = useState(false);

  async function handleSubmit(e: FormEvent) {
    e.preventDefault();
    setError('');
    setSubmitting(true);
    try {
      await api.post('/api/setup/register', {
        email,
        display_name: displayName,
        password,
      });
      // Auto-login after setup
      const info = await api.post<SessionInfo>('/api/auth/login', {
        email,
        password,
      });
      setCsrf(info.csrf_token);
      onComplete();
    } catch (err) {
      if (err instanceof ApiError) {
        if (err.status === 409) {
          setError('Setup already completed. Redirecting to login.');
          setTimeout(onComplete, 1500);
          return;
        }
        setError(err.body.message);
      } else {
        setError('Network error. Please try again.');
      }
    } finally {
      setSubmitting(false);
    }
  }

  return (
    <div className="auth-page">
      <form className="auth-form" onSubmit={handleSubmit}>
        <h1>Welcome to kjxlkj</h1>
        <p className="auth-subtitle">Create your owner account</p>

        {error && (
          <div className="auth-error" role="alert">
            {error}
          </div>
        )}

        <label htmlFor="setup-email">Email</label>
        <input
          id="setup-email"
          type="email"
          autoComplete="email"
          required
          value={email}
          onChange={(e) => setEmail(e.target.value)}
        />

        <label htmlFor="setup-name">Display name</label>
        <input
          id="setup-name"
          type="text"
          autoComplete="name"
          required
          value={displayName}
          onChange={(e) => setDisplayName(e.target.value)}
        />

        <label htmlFor="setup-password">Password</label>
        <input
          id="setup-password"
          type="password"
          autoComplete="new-password"
          required
          minLength={8}
          value={password}
          onChange={(e) => setPassword(e.target.value)}
        />

        <button type="submit" disabled={submitting}>
          {submitting ? 'Creating…' : 'Create Owner Account'}
        </button>
      </form>
    </div>
  );
}
