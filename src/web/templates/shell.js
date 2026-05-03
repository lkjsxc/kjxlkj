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
    app.formatLocalTimes = formatLocalTimes; app.spacePath = spacePath;
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
    setupSpaceRoutes();
    setupImageNavigation();
    setupDrawer();

    function currentSpacePrefix() {
        var first = location.pathname.split('/').filter(Boolean)[0] || '';
        if (!first || ['_', 'account', 'api', 'assets', 'favicon.ico', 'healthz', 'login', 'logout', 'reset-password', 'setup', 'sitemap.xml', 'robots.txt'].includes(first)) {
            return '';
        }
        return '/' + first;
    }
    function spacePath(path) {
        var prefix = currentSpacePrefix();
        if (!prefix || !path || path[0] !== '/') return path;
        if (path === '/') return prefix;
        if (path === '/admin/settings') return prefix + '/settings';
        if (path === '/admin/password') return '/account/password';
        if (path === '/admin/site-icon') return prefix + '/settings/site-icon';
        if (path === '/admin/site-icon/reset') return prefix + '/settings/site-icon/reset';
        if (path === '/admin/markdown-preview') return prefix + '/markdown-preview';
        if (path === '/resources/favorites/order') return prefix + '/favorites/order';
        if (path === '/admin' || path.startsWith('/admin?')) return prefix + path;
        if (path === '/search' || path.startsWith('/search?')) return prefix + path;
        if (path === '/live' || path.startsWith('/live/')) return prefix + path;
        if (path === '/resources' || path.startsWith('/resources/')) return prefix + path;
        return path;
    }
    function setupSpaceRoutes() {
        var nativeFetch = window.fetch;
        window.fetch = function (resource, options) {
            if (typeof resource === 'string') resource = spacePath(resource);
            return nativeFetch.call(this, resource, options);
        };
        document.addEventListener('click', function (event) {
            var link = event.target.closest('a[href]');
            if (link) link.setAttribute('href', spacePath(link.getAttribute('href')));
        }, true);
        document.addEventListener('submit', function (event) {
            var form = event.target.closest('form[action]');
            if (form) form.setAttribute('action', spacePath(form.getAttribute('action')));
        }, true);
    }
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

    function setupImageNavigation() {
        document.addEventListener('click', function (event) {
            if (event.defaultPrevented || event.button !== 0) return;
            var image = event.target.closest('img[data-resource-image-href]');
            if (!image || image.closest('a[href]')) return;
            event.preventDefault();
            app.navigate?.(image.dataset.resourceImageHref, 'push');
        });
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
