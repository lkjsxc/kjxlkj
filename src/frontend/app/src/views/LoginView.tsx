/**
 * LoginView: Authenticated session entry.
 * Per /docs/spec/ui/web-app.md: shown when setup is locked.
 */
import { useState, type FormEvent } from "react";
import { login } from "../api";
import { useAppDispatch } from "../state";

export function LoginView() {
  const dispatch = useAppDispatch();
  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");
  const [error, setError] = useState("");

  async function handleSubmit(e: FormEvent) {
    e.preventDefault();
    setError("");
    try {
      const session = await login(email, password);
      dispatch({ type: "SET_SESSION", session });
    } catch (err) {
      setError(err instanceof Error ? err.message : "Login failed");
    }
  }

  return (
    <div className="auth-view">
      <h1>Log In</h1>
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
          Password
          <input
            type="password"
            value={password}
            onChange={(e) => setPassword(e.target.value)}
            required
          />
        </label>
        <button type="submit">Log In</button>
        {error && <p className="error" role="alert">{error}</p>}
      </form>
    </div>
  );
}
