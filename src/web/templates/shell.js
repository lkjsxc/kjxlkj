(function () {
    var app = window.kjxlkj = window.kjxlkj || {};
    app.cleanups = app.cleanups || [];
    app.registerCleanup = function (cleanup) {
        if (typeof cleanup === 'function') app.cleanups.push(cleanup);
    };
    app.cleanupPage = function () {
        while (app.cleanups.length) {
            try {
                app.cleanups.pop()();
            } catch (_) {}
        }
        delete app.beforeNavigate;
    };
    app.formatLocalTimes = formatLocalTimes;
    app.runPageScripts = runPageScripts;
    app.setupDrawer = setupDrawer;
    app.syncHeadDocument = syncHeadDocument;

    var headSelectors = [
        'meta[name="description"]', 'meta[name="robots"]', "link[rel='canonical']",
        'meta[property="og:title"]', 'meta[property="og:description"]', 'meta[property="og:type"]',
        'meta[property="og:url"]', 'meta[property="og:image"]', 'meta[property="og:image:type"]',
        'meta[name="twitter:card"]', 'meta[name="twitter:title"]', 'meta[name="twitter:description"]',
        'meta[name="twitter:image"]'
    ];

    formatLocalTimes();
    setupDrawer();

    function formatLocalTimes(root) {
        var formatter = new Intl.DateTimeFormat(undefined, {
            year: 'numeric', month: '2-digit', day: '2-digit', hour: '2-digit', minute: '2-digit', hour12: false,
        });
        var scope = root && typeof root.querySelectorAll === 'function' ? root : document;
        var nodes = Array.from(scope.querySelectorAll('.local-time'));
        if (scope !== document && scope.matches && scope.matches('.local-time')) nodes.unshift(scope);
        nodes.forEach(function (node) {
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
        if (typeof app.disposeDrawer === 'function') app.disposeDrawer();
        var toggle = document.querySelector('[data-menu-toggle]');
        var panel = document.querySelector('[data-menu-panel]');
        var backdrop = document.querySelector('[data-menu-backdrop]');
        if (!toggle || !panel || !backdrop) {
            app.disposeDrawer = null;
            return;
        }
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

        function onToggle() {
            if (document.body.classList.contains('rail-open')) {
                closeMenu(false);
            } else {
                openMenu();
            }
        }

        function onBackdrop() { closeMenu(true); }

        function onKeydown(event) {
            if (event.key === 'Escape' && document.body.classList.contains('rail-open')) {
                closeMenu(true);
            }
        }

        function onMediaChange() { closeMenu(false); }

        toggle.addEventListener('click', onToggle);
        backdrop.addEventListener('click', onBackdrop);
        document.addEventListener('keydown', onKeydown);
        media.addEventListener('change', onMediaChange);
        app.disposeDrawer = function () {
            toggle.removeEventListener('click', onToggle);
            backdrop.removeEventListener('click', onBackdrop);
            document.removeEventListener('keydown', onKeydown);
            media.removeEventListener('change', onMediaChange);
        };
        sync();
    }

    function syncHeadDocument(nextDocument) {
        document.title = nextDocument.title;
        headSelectors.forEach(function (selector) {
            syncHeadNode(nextDocument, selector);
        });
    }

    function syncHeadNode(nextDocument, selector) {
        var current = document.head.querySelector(selector);
        var next = nextDocument.head.querySelector(selector);
        if (!next) return current?.remove();
        var clone = next.cloneNode(true);
        if (current) current.replaceWith(clone);
        else document.head.appendChild(clone);
    }

    function runPageScripts(nextDocument) {
        Array.from(nextDocument.body.querySelectorAll('script')).slice(1).forEach(function (script) {
            var clone = document.createElement('script');
            Array.from(script.attributes).forEach(function (attr) { clone.setAttribute(attr.name, attr.value); });
            clone.textContent = script.textContent;
            document.body.appendChild(clone);
            clone.remove();
        });
    }
})();
