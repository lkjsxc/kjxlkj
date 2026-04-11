(function () {
    setupUnsavedPrompt();

    var list = document.querySelector('[data-settings-order-list]');
    if (!list || list.children.length < 2) return;

    var dragging = null;

    Array.from(list.children).forEach(function (item) {
        bindMoveButtons(item);
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

    function bindMoveButtons(item) {
        Array.from(item.querySelectorAll('[data-settings-move]')).forEach(function (button) {
            button.addEventListener('click', function () {
                if (button.disabled) return;
                if (button.dataset.settingsMove === 'up' && item.previousElementSibling) {
                    list.insertBefore(item, item.previousElementSibling);
                    syncOrder();
                }
                if (button.dataset.settingsMove === 'down' && item.nextElementSibling) {
                    list.insertBefore(item, item.nextElementSibling.nextElementSibling);
                    syncOrder();
                }
            });
        });
    }

    function syncOrder() {
        var items = Array.from(list.querySelectorAll('[data-settings-order-item]'));
        items.forEach(function (item, index) {
            var value = String(index + 1);
            var input = item.querySelector('input[type="hidden"][name$="_position"]');
            if (input) input.value = value;
        });
        items.forEach(function (item, index) {
            var upButton = item.querySelector('[data-settings-move="up"]');
            var downButton = item.querySelector('[data-settings-move="down"]');
            if (upButton) upButton.disabled = index === 0;
            if (downButton) downButton.disabled = index === items.length - 1;
        });
    }

    function setupUnsavedPrompt() {
        var forms = Array.from(document.querySelectorAll([
            'form[action="/admin/settings"]',
            'form[action="/admin/password"]',
            'form[action="/admin/site-icon"]'
        ].join(','))).map(function (form) {
            return { form: form, initial: formSnapshot(form), submitted: false };
        });
        if (!forms.length) return;
        var previousGuard = window.kjxlkj?.beforeNavigate;
        var lastCanceledAt = 0;
        forms.forEach(function (state) {
            state.form.addEventListener('submit', function () { state.submitted = true; });
        });
        var handler = function (event) {
            if (!hasDirtyForms(forms)) return;
            event.preventDefault();
            event.returnValue = '';
        };
        var guard = async function (url, historyMode) {
            if (typeof previousGuard === 'function' && !await previousGuard(url, historyMode)) {
                return false;
            }
            if (!hasDirtyForms(forms)) return true;
            if (Date.now() - lastCanceledAt < 2000) return false;
            var confirmed = window.confirm('Leave settings without saving?');
            if (!confirmed) lastCanceledAt = Date.now();
            return confirmed;
        };
        if (window.kjxlkj) window.kjxlkj.beforeNavigate = guard;
        window.addEventListener('beforeunload', handler);
        if (window.kjxlkj) {
            window.kjxlkj.settingsBeforeUnload = handler;
            window.kjxlkj.registerCleanup?.(function () {
                window.removeEventListener('beforeunload', handler);
                if (window.kjxlkj?.beforeNavigate === guard) {
                    if (typeof previousGuard === 'function') {
                        window.kjxlkj.beforeNavigate = previousGuard;
                    } else {
                        delete window.kjxlkj.beforeNavigate;
                    }
                }
                if (window.kjxlkj?.settingsBeforeUnload === handler) {
                    delete window.kjxlkj.settingsBeforeUnload;
                }
            });
        }
    }

    function hasDirtyForms(forms) {
        return forms.some(function (state) {
            return !state.submitted && formSnapshot(state.form) !== state.initial;
        });
    }

    function formSnapshot(form) {
        return new URLSearchParams(new FormData(form)).toString();
    }
})();
