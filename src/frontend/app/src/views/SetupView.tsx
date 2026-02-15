/**
 * SetupView: First-run owner registration.
 * Per /docs/spec/ui/web-app.md: shown only while setup is available.
 * If setup is locked (409), UI MUST switch to login-only presentation.
 */
import { useState, type FormEvent } from "react";
import { register, ApiError } from "../api";
import { useAppDispatch } from "../state";

export function SetupView({
  onLocked,
}: {
  onLocked: () => void;
}) {
  const dispatch = useAppDispatch();
  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");
  const [name, setName] = useState("");
  const [error, setError] = useState("");

  async function handleSubmit(e: FormEvent) {
    e.preventDefault();
    setError("");
    try {
      const session = await register(email, password, name);
      dispatch({ type: "SET_SESSION", session });
    } catch (err) {
      if (err instanceof ApiError && err.status === 409) {
        onLocked();
        return;
      }
      setError(err instanceof Error ? err.message : "Registration failed");
    }
  }

  return (
    <div className="auth-view">
      <h1>Welcome — Set Up Owner Account</h1>
      <form onSubmit={handleSubmit}>
        <label>
          Email
          <input
            type="email"
            value={email}
            onChange={(e) => setEmail(e.target.value)}
            required
            autoFocus
          />
        </label>
        <label>
          Display Name
          <input
            type="text"
            value={name}
            onChange={(e) => setName(e.target.value)}
            required
          />
        </label>
        <label>
          Password
          <input
            type="password"
            value={password}
            onChange={(e) => setPassword(e.target.value)}
            required
            minLength={8}
          />
        </label>
        <button type="submit">Create Owner Account</button>
        {error && <p className="error" role="alert">{error}</p>}
      </form>
    </div>
  );
}
