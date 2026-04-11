var deleteState = null;

function createNote() {
    fetch('/resources/notes', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ body: defaultNewNoteBody() })
    })
        .then(function (response) {
            if (!response.ok) throw new Error('create failed');
            return response.json();
        })
        .then(function (note) { window.location.href = '/' + note.id; })
        .catch(function () { alert('Failed to create note'); });
}

function deleteResource(button, id) {
    if (!button || !id) return;
    if (isDeleteArmed(button, id)) return performDelete(button, id);
    resetDeleteState();
    deleteState = {
        button: button,
        id: id,
        label: button.textContent.trim(),
        timer: window.setTimeout(resetDeleteState, 4000)
    };
    button.textContent = 'Press again to delete';
}

function isDeleteArmed(button, id) {
    return !!deleteState && deleteState.button === button && deleteState.id === id;
}

function performDelete(button, id) {
    resetDeleteState();
    button.disabled = true;
    button.textContent = 'Deleting...';
    fetch('/resources/' + id, { method: 'DELETE' })
        .then(function (response) {
            if (!response.ok) throw new Error('delete failed');
            window.location.href = '/admin';
        })
        .catch(function () { alert('Failed to delete resource'); });
}

function resetDeleteState() {
    if (!deleteState) return;
    window.clearTimeout(deleteState.timer);
    deleteState.button.textContent = deleteState.label;
    deleteState = null;
}

function defaultNewNoteBody() {
    return '# ' + localMinuteStamp() + '\n';
}

function localMinuteStamp() {
    var date = new Date();
    return [
        date.getFullYear(),
        pad(date.getMonth() + 1),
        pad(date.getDate())
    ].join('-') + ' ' + [pad(date.getHours()), pad(date.getMinutes())].join(':');
}

function pad(value) {
    return String(value).padStart(2, '0');
}
