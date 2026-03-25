(function () {
    function isCompact() {
        return window.innerWidth <= 900;
    }

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
        document.body.classList.remove('nav-open');
        trigger.setAttribute('aria-expanded', 'false');
        document
            .getElementById('shell-rail')
            ?.setAttribute('aria-hidden', isCompact() ? 'true' : 'false');
    }

    function openMenu() {
        var shell = appShell();
        var trigger = button();
        if (!shell || !trigger) return;
        shell.classList.add('drawer-open');
        document.body.classList.add('nav-open');
        trigger.setAttribute('aria-expanded', 'true');
        document.getElementById('shell-rail')?.setAttribute('aria-hidden', 'false');
        document.querySelector('.rail-close')?.focus();
    }

    function syncState() {
        var shell = appShell();
        var trigger = button();
        var rail = document.getElementById('shell-rail');
        if (!shell || !trigger || !rail) return;
        if (isCompact()) {
            trigger.setAttribute(
                'aria-expanded',
                shell.classList.contains('drawer-open') ? 'true' : 'false'
            );
            rail.setAttribute(
                'aria-hidden',
                shell.classList.contains('drawer-open') ? 'false' : 'true'
            );
        } else {
            shell.classList.remove('drawer-open');
            document.body.classList.remove('nav-open');
            trigger.setAttribute('aria-expanded', 'false');
            rail.setAttribute('aria-hidden', 'false');
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
        syncState();
    });

    syncState();
})();
