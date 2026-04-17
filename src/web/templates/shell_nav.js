(function () {
    var app = window.kjxlkj = window.kjxlkj || {};
    if (app.shellNavReady) return;
    app.shellNavReady = true;
    app.navigate = requestNavigation;
    app.replaceCurrentUrl = replaceCurrentUrl;
    app.currentUrl = window.location.href;
    app.historyIndex = ensureHistoryState();

    document.addEventListener('click', onLinkClick);
    window.addEventListener('popstate', onPopState);

    async function onPopState(event) {
        if (app.ignoreNextPopstate) {
            app.ignoreNextPopstate = false;
            return;
        }
        var direction = compareHistoryIndex(event.state);
        var result = await requestNavigation(window.location.href, 'pop');
        if (result === 'blocked') revertPop(direction);
    }

    async function onLinkClick(event) {
        if (event.defaultPrevented || event.button !== 0 || event.metaKey || event.ctrlKey || event.shiftKey || event.altKey) return;
        var link = event.target.closest('a[href]');
        if (!link || !shouldIntercept(link)) return;
        event.preventDefault();
        await requestNavigation(app.resolveRememberedUrl?.(link.href) || link.href, 'push');
    }

    async function requestNavigation(url, historyMode) {
        if (app.navigating) {
            app.pendingNavigation = { url: url, historyMode: historyMode };
            app.abortNavigation?.();
            return 'queued';
        }
        return navigate(url, historyMode);
    }

    async function navigate(url, historyMode) {
        if (app.navigating) return;
        var currentShell = document.querySelector('.shell-frame');
        if (!currentShell) return fullNavigate(url);
        app.captureCurrentPageState?.(true);
        app.navigating = true;
        app.navigationUrl = url;
        app.navigationMode = historyMode;
        var controller = null;
        try {
            if (typeof app.beforeNavigate === 'function') {
                try {
                    if (!await app.beforeNavigate(url, historyMode)) return 'blocked';
                } catch {
                    return 'blocked';
                }
            }
            controller = new AbortController();
            app.abortNavigation = function () { controller.abort(); };
            var rail = document.querySelector('.shell-rail');
            var railScroll = rail ? rail.scrollTop : 0;
            var drawerOpen = document.body.classList.contains('rail-open');
            var response = await fetch(url, {
                headers: { 'X-Requested-With': 'fetch' },
                signal: controller.signal
            });
            if (!response.ok) return fullNavigate(url);
            var text = await response.text();
            var nextDocument = new DOMParser().parseFromString(text, 'text/html');
            var nextShell = nextDocument.querySelector('.shell-frame');
            if (!nextShell) return fullNavigate(url);
            if (typeof app.cleanupPage === 'function') app.cleanupPage();
            currentShell.replaceWith(nextShell);
            app.syncHeadDocument?.(nextDocument);
            app.setupDrawer?.();
            document.body.classList.toggle('rail-open', drawerOpen);
            app.formatLocalTimes?.();
            app.runPageScripts?.(nextDocument);
            await restoreRailScroll(railScroll);
            syncHistory(url, historyMode);
            if (!await app.restoreRememberedPageState?.()) window.scrollTo(0, 0);
            app.captureCurrentPageState?.(true);
            return 'ok';
        } catch (_) {
            if (controller?.signal.aborted) return 'queued';
            return fullNavigate(url);
        } finally {
            app.navigating = false;
            app.abortNavigation = null;
            app.navigationUrl = null;
            app.navigationMode = null;
            if (app.pendingNavigation) {
                var pending = app.pendingNavigation;
                app.pendingNavigation = null;
                requestAnimationFrame(function () {
                    requestNavigation(pending.url, pending.historyMode);
                });
            }
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

    function syncHistory(url, historyMode) {
        if (historyMode === 'push' && url !== window.location.href) {
            app.historyIndex = (typeof app.historyIndex === 'number' ? app.historyIndex : 0) + 1;
            window.history.pushState(historyState(app.historyIndex), '', url);
        } else if (historyMode === 'replace') {
            replaceCurrentUrl(url);
            return;
        } else if (historyMode === 'pop') {
            var index = readHistoryIndex(window.history.state);
            if (index !== null) app.historyIndex = index;
        }
        app.currentUrl = window.location.href;
    }

    function replaceCurrentUrl(url) {
        if (!url) return;
        window.history.replaceState(historyState(app.historyIndex), '', url);
        app.currentUrl = window.location.href;
    }

    function ensureHistoryState() {
        var index = readHistoryIndex(window.history.state);
        if (index === null) index = 0;
        window.history.replaceState(historyState(index), '', window.location.href);
        return index;
    }

    function historyState(index) {
        return { kjxlkj: true, index: index };
    }

    function readHistoryIndex(state) {
        return state && state.kjxlkj && typeof state.index === 'number' ? state.index : null;
    }

    function compareHistoryIndex(state) {
        var next = readHistoryIndex(state);
        if (next === null || typeof app.historyIndex !== 'number') return 0;
        return next - app.historyIndex;
    }

    function revertPop(direction) {
        if (!direction) return replaceCurrentUrl(app.currentUrl);
        app.ignoreNextPopstate = true;
        window.history.go(direction < 0 ? 1 : -1);
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
        return 'blocked';
    }
})();
