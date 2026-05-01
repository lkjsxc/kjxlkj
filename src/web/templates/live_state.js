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
        cameraFacing: root.querySelector('[data-live-camera-facing]'),
        camera: root.querySelector('[data-live-camera]'),
        height: root.querySelector('[data-live-height]'),
        fps: root.querySelector('[data-live-fps]'),
        mic: root.querySelector('[data-live-mic]'),
        viewerCount: root.querySelector('[data-live-viewer-count]'),
        ws: null,
        peer: null,
        sentOffer: false,
        localIce: [],
        localStream: null,
        activeVideo: {},
        closed: false,
    };
    live.config = readConfig();
    live.setStatus = setStatus;
    live.selectedVideo = selectedVideo;
    live.sameVideo = sameVideo;
    live.videoConstraints = videoConstraints;
    live.cameraConstraints = cameraConstraints;
    live.cameraFallbackConstraints = cameraFallbackConstraints;
    live.setRunning = setRunning;
    live.statusText = statusText;
    live.updateViewerCount = updateViewerCount;
    live.syncSourceUi = syncSourceUi;

    function readConfig() {
        try { return JSON.parse(document.getElementById('live-config').textContent || '{}'); }
        catch (error) {
            console.warn('kjxlkj live config parse failed', error);
            return {};
        }
    }

    function setStatus(label, text) {
        live.state.textContent = label;
        live.detail.textContent = text || '';
    }

    function selectedVideo() {
        return {
            source: live.source?.value || live.config.source || 'screen',
            facing: live.cameraFacing?.value || live.config.cameraFacing || 'environment',
            device: live.camera?.value || '',
        };
    }

    function sameVideo(next) {
        return live.activeVideo.source === next.source &&
            live.activeVideo.facing === next.facing &&
            live.activeVideo.device === next.device;
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
        else constraints.facingMode = { ideal: live.cameraFacing?.value || live.config.cameraFacing || 'environment' };
        return constraints;
    }

    function cameraFallbackConstraints() {
        var constraints = videoConstraints();
        if (live.camera?.value) constraints.deviceId = { exact: live.camera.value };
        return constraints;
    }

    function setRunning(running) {
        if (live.start) live.start.disabled = running;
        if (live.stop) live.stop.disabled = !running;
    }

    function statusText() {
        var label = live.source?.value === 'camera' ? cameraLabel() : 'Screen';
        return label + (live.mic?.checked ? ' and microphone' : '') + ' active.';
    }

    function cameraLabel() {
        return (live.cameraFacing?.value || live.config.cameraFacing) === 'user'
            ? 'Front camera'
            : 'Rear camera';
    }

    function updateViewerCount(count) {
        if (live.viewerCount) {
            live.viewerCount.textContent = count + (count === 1 ? ' viewer' : ' viewers');
        }
    }

    function syncSourceUi() {
        var disabled = live.source?.value !== 'camera';
        if (live.cameraFacing) live.cameraFacing.disabled = disabled;
        if (live.camera) live.camera.disabled = disabled;
    }
})();
