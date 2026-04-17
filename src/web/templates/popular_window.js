(function () {
    if (window.kjxlkj?.popularWindowReady) return;
    if (window.kjxlkj) window.kjxlkj.popularWindowReady = true;
    if (window.kjxlkj) window.kjxlkj.refreshPopularSection = refreshSection;
    var controllers = {};
    var requestIds = {};

    document.addEventListener('click', function (event) {
        var button = event.target.closest('[data-popular-window]');
        if (!button) return;
        var section = button.closest('[data-popular-section]');
        if (!section || button.getAttribute('aria-pressed') === 'true') return;
        event.preventDefault();
        refreshSection(section, button.dataset.popularWindow || '');
    });

    async function refreshSection(section, windowValue) {
        var surface = section.dataset.popularSurface || '';
        if (!surface || !windowValue) return;
        clearError(section);
        setBusy(section, true);
        if (controllers[surface]) controllers[surface].abort();
        var controller = new AbortController();
        controllers[surface] = controller;
        requestIds[surface] = (requestIds[surface] || 0) + 1;
        var requestId = requestIds[surface];
        try {
            var response = await fetch('/_/popular-resources/' + surface + '/' + windowValue, {
                headers: { 'X-Requested-With': 'fetch' },
                signal: controller.signal,
            });
            if (!response.ok) throw new Error('Popular refresh failed.');
            var replacement = parseSection(await response.text());
            window.kjxlkj?.formatLocalTimes?.(replacement);
            if (requestIds[surface] !== requestId) return;
            section.replaceWith(replacement);
            window.kjxlkj?.captureCurrentPageState?.();
        } catch (error) {
            if (error.name === 'AbortError' || requestIds[surface] !== requestId) return;
            setBusy(section, false);
            showError(section, 'Failed to refresh Popular.');
        } finally {
            if (controllers[surface] === controller) controllers[surface] = null;
            var current = document.querySelector('[data-popular-section][data-popular-surface="' + surface + '"]');
            if (current && requestIds[surface] === requestId) setBusy(current, false);
        }
    }

    function parseSection(html) {
        var template = document.createElement('template');
        template.innerHTML = html.trim();
        var section = template.content.firstElementChild;
        if (!section || !section.matches('[data-popular-section]')) {
            throw new Error('Popular refresh failed.');
        }
        return section;
    }

    function setBusy(section, busy) {
        section.setAttribute('aria-busy', busy ? 'true' : 'false');
        Array.from(section.querySelectorAll('[data-popular-window]')).forEach(function (button) {
            button.disabled = busy;
        });
    }

    function clearError(section) {
        var errorNode = section.querySelector('[data-popular-error]');
        if (!errorNode) return;
        errorNode.hidden = true;
        errorNode.textContent = '';
    }

    function showError(section, message) {
        var errorNode = section.querySelector('[data-popular-error]');
        if (!errorNode) return;
        errorNode.hidden = false;
        errorNode.textContent = message;
    }
})();
