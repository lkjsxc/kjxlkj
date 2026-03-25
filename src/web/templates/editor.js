// Note editor functionality

function createNote() {
    fetch('/records', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({})
    })
    .then(r => r.json())
    .then(note => {
        window.location.href = '/' + note.slug;
    })
    .catch(err => {
        alert('Failed to create note: ' + err.message);
    });
}

function saveNote() {
    if (typeof simplemde === 'undefined' || typeof currentSlug === 'undefined') return;
    
    var status = document.getElementById('save-status');
    status.textContent = 'Saving...';
    
    fetch('/records/' + currentSlug, {
        method: 'PUT',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
            body: simplemde.value(),
            is_private: isPrivate
        })
    })
    .then(r => {
        if (r.ok) {
            status.textContent = 'Saved';
            setTimeout(() => { status.textContent = ''; }, 2000);
        } else {
            status.textContent = 'Error saving';
        }
    })
    .catch(() => {
        status.textContent = 'Error saving';
    });
}

function togglePrivate() {
    var checkbox = document.getElementById('private-toggle');
    isPrivate = checkbox.checked;
    var label = checkbox.nextElementSibling;
    label.textContent = isPrivate ? 'Private' : 'Public';
    saveNote();
}

function deleteNote(slug) {
    if (!confirm('Delete this note?')) return;
    
    fetch('/records/' + slug, { method: 'DELETE' })
    .then(r => {
        if (r.ok) {
            window.location.href = '/admin';
        } else {
            alert('Failed to delete note');
        }
    });
}
