(function () {
    var input = document.querySelector('[data-settings-search-input]');
    var empty = document.querySelector('[data-settings-search-empty]');
    var sections = Array.from(document.querySelectorAll('.settings-section'));
    if (!input || !sections.length) return;

    input.addEventListener('input', applyFilter);
    applyFilter();

    function applyFilter() {
        var query = normalize(input.value);
        var visible = 0;
        sections.forEach(function (section) {
            if (section.classList.contains('settings-search-section')) return;
            var items = Array.from(section.querySelectorAll('[data-settings-item]'));
            if (!items.length) return;
            var sectionMatch = normalize(section.querySelector('.section-head')?.textContent).includes(query);
            var matches = 0;
            items.forEach(function (item) {
                var match = !query || sectionMatch || normalize(item.dataset.settingsSearch || item.textContent).includes(query);
                item.hidden = !match;
                if (match) matches += 1;
            });
            section.hidden = matches === 0;
            visible += matches;
        });
        if (empty) empty.hidden = !query || visible > 0;
    }

    function normalize(value) {
        return (value || '').replace(/\s+/g, ' ').trim().toLowerCase();
    }
})();
