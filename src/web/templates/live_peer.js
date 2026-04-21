(function () {
    var live = window.kjxlkjLive;
    if (!live) return;
    live.connect = connect;
    live.renegotiateAll = renegotiateAll;

    function send(message) {
        if (live.ws && live.ws.readyState === WebSocket.OPEN) live.ws.send(JSON.stringify(message));
    }

    function connect(nextRole) {
        live.ws = new WebSocket((location.protocol === 'https:' ? 'wss://' : 'ws://') + location.host + '/live/ws');
        live.ws.addEventListener('open', function () { send({ type: 'hello', role: nextRole }); });
        live.ws.addEventListener('message', function (event) { onMessage(JSON.parse(event.data)); });
        live.ws.addEventListener('close', function () {
            if (live.role === 'viewer' && !live.closed) resetViewer('Connection closed.');
            if (live.role === 'broadcaster' && !live.closed) live.stopBroadcast(false);
        });
    }

    async function onMessage(message) {
        if (message.type === 'error') return live.setStatus('Live unavailable', message.message);
        if (message.type === 'viewer_count') return live.updateViewerCount(message.count);
        if (message.type === 'stream_started' && live.role === 'viewer') return startViewer();
        if (message.type === 'stream_ended') return resetViewer('Broadcast ended.');
        if (live.role === 'broadcaster' && message.type === 'viewer_ready') return offerViewer(message.viewer_id);
        if (live.role === 'broadcaster' && message.type === 'answer') return receiveAnswer(message);
        if (live.role === 'viewer' && message.type === 'offer') return receiveOffer(message);
        if (message.type === 'ice') return receiveIce(message);
    }

    function newPeer(viewerId) {
        var peer = new RTCPeerConnection({ iceServers: live.config.iceServers || [] });
        peer.onicecandidate = function (event) {
            if (event.candidate) send({ type: 'ice', viewer_id: viewerId, candidate: event.candidate });
        };
        if (live.role === 'viewer') {
            peer.ontrack = function (event) {
                live.video.srcObject = event.streams[0];
                live.video.muted = false;
                live.setStatus('Live now', 'Broadcast is playing.');
            };
        }
        return peer;
    }

    async function offerViewer(viewerId) {
        if (!live.localStream) return;
        var peer = live.peers[viewerId] = newPeer(viewerId);
        live.localStream.getTracks().forEach(function (track) {
            peer.addTrack(track, live.localStream);
        });
        await negotiate(peer, viewerId);
    }

    async function renegotiateAll() {
        await Promise.all(Object.entries(live.peers).map(function ([id, peer]) {
            return negotiate(peer, id);
        }));
    }

    async function negotiate(peer, viewerId) {
        await peer.setLocalDescription(await peer.createOffer());
        send({ type: 'offer', viewer_id: viewerId, sdp: peer.localDescription });
    }

    async function receiveOffer(message) {
        var peer = live.peers[message.viewer_id] || (live.peers[message.viewer_id] = newPeer(message.viewer_id));
        await peer.setRemoteDescription(message.sdp);
        await peer.setLocalDescription(await peer.createAnswer());
        send({ type: 'answer', viewer_id: message.viewer_id, sdp: peer.localDescription });
    }

    async function receiveAnswer(message) {
        if (live.peers[message.viewer_id]) {
            await live.peers[message.viewer_id].setRemoteDescription(message.sdp);
        }
    }

    async function receiveIce(message) {
        var peer = live.peers[message.viewer_id];
        if (peer && message.candidate) await peer.addIceCandidate(message.candidate);
    }

    function startViewer() {
        live.setStatus('Connecting live', 'Preparing video.');
        if (!live.ws || live.ws.readyState !== WebSocket.OPEN) connect('viewer');
    }

    function resetViewer(text) {
        live.closePeers();
        live.video.srcObject = null;
        live.setStatus('Waiting for broadcast', text || 'No active stream.');
    }
})();
