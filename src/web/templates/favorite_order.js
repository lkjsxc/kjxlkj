(function () {
    var list = document.querySelector('[data-favorite-order]');
    var errorNode = document.querySelector('[data-favorite-order-error]');
    if (!list || list.children.length < 2) return;

    var dragging = null;
    var saving = false;

    Array.from(list.children).forEach(function (item) {
        item.addEventListener('dragstart', function (event) {
            dragging = item;
            item.classList.add('dragging');
            event.dataTransfer.effectAllowed = 'move';
            event.dataTransfer.setData('text/plain', item.dataset.favoriteId || '');
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
            if (!dragging) return;
            saveOrder();
        });

        item.addEventListener('dragend', function () {
            item.classList.remove('dragging');
            dragging = null;
        });
    });

    list.addEventListener('dragover', function (event) {
        if (!dragging) return;
        event.preventDefault();
        if (event.target === list && list.lastElementChild !== dragging) {
            list.appendChild(dragging);
        }
    });

    async function saveOrder() {
        if (saving) return;
        saving = true;
        clearError();
        try {
            var response = await fetch('/records/favorites/order', {
                method: 'PUT',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({ ids: currentIds() }),
            });
            if (!response.ok) throw new Error(await responseMessage(response));
        } catch (error) {
            showError(error.message || 'Favorite order save failed.');
        } finally {
            saving = false;
        }
    }

    function currentIds() {
        return Array.from(list.querySelectorAll('[data-favorite-id]')).map(function (item) {
            return item.dataset.favoriteId;
        });
    }

    async function responseMessage(response) {
        try {
            var body = await response.json();
            return body.message || 'Favorite order save failed.';
        } catch {
            return 'Favorite order save failed.';
        }
    }

    function clearError() {
        if (errorNode) errorNode.textContent = '';
    }

    function showError(message) {
        if (errorNode) errorNode.textContent = message;
    }
})();
