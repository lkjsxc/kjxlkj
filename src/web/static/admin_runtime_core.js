(() => {
  const state = {
    dirty: false,
    saveTimer: null,
    savePromise: null,
  };

  const byId = (id) => document.getElementById(id);
  const editorForm = () => byId("admin-editor-form");

  const parseAndSwap = (html, ids) => {
    const parsed = new DOMParser().parseFromString(html, "text/html");
    ids.forEach((id) => {
      const current = byId(id);
      const next = parsed.getElementById(id);
      if (current && next) {
        current.replaceWith(next);
      }
    });
  };

  const setDirty = (next) => {
    state.dirty = next;
    const indicator = byId("admin-unsaved-indicator");
    if (indicator) {
      indicator.dataset.unsaved = String(next);
      indicator.textContent = next ? "Unsaved changes" : "All changes saved";
    }
  };

  const postEditor = async (path, keepalive = false) => {
    const form = editorForm();
    if (!form) {
      return { ok: false, text: "" };
    }

    const body = new URLSearchParams(new FormData(form)).toString();
    const response = await fetch(path, {
      method: "POST",
      keepalive,
      credentials: "same-origin",
      headers: {
        "Content-Type": "application/x-www-form-urlencoded;charset=UTF-8",
        "HX-Request": "true",
      },
      body,
    });

    return { ok: response.ok, text: await response.text() };
  };

  window.AdminRuntimeShared = {
    state,
    byId,
    editorForm,
    parseAndSwap,
    postEditor,
    setDirty,
  };

  setDirty(false);
})();
