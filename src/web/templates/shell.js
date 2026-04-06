(function () {
    formatLocalTimes();
    setupDrawer();

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
            var map = Object.fromEntries(
                parts
                    .filter(function (part) { return part.type !== 'literal'; })
                    .map(function (part) { return [part.type, part.value]; })
            );
            node.textContent =
                [map.year, map.month, map.day].join('-') + ' ' + map.hour + ':' + map.minute;
        });
    }

    function setupDrawer() {
        var toggle = document.querySelector('[data-menu-toggle]');
        var panel = document.querySelector('[data-menu-panel]');
        var backdrop = document.querySelector('[data-menu-backdrop]');
        if (!toggle || !panel || !backdrop) return;
        var media = window.matchMedia('(max-width: 900px)');

        function sync() {
            var open = document.body.classList.contains('rail-open');
            if (media.matches) {
                panel.removeAttribute('aria-hidden');
                panel.toggleAttribute('inert', !open);
                toggle.setAttribute('aria-expanded', open ? 'true' : 'false');
                backdrop.hidden = !open;
            } else {
                document.body.classList.remove('rail-open');
                panel.removeAttribute('aria-hidden');
                panel.removeAttribute('inert');
                toggle.setAttribute('aria-expanded', 'false');
                backdrop.hidden = true;
            }
        }

        function openMenu() {
            if (!media.matches) return;
            document.body.classList.add('rail-open');
            sync();
            panel.querySelector('a, button, input, textarea, select')?.focus();
        }

        function closeMenu(restoreFocus) {
            document.body.classList.remove('rail-open');
            sync();
            if (restoreFocus) toggle.focus();
        }

        toggle.addEventListener('click', function () {
            if (document.body.classList.contains('rail-open')) {
                closeMenu(false);
            } else {
                openMenu();
            }
        });
        backdrop.addEventListener('click', function () { closeMenu(true); });
        document.addEventListener('keydown', function (event) {
            if (event.key === 'Escape' && document.body.classList.contains('rail-open')) {
                closeMenu(true);
            }
        });
        media.addEventListener('change', function () { closeMenu(false); });
        sync();
    }
})();
