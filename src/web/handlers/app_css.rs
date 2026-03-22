pub const APP_CSS: &str = r#"
:root { --bg: #121212; --panel: #1e1e1e; --text: #dddddd; --muted: #9a9a9a; --accent: #7a7a7a; }
* { box-sizing: border-box; }
body { margin: 0; color: var(--text); background: var(--bg); font-family: ui-sans-serif,system-ui; }
a { color: var(--text); text-decoration: none; }
#app-shell { display: grid; grid-template-columns: 280px 1fr; min-height: 100vh; }
#app-topbar { display: none; align-items: center; gap: 0.8rem; padding: 0.8rem 1rem; background: var(--panel); border-bottom: 1px solid #2b2b2b; position: sticky; top: 0; }
#app-nav-toggle { border: 1px solid #3a3a3a; background: transparent; color: var(--text); border-radius: 6px; padding: 0.3rem 0.5rem; }
#app-nav { padding: 1rem; background: #171717; border-right: 1px solid #2b2b2b; overflow: auto; }
#app-nav h2 { font-size: 0.95rem; color: var(--muted); margin: 0.6rem 0; }
#app-nav ul { margin: 0; padding-left: 1rem; }
#app-main { padding: 1rem 1.2rem; }
main { max-width: 1100px; }
input, textarea, button { background: #151515; color: var(--text); border: 1px solid #444; border-radius: 6px; padding: 0.45rem; }
textarea { width: 100%; }
button { cursor: pointer; }
@media (max-width: 900px) {
  #app-shell { grid-template-columns: 1fr; }
  #app-topbar { display: flex; }
  #app-nav { position: fixed; inset: 56px auto 0 0; width: 280px; transform: translateX(-100%); transition: transform 0.2s; z-index: 20; }
  #app-shell[data-nav-open="true"] #app-nav { transform: translateX(0); }
}
"#;
