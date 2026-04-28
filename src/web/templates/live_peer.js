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
        peer._viewerId = viewerId;
        peer._pendingIce = live.pendingIce[viewerId] || [];
        delete live.pendingIce[viewerId];
        peer.onicecandidate = function (event) {
            if (event.candidate) send({ type: 'ice', viewer_id: viewerId, candidate: event.candidate });
        };
        peer.oniceconnectionstatechange = function () {
            if (peer.iceConnectionState === 'failed') failPeer(viewerId, 'Live connection failed.');
            if (peer.iceConnectionState === 'disconnected') {
                if (live.role === 'viewer') live.setStatus('Reconnecting live', 'Media connection interrupted.');
            }
        };
        peer.onconnectionstatechange = function () {
            if (peer.connectionState === 'failed') failPeer(viewerId, 'Live connection failed.');
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
        await addPendingIce(peer);
        await peer.setLocalDescription(await peer.createAnswer());
        send({ type: 'answer', viewer_id: message.viewer_id, sdp: peer.localDescription });
    }

    async function receiveAnswer(message) {
        if (live.peers[message.viewer_id]) {
            var peer = live.peers[message.viewer_id];
            await peer.setRemoteDescription(message.sdp);
            await addPendingIce(peer);
        }
    }

    async function receiveIce(message) {
        var peer = live.peers[message.viewer_id];
        if (!message.candidate) return;
        if (!peer) return queueIce(message.viewer_id, message.candidate);
        if (!peer.remoteDescription) return peer._pendingIce.push(message.candidate);
        await addIce(peer, message.candidate);
    }

    function queueIce(viewerId, candidate) {
        live.pendingIce[viewerId] = live.pendingIce[viewerId] || [];
        live.pendingIce[viewerId].push(candidate);
    }

    async function addPendingIce(peer) {
        var candidates = peer._pendingIce.splice(0);
        for (var i = 0; i < candidates.length; i += 1) await addIce(peer, candidates[i]);
    }

    async function addIce(peer, candidate) {
        try {
            await peer.addIceCandidate(candidate);
        } catch (_) {
            failPeer(peer._viewerId, 'Live connection failed.');
        }
    }

    function failPeer(viewerId, text) {
        var peer = live.peers[viewerId];
        if (peer) peer.close();
        delete live.peers[viewerId];
        if (live.role === 'viewer') resetViewer(text);
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
