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
        document.getElementById('shell-rail')?.setAttribute('aria-hidden', isCompact() ? 'true' : 'false');
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
            trigger.setAttribute('aria-expanded', shell.classList.contains('drawer-open') ? 'true' : 'false');
            rail.setAttribute('aria-hidden', shell.classList.contains('drawer-open') ? 'false' : 'true');
        } else {
            shell.classList.remove('drawer-open');
            document.body.classList.remove('nav-open');
            trigger.setAttribute('aria-expanded', 'false');
            rail.setAttribute('aria-hidden', 'false');
        }
    }

    function formatLocalTimes() {
        var formatter = new Intl.DateTimeFormat(undefined, {
            year: 'numeric',
            month: '2-digit',
            day: '2-digit',
            hour: '2-digit',
            minute: '2-digit',
            hour12: false,
        });
        document.querySelectorAll('.local-time').forEach(function (node) {
            var iso = node.getAttribute('datetime') || node.dataset.utc;
            if (!iso) return;
            var date = new Date(iso);
            if (Number.isNaN(date.getTime())) return;
            var parts = formatter.formatToParts(date);
            var map = Object.fromEntries(parts.filter(function (part) { return part.type !== 'literal'; }).map(function (part) { return [part.type, part.value]; }));
            node.textContent = [map.year, map.month, map.day].join('-') + ' ' + map.hour + ':' + map.minute;
        });
    }

    document.addEventListener('click', function (event) {
        if (event.target.closest('[data-menu-toggle]')) openMenu();
        if (event.target.closest('[data-menu-close]')) closeMenu();
    });

    document.addEventListener('keydown', function (event) {
        if (event.key === 'Escape') closeMenu();
    });

    window.addEventListener('resize', syncState);

    syncState();
    formatLocalTimes();
})();
