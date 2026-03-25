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
    status.dataset.state = 'saving';
    
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
            status.dataset.state = 'saved';
            setTimeout(() => { status.textContent = ''; }, 2000);
        } else {
            status.textContent = 'Save failed';
            status.dataset.state = 'error';
        }
    })
    .catch(() => {
        status.textContent = 'Save failed';
        status.dataset.state = 'error';
    });
}

function syncVisibilityHint() {
    var hint = document.getElementById('visibility-hint');
    if (!hint) return;
    hint.textContent = isPrivate ? 'Admin-only' : 'Guest-readable';
}

function togglePublic() {
    var checkbox = document.getElementById('public-toggle');
    isPrivate = !checkbox.checked;
    syncVisibilityHint();
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
