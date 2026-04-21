(function () {
    var root = document.querySelector('[data-live-root]');
    if (!root) return;
    var role = root.getAttribute('data-live-role');
    var video = root.querySelector('[data-live-video]');
    var state = root.querySelector('[data-live-state]');
    var detail = root.querySelector('[data-live-detail]');
    var start = root.querySelector('[data-live-start]');
    var stop = root.querySelector('[data-live-stop]');
    var ws = null;
    var localStream = null;
    var peers = {};

    function iceServers() {
        try { return JSON.parse(document.getElementById('live-ice-servers').textContent || '[]'); }
        catch (_) { return []; }
    }

    function setStatus(label, text) {
        state.textContent = label;
        detail.textContent = text || '';
    }

    function connect(nextRole) {
        ws = new WebSocket((location.protocol === 'https:' ? 'wss://' : 'ws://') + location.host + '/live/ws');
        ws.addEventListener('open', function () { send({ type: 'hello', role: nextRole }); });
        ws.addEventListener('message', function (event) { onMessage(JSON.parse(event.data)); });
        ws.addEventListener('close', function () {
            if (role === 'viewer') setStatus('Waiting for broadcast', 'Connection closed.');
            if (role === 'broadcaster') stopBroadcast(false);
        });
    }

    function send(message) {
        if (ws && ws.readyState === WebSocket.OPEN) ws.send(JSON.stringify(message));
    }

    async function onMessage(message) {
        if (message.type === 'error') return setStatus('Live unavailable', message.message);
        if (message.type === 'stream_started' && role === 'viewer') return startViewer();
        if (message.type === 'stream_ended') return resetViewer('Broadcast ended.');
        if (role === 'broadcaster' && message.type === 'viewer_ready') return offerViewer(message.viewer_id);
        if (role === 'broadcaster' && message.type === 'answer') return receiveAnswer(message);
        if (role === 'viewer' && message.type === 'offer') return receiveOffer(message);
        if (message.type === 'ice') return receiveIce(message);
    }

    async function startBroadcast() {
        try {
            var display = await navigator.mediaDevices.getDisplayMedia({ video: true });
            var audio = await navigator.mediaDevices.getUserMedia({ audio: true });
            localStream = new MediaStream(display.getVideoTracks().concat(audio.getAudioTracks()));
            video.srcObject = localStream;
            video.muted = true;
            connect('broadcaster');
            start.disabled = true;
            stop.disabled = false;
            setStatus('Broadcasting live', 'Screen and microphone are active.');
        } catch (error) {
            setStatus('Live unavailable', error.message || 'Could not start broadcast.');
        }
    }

    function stopBroadcast(closeSocket) {
        Object.values(peers).forEach(function (peer) { peer.close(); });
        peers = {};
        if (localStream) localStream.getTracks().forEach(function (track) { track.stop(); });
        localStream = null;
        video.srcObject = null;
        if (closeSocket !== false && ws) ws.close();
        if (start) start.disabled = false;
        if (stop) stop.disabled = true;
        setStatus('Waiting for broadcast', 'No active stream.');
    }

    function newPeer(viewerId) {
        var peer = new RTCPeerConnection({ iceServers: iceServers() });
        peer.onicecandidate = function (event) {
            if (event.candidate) send({ type: 'ice', viewer_id: viewerId, candidate: event.candidate });
        };
        if (role === 'viewer') {
            peer.ontrack = function (event) {
                video.srcObject = event.streams[0];
                video.muted = false;
                setStatus('Live now', 'Broadcast is playing.');
            };
        }
        return peer;
    }

    async function offerViewer(viewerId) {
        if (!localStream) return;
        var peer = peers[viewerId] = newPeer(viewerId);
        localStream.getTracks().forEach(function (track) { peer.addTrack(track, localStream); });
        await peer.setLocalDescription(await peer.createOffer());
        send({ type: 'offer', viewer_id: viewerId, sdp: peer.localDescription });
    }

    async function receiveAnswer(message) {
        if (peers[message.viewer_id]) await peers[message.viewer_id].setRemoteDescription(message.sdp);
    }

    function startViewer() {
        setStatus('Connecting live', 'Preparing video.');
        if (!ws || ws.readyState !== WebSocket.OPEN) connect('viewer');
    }

    async function receiveOffer(message) {
        var peer = peers[message.viewer_id] = newPeer(message.viewer_id);
        await peer.setRemoteDescription(message.sdp);
        await peer.setLocalDescription(await peer.createAnswer());
        send({ type: 'answer', viewer_id: message.viewer_id, sdp: peer.localDescription });
    }

    async function receiveIce(message) {
        var peer = peers[message.viewer_id];
        if (peer && message.candidate) await peer.addIceCandidate(message.candidate);
    }

    function resetViewer(text) {
        Object.values(peers).forEach(function (peer) { peer.close(); });
        peers = {};
        video.srcObject = null;
        setStatus('Waiting for broadcast', text || 'No active stream.');
    }

    if (start) start.addEventListener('click', startBroadcast);
    if (stop) stop.addEventListener('click', function () { stopBroadcast(true); });
    if (role === 'viewer') connect('viewer');
})();
