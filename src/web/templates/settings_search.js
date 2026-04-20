(function () {
    var input = document.querySelector('[data-settings-search-input]');
    var empty = document.querySelector('[data-settings-search-empty]');
    var rows = Array.from(document.querySelectorAll('[data-settings-row]'));
    if (!input || !rows.length) return;

    input.addEventListener('input', applyFilter);
    applyFilter();

    function applyFilter() {
        var query = normalize(input.value);
        var visible = 0;
        rows.forEach(function (row) {
            var match = !query || normalize(row.dataset.settingsSearch || row.textContent).includes(query);
            row.hidden = !match;
            if (match) visible += 1;
        });
        if (empty) empty.hidden = !query || visible > 0;
    }

    function normalize(value) {
        return (value || '').replace(/\s+/g, ' ').trim().toLowerCase();
    }
})();
