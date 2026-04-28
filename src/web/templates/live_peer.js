(function () {
    var live = window.kjxlkjLive;
    if (!live) return;
    live.connect = connect;
    live.publishOffer = publishOffer;

    function send(message) {
        if (live.ws && live.ws.readyState === WebSocket.OPEN) live.ws.send(JSON.stringify(message));
    }

    function connect(nextRole) {
        live.ws = new WebSocket((location.protocol === 'https:' ? 'wss://' : 'ws://') + location.host + '/live/ws');
        live.ws.addEventListener('open', function () {
            console.info('kjxlkj live websocket open', nextRole);
            send({ type: 'hello', role: nextRole });
            if (nextRole === 'broadcaster' && live.localStream) publishOffer();
        });
        live.ws.addEventListener('message', function (event) { onMessage(JSON.parse(event.data)); });
        live.ws.addEventListener('close', function () {
            console.info('kjxlkj live websocket closed', live.role);
            if (live.role === 'viewer' && !live.closed) resetViewer('Connection closed.');
            if (live.role === 'broadcaster' && !live.closed) live.stopBroadcast(false);
        });
    }

    async function onMessage(message) {
        if (message.type === 'error') return live.setStatus('Live unavailable', message.message);
        if (message.type === 'viewer_count') return live.updateViewerCount(message.count);
        if (message.type === 'stream_started' && live.role === 'viewer') return startViewer();
        if (message.type === 'stream_ended') return resetViewer('Broadcast ended.');
        if (message.type === 'answer') return receiveAnswer(message);
        if (message.type === 'ice') return receiveIce(message);
    }

    function newPeer() {
        var peer = new RTCPeerConnection({ iceServers: [] });
        peer.onicecandidate = function (event) {
            if (!event.candidate) return;
            console.debug('kjxlkj live local ICE candidate', event.candidate.candidate);
            if (live.sentOffer) send({ type: 'ice', candidate: event.candidate });
            else live.localIce.push(event.candidate);
        };
        peer.oniceconnectionstatechange = function () {
            console.info('kjxlkj live ICE state', peer.iceConnectionState);
            if (peer.iceConnectionState === 'failed') failPeer('Live connection failed.');
            if (peer.iceConnectionState === 'disconnected') {
                if (live.role === 'viewer') live.setStatus('Reconnecting live', 'Media connection interrupted.');
            }
        };
        peer.onconnectionstatechange = function () {
            console.info('kjxlkj live peer state', peer.connectionState);
            if (peer.connectionState === 'failed') failPeer('Live connection failed.');
        };
        if (live.role === 'viewer') {
            peer.ontrack = function (event) {
                console.info('kjxlkj live track received', event.track.kind);
                live.video.srcObject = event.streams[0];
                live.video.muted = false;
                live.setStatus('Live now', 'Broadcast is playing.');
            };
        }
        return peer;
    }

    async function publishOffer() {
        if (!live.localStream || !live.ws || live.ws.readyState !== WebSocket.OPEN) return;
        replacePeer();
        live.localStream.getTracks().forEach(function (track) {
            live.peer.addTrack(track, live.localStream);
        });
        preferPublisherCodecs();
        await negotiate('publish_offer');
    }

    async function startViewer() {
        live.setStatus('Connecting live', 'Preparing video.');
        if (!live.ws || live.ws.readyState !== WebSocket.OPEN) return connect('viewer');
        replacePeer();
        preferCodec(live.peer.addTransceiver('video', { direction: 'recvonly' }), 'video/VP8');
        preferCodec(live.peer.addTransceiver('audio', { direction: 'recvonly' }), 'audio/opus');
        await negotiate('view_offer');
    }

    async function negotiate(type) {
        live.sentOffer = false;
        live.localIce = [];
        var peer = live.peer;
        await peer.setLocalDescription(await peer.createOffer());
        console.info('kjxlkj live offer created', type);
        send({ type: type, sdp: peer.localDescription });
        live.sentOffer = true;
        live.localIce.splice(0).forEach(function (candidate) {
            send({ type: 'ice', candidate: candidate });
        });
    }

    async function receiveAnswer(message) {
        if (live.peer) {
            await live.peer.setRemoteDescription(message.sdp);
            console.info('kjxlkj live answer applied');
        }
    }

    async function receiveIce(message) {
        if (!message.candidate || !live.peer) return;
        try {
            await live.peer.addIceCandidate(message.candidate);
            console.debug('kjxlkj live remote ICE candidate applied', message.candidate.candidate);
        }
        catch (_) { failPeer('Live connection failed.'); }
    }

    function replacePeer() {
        live.closePeers();
        live.peer = newPeer();
        live.sentOffer = false;
        live.localIce = [];
    }

    function preferPublisherCodecs() {
        live.peer.getTransceivers().forEach(function (transceiver) {
            if (transceiver.sender?.track?.kind === 'video') preferCodec(transceiver, 'video/VP8');
            if (transceiver.sender?.track?.kind === 'audio') preferCodec(transceiver, 'audio/opus');
        });
    }

    function preferCodec(transceiver, mime) {
        if (!transceiver?.setCodecPreferences || !RTCRtpSender.getCapabilities) return;
        var codecs = RTCRtpSender.getCapabilities(mime.split('/')[0])?.codecs || [];
        var preferred = codecs.filter(function (codec) {
            return codec.mimeType.toLowerCase() === mime.toLowerCase();
        });
        if (preferred.length) transceiver.setCodecPreferences(preferred);
    }

    function failPeer(text) {
        live.closePeers();
        if (live.role === 'viewer') resetViewer(text);
    }

    function resetViewer(text) {
        live.closePeers();
        live.video.srcObject = null;
        live.setStatus('Waiting for broadcast', text || 'No active stream.');
    }
})();
