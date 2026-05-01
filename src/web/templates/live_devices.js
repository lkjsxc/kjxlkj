(function () {
    var live = window.kjxlkjLive;
    if (!live) return;
    var listening = false;
    live.loadCameras = loadCameras;
    live.initCameraDevices = initCameraDevices;
    live.cleanupCameraDevices = cleanupCameraDevices;

    function initCameraDevices() {
        if (listening || live.role !== 'broadcaster') return;
        listening = true;
        loadCameras();
        navigator.mediaDevices?.addEventListener?.('devicechange', loadCameras);
        document.addEventListener('visibilitychange', onVisible);
        window.addEventListener('focus', loadCameras);
    }

    async function loadCameras() {
        if (!live.camera || !navigator.mediaDevices?.enumerateDevices) return;
        var selected = live.camera.value;
        var devices = (await navigator.mediaDevices.enumerateDevices())
            .filter(function (item) { return item.kind === 'videoinput'; });
        live.camera.innerHTML = '<option value="">Auto by facing</option>' + devices.map(cameraOption).join('');
        if (selected && devices.some(function (item) { return item.deviceId === selected; })) {
            live.camera.value = selected;
        }
        live.syncSourceUi();
    }

    function cameraOption(item, index) {
        var label = item.label || 'Camera ' + (index + 1);
        return '<option value="' + escapeHtml(item.deviceId) + '">' + escapeHtml(label) + '</option>';
    }

    function onVisible() {
        if (!document.hidden) loadCameras();
    }

    function cleanupCameraDevices() {
        if (!listening) return;
        listening = false;
        navigator.mediaDevices?.removeEventListener?.('devicechange', loadCameras);
        document.removeEventListener('visibilitychange', onVisible);
        window.removeEventListener('focus', loadCameras);
    }

    function escapeHtml(value) {
        return String(value).replace(/[&<>"]/g, function (ch) {
            return ({ '&': '&amp;', '<': '&lt;', '>': '&gt;', '"': '&quot;' })[ch];
        });
    }
})();
