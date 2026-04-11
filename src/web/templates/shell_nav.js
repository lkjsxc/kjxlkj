(function () {
    var app = window.kjxlkj = window.kjxlkj || {};
    if (app.shellNavReady) return;
    app.shellNavReady = true;
    app.navigate = navigate;

    document.addEventListener('click', onLinkClick);
    window.addEventListener('popstate', function () {
        navigate(window.location.href, 'none');
    });

    async function onLinkClick(event) {
        if (event.defaultPrevented || event.button !== 0 || event.metaKey || event.ctrlKey || event.shiftKey || event.altKey) return;
        var link = event.target.closest('a[href]');
        if (!link || !shouldIntercept(link)) return;
        event.preventDefault();
        await navigate(link.href, 'push');
    }

    async function navigate(url, historyMode) {
        if (app.navigating) return;
        var currentShell = document.querySelector('.shell-frame');
        if (!currentShell) return fullNavigate(url);
        if (typeof app.beforeNavigate === 'function') {
            try {
                if (!await app.beforeNavigate(url)) return;
            } catch {
                return;
            }
        }
        app.navigating = true;
        var rail = document.querySelector('.shell-rail');
        var railScroll = rail ? rail.scrollTop : 0;
        var drawerOpen = document.body.classList.contains('rail-open');
        try {
            var response = await fetch(url, { headers: { 'X-Requested-With': 'fetch' } });
            if (!response.ok) return fullNavigate(url);
            var text = await response.text();
            var nextDocument = new DOMParser().parseFromString(text, 'text/html');
            var nextShell = nextDocument.querySelector('.shell-frame');
            if (!nextShell) return fullNavigate(url);
            if (typeof app.cleanupPage === 'function') app.cleanupPage();
            currentShell.replaceWith(nextShell);
            syncHead(nextDocument);
            app.setupDrawer?.();
            document.body.classList.toggle('rail-open', drawerOpen);
            app.formatLocalTimes?.();
            runPageScripts(nextDocument);
            await restoreRailScroll(railScroll);
            window.scrollTo(0, 0);
            syncHistory(url, historyMode);
        } catch (_) {
            fullNavigate(url);
        } finally {
            app.navigating = false;
        }
    }

    function shouldIntercept(link) {
        if (link.target || link.hasAttribute('download')) return false;
        var href = link.href;
        if (!href) return false;
        var url = new URL(href, window.location.href);
        if (url.origin !== window.location.origin) return false;
        if (url.href === window.location.href) return false;
        if (url.hash && url.pathname === window.location.pathname && url.search === window.location.search) return false;
        if (url.pathname.startsWith('/login') || url.pathname.startsWith('/setup') || url.pathname.startsWith('/reset-password')) return false;
        if (url.pathname.endsWith('/file') || url.pathname.startsWith('/assets/')) return false;
        return !!document.querySelector('.shell-frame');
    }

    function syncHead(nextDocument) {
        document.title = nextDocument.title;
        syncHeadNode(nextDocument, 'meta[name="description"]');
        syncHeadNode(nextDocument, 'meta[name="robots"]');
        syncHeadNode(nextDocument, "link[rel='canonical']");
    }

    function syncHeadNode(nextDocument, selector) {
        var current = document.head.querySelector(selector);
        var next = nextDocument.head.querySelector(selector);
        if (!next) {
            if (current) current.remove();
            return;
        }
        var clone = next.cloneNode(true);
        if (current) {
            current.replaceWith(clone);
        } else {
            document.head.appendChild(clone);
        }
    }

    function runPageScripts(nextDocument) {
        Array.from(nextDocument.body.querySelectorAll('script')).slice(1).forEach(function (script) {
            var clone = document.createElement('script');
            Array.from(script.attributes).forEach(function (attr) {
                clone.setAttribute(attr.name, attr.value);
            });
            clone.textContent = script.textContent;
            document.body.appendChild(clone);
            clone.remove();
        });
    }

    function syncHistory(url, historyMode) {
        if (historyMode === 'push' && url !== window.location.href) {
            window.history.pushState({}, '', url);
        } else if (historyMode === 'replace') {
            window.history.replaceState({}, '', url);
        }
    }

    async function restoreRailScroll(scrollTop) {
        await nextFrame();
        setRailScroll(scrollTop);
        await nextFrame();
        setRailScroll(scrollTop);
    }

    function setRailScroll(scrollTop) {
        var rail = document.querySelector('.shell-rail');
        if (rail) rail.scrollTop = scrollTop;
    }

    function nextFrame() {
        return new Promise(function (resolve) {
            requestAnimationFrame(function () { resolve(); });
        });
    }

    function fullNavigate(url) {
        window.location.href = url;
    }
})();
