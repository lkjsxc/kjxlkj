(function () {
    var uploadButton = document.querySelector('[data-site-icon-upload]');
    var resetButton = document.querySelector('[data-site-icon-reset]');
    var input = document.querySelector('[data-site-icon-input]');
    var preview = document.querySelector('[data-site-icon-preview]');
    var current = document.querySelector('[data-site-icon-current]');
    var status = document.querySelector('[data-site-icon-status]');
    if (!uploadButton || !resetButton || !input || !preview || !current || !status) return;

    var busy = false;

    uploadButton.addEventListener('click', function () {
        if (!busy) input.click();
    });

    input.addEventListener('change', function () {
        var file = input.files && input.files[0];
        input.value = '';
        if (!file || busy) return;
        var formData = new FormData();
        formData.append('icon', file);
        send('/admin/site-icon', formData, 'Icon uploaded.');
    });

    resetButton.addEventListener('click', function () {
        if (busy || resetButton.hidden) return;
        send('/admin/site-icon/reset', null, 'Icon reset.');
    });

    async function send(url, body, successMessage) {
        busy = true;
        setStatus(body ? 'Uploading icon...' : 'Resetting icon...', '');
        toggleButtons(true);
        try {
            var response = await fetch(url, { method: 'POST', body: body || undefined });
            var payload = await readResponse(response);
            if (!response.ok) throw new Error(payload.message || 'Site icon update failed.');
            applyState(payload, successMessage);
        } catch (error) {
            setStatus(error.message || 'Site icon update failed.', 'error');
        } finally {
            busy = false;
            toggleButtons(false);
        }
    }

    function applyState(payload, successMessage) {
        preview.src = '/assets/site-icon?v=' + Date.now();
        current.textContent = payload.configured
            ? 'Current icon: ' + (payload.content_type || 'uploaded icon')
            : 'Current icon: bundled default';
        resetButton.hidden = !payload.configured;
        setStatus(successMessage, '');
    }

    function toggleButtons(disabled) {
        uploadButton.disabled = disabled;
        resetButton.disabled = disabled;
    }

    function setStatus(message, tone) {
        status.textContent = message;
        status.dataset.tone = tone || '';
    }

    async function readResponse(response) {
        var text = await response.text();
        if (!text) return {};
        try {
            return JSON.parse(text);
        } catch (_) {
            return { message: text.replace(/<[^>]*>/g, ' ').replace(/\s+/g, ' ').trim() };
        }
    }
})();
