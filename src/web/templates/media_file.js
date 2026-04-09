function initMediaFileForm() {
    var form = document.getElementById('media-file-form');
    if (!form) return;
    form.addEventListener('submit', async function (event) {
        event.preventDefault();
        var input = document.getElementById('media-file-input');
        if (!input?.files?.length) {
            setMediaFileStatus('Select one replacement file.');
            return;
        }
        var submit = document.getElementById('media-file-submit');
        var formData = new FormData();
        formData.append('file', input.files[0]);
        setMediaFileStatus('');
        if (submit) submit.disabled = true;
        try {
            var response = await fetch('/resources/media/' + currentId + '/file', {
                method: 'PUT',
                body: formData
            });
            var payload = await response.json();
            if (!response.ok) throw new Error(payload.message || 'File replacement failed.');
            window.location.reload();
        } catch (error) {
            setMediaFileStatus(error.message || 'File replacement failed.');
            if (submit) submit.disabled = false;
        }
    });
}

function setMediaFileStatus(message) {
    var node = document.getElementById('media-file-status');
    if (!node) return;
    node.hidden = !message;
    node.textContent = message;
}
