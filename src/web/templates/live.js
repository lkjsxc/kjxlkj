(function () {
    var live = window.kjxlkjLive;
    if (!live) return;

    if (live.source) {
        live.source.addEventListener('change', function () {
            live.syncSourceUi();
            live.loadCameras?.();
            live.applyLiveChange();
        });
    }
    [live.cameraFacing, live.camera, live.height, live.fps, live.mic].forEach(function (control) {
        if (control) control.addEventListener('change', live.applyLiveChange);
    });
    if (live.start) live.start.addEventListener('click', live.startBroadcast);
    if (live.stop) {
        live.stop.addEventListener('click', function () {
            live.stopBroadcast(true);
        });
    }
    if (live.role === 'viewer') live.connect('viewer');
    if (live.role === 'broadcaster') live.initCameraDevices?.();
    window.addEventListener('pagehide', live.cleanup, { once: true });
    live.app.registerCleanup?.(live.cleanup);
})();
