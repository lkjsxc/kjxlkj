(function () {
    var root = document.querySelector('[data-live-root]');
    if (!root) return;
    var live = window.kjxlkjLive = {
        root: root,
        app: window.kjxlkj || {},
        role: root.dataset.liveRole,
        video: root.querySelector('[data-live-video]'),
        state: root.querySelector('[data-live-state]'),
        detail: root.querySelector('[data-live-detail]'),
        start: root.querySelector('[data-live-start]'),
        stop: root.querySelector('[data-live-stop]'),
        source: root.querySelector('[data-live-source]'),
        camera: root.querySelector('[data-live-camera]'),
        refresh: root.querySelector('[data-live-camera-refresh]'),
        height: root.querySelector('[data-live-height]'),
        fps: root.querySelector('[data-live-fps]'),
        mic: root.querySelector('[data-live-mic]'),
        viewerCount: root.querySelector('[data-live-viewer-count]'),
        ws: null,
        localStream: null,
        peers: {},
        pendingIce: {},
        activeVideo: {},
        closed: false,
    };
    live.config = readConfig();
    live.setStatus = setStatus;
    live.selectedVideo = selectedVideo;
    live.sameVideo = sameVideo;
    live.videoConstraints = videoConstraints;
    live.cameraConstraints = cameraConstraints;
    live.setRunning = setRunning;
    live.statusText = statusText;
    live.updateViewerCount = updateViewerCount;
    live.loadCameras = loadCameras;
    live.syncSourceUi = syncSourceUi;

    function readConfig() {
        try { return JSON.parse(document.getElementById('live-config').textContent || '{}'); }
        catch (_) { return {}; }
    }

    function setStatus(label, text) {
        live.state.textContent = label;
        live.detail.textContent = text || '';
    }

    function selectedVideo() {
        return {
            source: live.source?.value || live.config.source || 'screen',
            device: live.camera?.value || '',
        };
    }

    function sameVideo(next) {
        return live.activeVideo.source === next.source && live.activeVideo.device === next.device;
    }

    function videoConstraints() {
        return {
            height: { ideal: Number(live.height?.value || live.config.height || 1080) },
            frameRate: { ideal: Number(live.fps?.value || live.config.fps || 60) },
        };
    }

    function cameraConstraints() {
        var constraints = videoConstraints();
        if (live.camera?.value) constraints.deviceId = { exact: live.camera.value };
        return constraints;
    }

    function setRunning(running) {
        if (live.start) live.start.disabled = running;
        if (live.stop) live.stop.disabled = !running;
    }

    function statusText() {
        var label = live.source?.value === 'camera' ? 'Camera' : 'Screen';
        return label + (live.mic?.checked ? ' and microphone' : '') + ' active.';
    }

    function updateViewerCount(count) {
        if (live.viewerCount) {
            live.viewerCount.textContent = count + (count === 1 ? ' viewer' : ' viewers');
        }
    }

    async function loadCameras() {
        if (!live.camera || !navigator.mediaDevices?.enumerateDevices) return;
        var selected = live.camera.value;
        var devices = (await navigator.mediaDevices.enumerateDevices())
            .filter(function (item) { return item.kind === 'videoinput'; });
        live.camera.innerHTML = devices.map(cameraOption).join('');
        if (selected) live.camera.value = selected;
        syncSourceUi();
    }

    function cameraOption(item, index) {
        var label = item.label || 'Camera ' + (index + 1);
        return '<option value="' + escapeHtml(item.deviceId) + '">' + escapeHtml(label) + '</option>';
    }

    function syncSourceUi() {
        if (live.camera) live.camera.disabled = live.source?.value !== 'camera';
        if (live.refresh) live.refresh.disabled = live.source?.value !== 'camera';
    }

    function escapeHtml(value) {
        return String(value).replace(/[&<>"]/g, function (ch) {
            return ({ '&': '&amp;', '<': '&lt;', '>': '&gt;', '"': '&quot;' })[ch];
        });
    }
})();
