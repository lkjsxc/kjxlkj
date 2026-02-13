// kjxlkj — Authentication UI module
// UX-AUTH-01, UX-AUTH-02, UX-AUTH-03
'use strict';

const AuthUI = {
  render(state) {
    return `
    <div class="auth-screen">
      <div class="card">
        <h2>kjxlkj</h2>
        <div id="auth-error" class="error"></div>
        <input id="auth-user" type="text" placeholder="Username" aria-label="Username" autocomplete="username">
        <input id="auth-pass" type="password" placeholder="Password" aria-label="Password" autocomplete="current-password">
        <button id="auth-login-btn">Login</button>
        <button id="auth-setup-btn" style="background:var(--bg3)">First-time Setup</button>
      </div>
    </div>`;
  },

  bind() {
    const loginBtn = document.getElementById('auth-login-btn');
    const setupBtn = document.getElementById('auth-setup-btn');
    if (loginBtn) loginBtn.onclick = () => this.doLogin();
    if (setupBtn) setupBtn.onclick = () => this.doSetup();
    // Enter key submits
    const passInput = document.getElementById('auth-pass');
    if (passInput) passInput.onkeydown = (e) => { if (e.key === 'Enter') this.doLogin(); };
  },

  async doLogin() {
    const user = document.getElementById('auth-user').value;
    const pass = document.getElementById('auth-pass').value;
    const errEl = document.getElementById('auth-error');
    try {
      const r = await fetch('/api/auth/login', {
        method: 'POST', headers: {'Content-Type':'application/json'},
        body: JSON.stringify({username: user, password: pass}),
      });
      if (r.ok) { await App.checkSession(); App.render(); }
      else {
        const body = await r.json().catch(() => ({}));
        errEl.textContent = body.message || 'Login failed';
      }
    } catch { errEl.textContent = 'Network error'; }
  },

  async doSetup() {
    const user = document.getElementById('auth-user').value;
    const pass = document.getElementById('auth-pass').value;
    const errEl = document.getElementById('auth-error');
    try {
      const r = await fetch('/api/setup/register', {
        method: 'POST', headers: {'Content-Type':'application/json'},
        body: JSON.stringify({username: user, password: pass}),
      });
      if (r.ok) { await App.checkSession(); App.render(); }
      else {
        const body = await r.json().catch(() => ({}));
        // UX-AUTH-02: 409 means setup locked — show login only hint.
        if (r.status === 409) errEl.textContent = 'Setup already completed. Please login.';
        else errEl.textContent = body.message || 'Setup failed';
      }
    } catch { errEl.textContent = 'Network error'; }
  },
};
