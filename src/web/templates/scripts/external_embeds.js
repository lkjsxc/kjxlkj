(function () {
    var app = window.kjxlkj = window.kjxlkj || {};
    app.hydrateExternalEmbeds = hydrateExternalEmbeds;
    hydrateExternalEmbeds();

    function hydrateExternalEmbeds(root) {
        var scope = root && root.querySelectorAll ? root : document;
        scope.querySelectorAll('.external-embed-social[data-embed-provider]:not([data-embed-hydrated])')
            .forEach(hydrateSocialEmbed);
    }

    function hydrateSocialEmbed(node) {
        var provider = node.dataset.embedProvider;
        var url = node.dataset.embedUrl;
        if (!provider || !url) return;
        node.dataset.embedHydrated = 'true';
        if (provider === 'x') return hydrateX(node, url);
        if (provider === 'instagram') return hydrateInstagram(node, url);
        if (provider === 'bluesky') return hydrateBluesky(node, url);
    }

    function hydrateX(node, url) {
        node.innerHTML = '<blockquote class="twitter-tweet"><a href="' + escapeAttr(url) + '"></a></blockquote>';
        loadScript('https://platform.twitter.com/widgets.js', function () {
            window.twttr?.widgets?.load?.(node);
        });
    }

    function hydrateInstagram(node, url) {
        node.innerHTML = '<blockquote class="instagram-media" data-instgrm-permalink="' + escapeAttr(url) + '"><a href="' + escapeAttr(url) + '"></a></blockquote>';
        loadScript('https://www.instagram.com/embed.js', function () {
            window.instgrm?.Embeds?.process?.();
        });
    }

    function hydrateBluesky(node, url) {
        var uri = blueskyUri(url);
        if (uri) node.innerHTML = '<blockquote class="bluesky-embed" data-bluesky-uri="' + escapeAttr(uri) + '"><a href="' + escapeAttr(url) + '"></a></blockquote>';
        loadScript('https://embed.bsky.app/static/embed.js', function () {});
    }

    function blueskyUri(url) {
        try {
            var path = new URL(url).pathname.split('/').filter(Boolean);
            if (path[0] === 'profile' && path[2] === 'post' && path[1] && path[3]) {
                return 'at://' + path[1] + '/app.bsky.feed.post/' + path[3];
            }
        } catch (_) {}
        return null;
    }

    function loadScript(src, callback) {
        var existing = document.querySelector('script[src="' + src + '"]');
        if (existing) {
            existing.addEventListener('load', callback, { once: true });
            callback();
            return;
        }
        var script = document.createElement('script');
        script.async = true;
        script.src = src;
        script.onload = callback;
        document.head.appendChild(script);
    }

    function escapeAttr(value) {
        return String(value)
            .replace(/&/g, '&amp;')
            .replace(/</g, '&lt;')
            .replace(/>/g, '&gt;')
            .replace(/"/g, '&quot;');
    }
})();
