(() => {
  const shell = document.getElementById("app-shell");
  const toggle = document.getElementById("app-nav-toggle");
  if (!shell || !toggle) return;
  const setOpen = (open) => {
    shell.dataset.navOpen = String(open);
    toggle.setAttribute("aria-expanded", String(open));
  };
  setOpen(false);
  toggle.addEventListener("click", () => setOpen(shell.dataset.navOpen !== "true"));
  document.addEventListener("keydown", (event) => {
    if (event.key === "Escape") setOpen(false);
  });
})();
