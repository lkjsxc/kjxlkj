(() => {
  const shared = window.AdminRuntimeShared;
  if (!shared) {
    return;
  }

  const fieldIds = new Set(["title", "body", "private"]);
  const savePath = "/admin/save";
  const previewPath = "/admin/preview";

  const saveNow = async (force = false) => {
    if (!force && !shared.state.dirty) {
      return true;
    }
    if (shared.state.savePromise) {
      return shared.state.savePromise;
    }

    shared.state.savePromise = shared
      .postEditor(savePath)
      .then(({ ok, text }) => {
        if (!ok) {
          return false;
        }
        shared.parseAndSwap(text, [
          "admin-status-banner",
          "admin-conflict-banner",
          "last_known_revision",
        ]);
        shared.setDirty(false);
        return true;
      })
      .catch(() => false)
      .finally(() => {
        shared.state.savePromise = null;
      });

    return shared.state.savePromise;
  };

  const scheduleAutosave = () => {
    if (!shared.state.dirty) {
      return;
    }
    window.clearTimeout(shared.state.saveTimer);
    shared.state.saveTimer = window.setTimeout(() => {
      void saveNow();
    }, 2000);
  };

  const previewNow = async () => {
    const { ok, text } = await shared.postEditor(previewPath);
    if (ok) {
      shared.parseAndSwap(text, ["admin-preview-pane"]);
    }
  };

  const allowNavigation = async () => {
    if (!shared.state.dirty) {
      return true;
    }
    if (shared.state.savePromise) {
      await Promise.race([
        shared.state.savePromise,
        new Promise((resolve) => window.setTimeout(resolve, 1200)),
      ]);
      if (!shared.state.dirty) {
        return true;
      }
    }
    return window.confirm("Discard unsaved changes?");
  };

  const routeOpen = async (anchor) => {
    if (!(await allowNavigation())) {
      return;
    }
    const hxGet = anchor.getAttribute("hx-get");
    if (window.htmx && hxGet) {
      window.htmx.ajax("GET", hxGet, {
        target: "#admin-editor-pane",
        swap: "outerHTML",
      });
      shared.setDirty(false);
    } else {
      window.location.assign(anchor.href);
    }
  };

  document.addEventListener(
    "input",
    (event) => {
      if (fieldIds.has(event.target.id)) {
        shared.setDirty(true);
        scheduleAutosave();
      }
    },
    true
  );

  document.addEventListener(
    "change",
    (event) => {
      if (fieldIds.has(event.target.id)) {
        shared.setDirty(true);
        scheduleAutosave();
      }
    },
    true
  );

  document.addEventListener(
    "blur",
    (event) => {
      if (fieldIds.has(event.target.id) && shared.state.dirty) {
        void saveNow();
      }
    },
    true
  );

  document.addEventListener(
    "submit",
    (event) => {
      const form = event.target;
      if (!(form instanceof HTMLFormElement)) {
        return;
      }
      if (form.id === "admin-editor-form") {
        event.preventDefault();
        void saveNow(true);
      } else if (form.matches("[data-admin-nav-form]")) {
        event.preventDefault();
        void allowNavigation().then((allowed) => {
          if (allowed) {
            form.submit();
          }
        });
      }
    },
    true
  );

  document.addEventListener("click", (event) => {
    const previewButton = event.target.closest("#admin-preview-button");
    if (previewButton) {
      event.preventDefault();
      void previewNow();
      return;
    }

    const openLink = event.target.closest("a[data-admin-open]");
    if (openLink) {
      event.preventDefault();
      void routeOpen(openLink);
      return;
    }

    const continueButton = event.target.closest('button[data-action="continue-editing"]');
    if (continueButton) {
      const banner = shared.byId("admin-conflict-banner");
      if (banner) {
        banner.dataset.conflict = "false";
        banner.textContent = "";
      }
    }
  });

  window.addEventListener("beforeunload", (event) => {
    if (!shared.state.dirty) {
      return;
    }
    void shared.postEditor(savePath, true);
    event.preventDefault();
    event.returnValue = "";
  });

  window.AdminRuntime = {
    allowNavigation,
    focusQuickOpen() {
      const quickOpen = shared.byId("admin-quick-open");
      quickOpen?.focus();
      quickOpen?.select();
    },
    previewNow,
    saveNow,
    showCreate() {
      const panel = shared.byId("admin-create-panel");
      if (panel) {
        panel.hidden = false;
      }
      shared.byId("admin-create-slug")?.focus();
    },
  };
})();
