(function () {
    var live = window.kjxlkjLive;
    if (!live) return;
    live.startBroadcast = startBroadcast;
    live.applyLiveChange = applyLiveChange;
    live.stopBroadcast = stopBroadcast;
    live.cleanup = cleanup;
    live.closePeers = closePeers;

    async function startBroadcast() {
        try {
            live.localStream = new MediaStream();
            live.localStream.addTrack(await captureVideo());
            if (live.mic?.checked) await addAudioTrack();
            live.video.srcObject = live.localStream;
            live.video.muted = true;
            live.connect('broadcaster');
            live.setRunning(true);
            live.setStatus('Broadcasting live', live.statusText());
        } catch (error) {
            stopTracks(live.localStream);
            live.localStream = null;
            live.setStatus('Live unavailable', error.message || 'Could not start broadcast.');
        }
    }

    async function applyLiveChange() {
        if (!live.localStream) return live.syncSourceUi();
        try {
            await syncVideoTrack();
            await syncAudioTrack();
            await live.publishOffer();
            live.setStatus('Broadcasting live', live.statusText());
        } catch (error) {
            live.setStatus('Live change failed', error.message || 'Existing stream kept.');
            live.syncSourceUi();
        }
    }

    async function syncVideoTrack() {
        var current = live.localStream.getVideoTracks()[0];
        var next = live.selectedVideo();
        if (current && live.sameVideo(next) && await applyVideoConstraints(current)) return;
        var replacement = await captureVideo();
        if (current) {
            live.localStream.removeTrack(current);
            current.stop();
        }
        live.localStream.addTrack(replacement);
        if (live.peer) {
            var sender = live.peer.getSenders().find(function (item) { return item.track?.kind === 'video'; });
            if (sender) sender.replaceTrack(replacement);
            else live.peer.addTrack(replacement, live.localStream);
        }
    }

    async function applyVideoConstraints(track) {
        try {
            await track.applyConstraints(live.videoConstraints());
            return true;
        } catch (_) {
            return false;
        }
    }

    async function syncAudioTrack() {
        var tracks = live.localStream.getAudioTracks();
        if (live.mic?.checked && tracks.length) return;
        if (live.mic?.checked) return addPeerAudioTrack(await captureAudio());
        tracks.forEach(function (track) {
            live.localStream.removeTrack(track);
            track.stop();
        });
        if (live.peer) {
            live.peer.getSenders()
                .filter(function (sender) { return sender.track?.kind === 'audio'; })
                .forEach(function (sender) { sender.replaceTrack(null); });
        }
    }

    async function captureVideo() {
        var next = live.selectedVideo();
        var stream = next.source === 'camera'
            ? await navigator.mediaDevices.getUserMedia({ video: live.cameraConstraints(), audio: false })
            : await navigator.mediaDevices.getDisplayMedia({ video: live.videoConstraints(), audio: false });
        var track = stream.getVideoTracks()[0];
        live.activeVideo = next;
        track.addEventListener('ended', function () {
            if (live.localStream?.getVideoTracks()[0] === track) stopBroadcast(true);
        });
        return track;
    }

    async function captureAudio() {
        var stream = await navigator.mediaDevices.getUserMedia({ audio: true, video: false });
        return stream.getAudioTracks()[0];
    }

    async function addAudioTrack() {
        live.localStream.addTrack(await captureAudio());
    }

    function addPeerAudioTrack(track) {
        live.localStream.addTrack(track);
        if (live.peer) live.peer.addTrack(track, live.localStream);
    }

    function stopBroadcast(closeSocket) {
        closePeers();
        stopTracks(live.localStream);
        live.localStream = null;
        live.video.srcObject = null;
        if (closeSocket !== false && live.ws) live.ws.close();
        live.setRunning(false);
        live.setStatus('Waiting for broadcast', 'No active stream.');
    }

    function closePeers() {
        if (live.peer) live.peer.close();
        live.peer = null;
        live.sentOffer = false;
        live.localIce = [];
    }

    function stopTracks(stream) {
        if (stream) stream.getTracks().forEach(function (track) { track.stop(); });
    }

    function cleanup() {
        live.closed = true;
        if (live.role === 'broadcaster') stopBroadcast(true);
        else {
            closePeers();
            if (live.ws) live.ws.close();
        }
    }
})();
