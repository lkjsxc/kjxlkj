(function () {
    var app = window.kjxlkj = window.kjxlkj || {};
    if (app.shellMemoryReady) return;
    app.shellMemoryReady = true;
    app.captureCurrentPageState = captureCurrentPageState;
    app.resolveRememberedUrl = resolveRememberedUrl;
    app.restoreRememberedPageState = restoreRememberedPageState;
    var storageKey = 'kjxlkj:shell-memory:v1';
    var scrollTimer = null;

    window.addEventListener('scroll', scheduleCapture, { passive: true });
    document.addEventListener('change', scheduleCapture, true);
    init();

    function init() {
        captureCurrentPageState();
        updateRememberedLinks();
    }

    function captureCurrentPageState(force) {
        if (app.navigating && !force) return;
        var state = readState();
        if (window.location.pathname === '/') {
            state.home = {
                popularWindow: currentPopularWindow() || state.home?.popularWindow || '30d',
                scrollY: Math.round(window.scrollY),
            };
        } else if (window.location.pathname === '/search') {
            state.search = {
                url: window.location.pathname + window.location.search,
                scrollY: Math.round(window.scrollY),
            };
        }
        writeState(state);
        updateRememberedLinks();
    }

    function resolveRememberedUrl(url) {
        var target = new URL(url, window.location.href);
        if (target.origin !== window.location.origin) return target.href;
        if (target.pathname === '/search') return searchHref(target);
        return target.href;
    }

    async function restoreRememberedPageState() {
        updateRememberedLinks();
        if (window.location.pathname === '/') return restoreHomeState();
        if (window.location.pathname === '/search') return restoreSearchState();
        return false;
    }

    async function restoreHomeState() {
        var state = readState().home;
        if (!state) return false;
        var section = document.querySelector('[data-popular-section][data-popular-surface="home"]');
        if (section && state.popularWindow && currentPopularWindow() !== state.popularWindow) {
            if (typeof app.refreshPopularSection === 'function') {
                await app.refreshPopularSection(section, state.popularWindow);
            } else {
                document.querySelector('[data-popular-window="' + state.popularWindow + '"]')?.click();
                await nextFrame();
                await nextFrame();
            }
        }
        await restoreScroll(state.scrollY);
        return true;
    }

    async function restoreSearchState() {
        var state = readState().search;
        if (!state) return false;
        await restoreScroll(state.scrollY);
        return true;
    }

    function scheduleCapture() {
        if (!isRememberedPage()) return;
        window.clearTimeout(scrollTimer);
        scrollTimer = window.setTimeout(captureCurrentPageState, 120);
    }

    function updateRememberedLinks() {
        var state = readState();
        updateRememberedHref('home', '/');
        updateRememberedHref('search', state.search?.url || '/search');
    }

    function updateRememberedHref(name, href) {
        document.querySelectorAll('[data-shell-remember="' + name + '"]').forEach(function (link) {
            link.setAttribute('href', href);
        });
    }

    function currentPopularWindow() {
        return document.querySelector(
            '[data-popular-section][data-popular-surface="home"] [data-popular-window][aria-pressed="true"]'
        )?.dataset.popularWindow || null;
    }

    function searchHref(fallback) {
        var remembered = readState().search?.url;
        return remembered ? new URL(remembered, window.location.origin).href : fallback.href;
    }

    function isRememberedPage() {
        return window.location.pathname === '/' || window.location.pathname === '/search';
    }

    function readState() {
        try {
            return JSON.parse(window.sessionStorage.getItem(storageKey) || '{}');
        } catch (_) {
            return {};
        }
    }

    function writeState(state) {
        try {
            window.sessionStorage.setItem(storageKey, JSON.stringify(state));
        } catch (_) {}
    }

    async function restoreScroll(scrollY) {
        await nextFrame();
        window.scrollTo(0, scrollY || 0);
        await nextFrame();
        window.scrollTo(0, scrollY || 0);
        captureCurrentPageState();
    }

    function nextFrame() {
        return new Promise(function (resolve) {
            requestAnimationFrame(function () { resolve(); });
        });
    }
})();
