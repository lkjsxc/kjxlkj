(() => {
  const hasPrimary = (event) => (event.ctrlKey || event.metaKey) && !event.altKey;

  document.addEventListener("keydown", (event) => {
    if (!hasPrimary(event)) {
      return;
    }

    const runtime = window.AdminRuntime;
    if (!runtime) {
      return;
    }

    const key = event.key.toLowerCase();
    if (key === "s" && !event.shiftKey) {
      event.preventDefault();
      void runtime.saveNow(true);
      return;
    }

    if (key === "n" && !event.shiftKey) {
      event.preventDefault();
      runtime.showCreate();
      return;
    }

    if (key === "p" && event.shiftKey) {
      event.preventDefault();
      void runtime.previewNow();
      return;
    }

    if (key === "k" && !event.shiftKey) {
      event.preventDefault();
      runtime.focusQuickOpen();
    }
  });
})();
