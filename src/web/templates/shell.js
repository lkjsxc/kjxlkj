(function () {
    function appShell() {
        return document.querySelector('.app-shell');
    }

    function button() {
        return document.querySelector('[data-menu-toggle]');
    }

    function closeMenu() {
        var shell = appShell();
        var trigger = button();
        if (!shell || !trigger) return;
        shell.classList.remove('drawer-open');
        trigger.setAttribute('aria-expanded', 'false');
        document.getElementById('shell-rail')?.setAttribute('aria-hidden', 'true');
    }

    function openMenu() {
        var shell = appShell();
        var trigger = button();
        if (!shell || !trigger) return;
        shell.classList.add('drawer-open');
        trigger.setAttribute('aria-expanded', 'true');
        document.getElementById('shell-rail')?.setAttribute('aria-hidden', 'false');
    }

    function syncState() {
        var trigger = button();
        if (!trigger) return;
        trigger.setAttribute('aria-expanded', 'false');
        if (window.innerWidth <= 900) {
            document.getElementById('shell-rail')?.setAttribute('aria-hidden', 'true');
        }
    }

    document.addEventListener('click', function (event) {
        if (event.target.closest('[data-menu-toggle]')) openMenu();
        if (event.target.closest('[data-menu-close]')) closeMenu();
    });

    document.addEventListener('keydown', function (event) {
        if (event.key === 'Escape') closeMenu();
    });

    window.addEventListener('resize', function () {
        if (window.innerWidth > 900) closeMenu();
    });

    syncState();
})();
