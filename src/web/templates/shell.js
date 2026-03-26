(function () {
    function formatLocalTimes() {
        var formatter = new Intl.DateTimeFormat(undefined, {
            year: 'numeric',
            month: '2-digit',
            day: '2-digit',
            hour: '2-digit',
            minute: '2-digit',
            hour12: false,
        });
        document.querySelectorAll('.local-time').forEach(function (node) {
            var iso = node.getAttribute('datetime') || node.dataset.utc;
            if (!iso) return;
            var date = new Date(iso);
            if (Number.isNaN(date.getTime())) return;
            var parts = formatter.formatToParts(date);
            var map = Object.fromEntries(parts.filter(function (part) { return part.type !== 'literal'; }).map(function (part) { return [part.type, part.value]; }));
            node.textContent = [map.year, map.month, map.day].join('-') + ' ' + map.hour + ':' + map.minute;
        });
    }
    formatLocalTimes();
})();
