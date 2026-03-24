let editMode = false;
let currentId = '';

function getToken() {
    return document.querySelector('meta[name="admin-token"]')?.content || '';
}

function createRecord() {
    editMode = false;
    currentId = '';
    document.getElementById('modal-title').textContent = 'Create Record';
    document.getElementById('record-id').value = '';
    document.getElementById('record-id').disabled = false;
    document.getElementById('record-title').value = '';
    document.getElementById('record-body').value = '';
    document.getElementById('record-tags').value = '';
    document.getElementById('modal').classList.remove('hidden');
}

function editRecord(id) {
    editMode = true;
    currentId = id;
    fetch('/v1/records/' + id)
        .then(r => r.json())
        .then(record => {
            document.getElementById('modal-title').textContent = 'Edit Record';
            document.getElementById('record-id').value = record.id;
            document.getElementById('record-id').disabled = true;
            document.getElementById('record-title').value = record.title;
            document.getElementById('record-body').value = record.body;
            document.getElementById('record-tags').value = record.tags.join(', ');
            document.getElementById('modal').classList.remove('hidden');
        });
}

function deleteRecord(id) {
    if (!confirm('Delete record "' + id + '"?')) return;
    fetch('/v1/records/' + id, {
        method: 'DELETE',
        headers: { 'x-admin-token': prompt('Enter admin token:') }
    }).then(r => {
        if (r.ok) location.reload();
        else alert('Delete failed');
    });
}

function closeModal() {
    document.getElementById('modal').classList.add('hidden');
}

document.getElementById('record-form').addEventListener('submit', function(e) {
    e.preventDefault();
    const id = editMode ? currentId : document.getElementById('record-id').value;
    const data = {
        title: document.getElementById('record-title').value,
        body: document.getElementById('record-body').value,
        tags: document.getElementById('record-tags').value.split(',').map(t => t.trim()).filter(t => t)
    };
    const token = prompt('Enter admin token:');
    fetch('/v1/records/' + id, {
        method: 'PUT',
        headers: { 'Content-Type': 'application/json', 'x-admin-token': token },
        body: JSON.stringify(data)
    }).then(r => {
        if (r.ok) location.reload();
        else r.json().then(err => alert(err.message || 'Save failed'));
    });
});

document.getElementById('modal').addEventListener('click', function(e) {
    if (e.target === this) closeModal();
});
