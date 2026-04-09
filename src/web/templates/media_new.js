document.getElementById('media-create-form')?.addEventListener('submit', async function (event) {
    event.preventDefault();
    var fileInput = document.getElementById('media-file-input');
    var aliasInput = document.getElementById('media-alias-input');
    var favoriteToggle = document.getElementById('media-favorite-toggle');
    var publicToggle = document.getElementById('media-public-toggle');
    var submit = document.getElementById('media-create-submit');
    var errorNode = document.getElementById('media-create-error');
    if (!fileInput?.files?.length) {
        showMediaCreateError('Select one image or video.');
        return;
    }
    var formData = new FormData();
    formData.append('file', fileInput.files[0]);
    if (aliasInput?.value.trim()) formData.append('alias', aliasInput.value.trim());
    formData.append('is_favorite', favoriteToggle?.checked ? 'true' : 'false');
    formData.append('is_private', publicToggle?.checked ? 'false' : 'true');
    if (submit) submit.disabled = true;
    if (errorNode) errorNode.hidden = true;
    try {
        var response = await fetch('/resources/media', { method: 'POST', body: formData });
        var payload = await response.json();
        if (!response.ok) throw new Error(payload.message || 'Media upload failed.');
        window.location.href = payload.alias ? '/' + payload.alias : '/' + payload.id;
    } catch (error) {
        showMediaCreateError(error.message || 'Media upload failed.');
        if (submit) submit.disabled = false;
    }
});

function showMediaCreateError(message) {
    var node = document.getElementById('media-create-error');
    if (!node) return;
    node.hidden = false;
    node.textContent = message;
}
