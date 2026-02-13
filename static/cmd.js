// kjxlkj — Command Palette (UX-NAV-02)
'use strict';

const CmdPalette = {
  renderContainer() {
    return `
    <div class="cmd-palette" id="cmd-palette" role="dialog" aria-label="Command palette">
      <input id="cmd-input" type="text" placeholder="Type a command or search..." aria-label="Command search">
      <div class="results" id="cmd-results"></div>
    </div>`;
  },

  toggle(open) {
    const el = document.getElementById('cmd-palette');
    if (!el) return;
    if (open) {
      el.classList.add('open');
      const input = document.getElementById('cmd-input');
      if (input) { input.value = ''; input.focus(); }
      this.bind();
    } else {
      el.classList.remove('open');
    }
  },

  bind() {
    const input = document.getElementById('cmd-input');
    if (!input) return;
    input.oninput = () => this.search(input.value);
    input.onkeydown = (e) => {
      if (e.key === 'Escape') { App.state.cmdOpen = false; this.toggle(false); }
      if (e.key === 'Enter') this.executeSelected();
    };
  },

  search(query) {
    const results = document.getElementById('cmd-results');
    if (!results) return;
    const q = query.toLowerCase();
    const commands = [
      {label: 'New Note', action: () => EditorUI.createNote()},
      {label: 'Logout', action: async () => { await fetch('/api/auth/logout',{method:'POST'}); App.state.user=null; App.render(); }},
    ];
    // Add notes as searchable items
    const noteItems = App.state.notes
      .filter(n => !q || (n.title||'').toLowerCase().includes(q))
      .slice(0, 10)
      .map(n => ({label: `Open: ${n.title||'Untitled'}`, action: () => EditorUI.openNote(n.id)}));

    const all = [...commands.filter(c => !q || c.label.toLowerCase().includes(q)), ...noteItems];
    results.innerHTML = all.map((item, i) =>
      `<div class="result-item${i===0?' selected':''}" data-idx="${i}">${esc(item.label)}</div>`
    ).join('');
    this._items = all;

    results.querySelectorAll('.result-item').forEach((el, i) => {
      el.onclick = () => { if (this._items[i]) { this._items[i].action(); this.toggle(false); App.state.cmdOpen=false; } };
    });
  },

  executeSelected() {
    if (this._items && this._items.length > 0) {
      this._items[0].action();
      this.toggle(false);
      App.state.cmdOpen = false;
    }
  },
};
