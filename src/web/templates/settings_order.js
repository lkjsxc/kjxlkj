(function () {
    var list = document.querySelector('[data-settings-order-list]');
    if (!list || list.children.length < 2) return;

    var dragging = null;

    Array.from(list.children).forEach(function (item) {
        item.addEventListener('dragstart', function (event) {
            dragging = item;
            item.classList.add('dragging');
            event.dataTransfer.effectAllowed = 'move';
            event.dataTransfer.setData('text/plain', item.textContent.trim());
        });

        item.addEventListener('dragover', function (event) {
            if (!dragging || dragging === item) return;
            event.preventDefault();
            var rect = item.getBoundingClientRect();
            var after = event.clientY - rect.top > rect.height / 2;
            list.insertBefore(dragging, after ? item.nextSibling : item);
        });

        item.addEventListener('drop', function (event) {
            event.preventDefault();
            syncOrder();
        });

        item.addEventListener('dragend', function () {
            item.classList.remove('dragging');
            dragging = null;
            syncOrder();
        });
    });

    list.addEventListener('dragover', function (event) {
        if (!dragging || event.target !== list || list.lastElementChild === dragging) return;
        event.preventDefault();
        list.appendChild(dragging);
    });

    syncOrder();

    function syncOrder() {
        Array.from(list.querySelectorAll('[data-settings-order-item]')).forEach(function (item, index) {
            var value = String(index + 1);
            var input = item.querySelector('input[type="hidden"][name$="_position"]');
            if (input) input.value = value;
        });
    }
})();
