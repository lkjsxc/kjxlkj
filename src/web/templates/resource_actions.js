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

function deleteResource(id) {
    if (!confirm('Delete this resource?')) return;
    fetch('/resources/' + id, { method: 'DELETE' })
        .then(function (response) {
            if (!response.ok) throw new Error('delete failed');
            window.location.href = '/admin';
        })
        .catch(function () { alert('Failed to delete resource'); });
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
