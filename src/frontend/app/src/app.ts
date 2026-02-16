/**
 * Application entry point per /docs/spec/ui/web-app.md
 *
 * Initializes the shell, checks session, and renders the appropriate view.
 * Per /docs/spec/ui/web-app.md: note-first, low-noise.
 */

import { authSession } from './api.js';
import { createInitialState } from './state.js';
import type { AppState } from './state.js';

/** Application shell per /docs/spec/ui/web-app.md */
export class App {
  private state: AppState;

  constructor() {
    this.state = createInitialState();
  }

  /** Initialize application */
  async init(): Promise<void> {
    // Per /docs/spec/ui/web-app.md: pre-auth 401 is expected and non-fatal
    const result = await authSession();
    if (result.ok && result.data.authenticated) {
      this.state = { ...this.state, session: result.data, view: 'notes_list' };
    } else {
      this.state = { ...this.state, view: 'login' };
    }
    this.render();
  }

  /** Render current view */
  private render(): void {
    const root = document.getElementById('app');
    if (!root) return;

    switch (this.state.view) {
      case 'setup':
        root.innerHTML = '<div class="view-setup"><h1>Setup</h1><p>Register first user (owner)</p></div>';
        break;
      case 'login':
        root.innerHTML = '<div class="view-login"><h1>Login</h1><form id="login-form"><input type="text" name="username" placeholder="Username" required /><input type="password" name="password" placeholder="Password" required /><button type="submit">Login</button></form></div>';
        break;
      case 'notes_list':
        root.innerHTML = '<div class="view-notes"><nav class="note-list"></nav><main class="note-editor"></main></div>';
        break;
      case 'note_detail':
        root.innerHTML = '<div class="view-detail"><main class="editor-surface"></main></div>';
        break;
      case 'agent_runs':
        root.innerHTML = '<div class="view-agent"><h2>Agent Runs</h2></div>';
        break;
    }
  }

  /** Get current state (for testing) */
  getState(): Readonly<AppState> {
    return this.state;
  }
}

/** Boot the application */
export function boot(): void {
  const app = new App();
  void app.init();
}
