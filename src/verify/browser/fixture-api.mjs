import assert from 'node:assert/strict';

const syntheticGraphic = `<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 160 90" data-fixture="synthetic-heic">
<rect width="160" height="90" fill="#10161f"/>
<circle cx="50" cy="45" r="24" fill="#6fb3ff"/>
<circle cx="102" cy="45" r="18" fill="#f4c266"/>
</svg>`;

export const imageUpload = {
    name: 'orbital-chart.svg',
    mimeType: 'image/svg+xml',
    bytes: Buffer.from(syntheticGraphic, 'utf8'),
    text: syntheticGraphic,
};

export const fileUpload = {
    name: 'orbital-archive.heic',
    mimeType: 'image/heic',
    bytes: Buffer.from(syntheticGraphic, 'utf8'),
    text: syntheticGraphic,
};

export async function buildVideoUpload(page, name = 'launch-clip.webm') {
    return {
        name,
        mimeType: 'video/webm',
        bytes: Buffer.from(await recordVideoBytes(page)),
    };
}

export async function createHistoryNote(page, media) {
    const note = await createNote(page, '# Orbit Ledger\n\nPrivate draft.', {
        isPrivate: true,
        alias: 'orbit-ledger',
        favorite: true,
    });
    for (const body of historyBodies(media)) {
        await updateResource(page, note.id, body, {
            isPrivate: body.includes('Private') || body.includes('draft'),
            alias: 'orbit-ledger',
            favorite: true,
        });
    }
    return { ...note, alias: 'orbit-ledger', snapshots: await listSnapshots(page, note.id) };
}

export async function createMedia(page, file, options) {
    const result = await page.evaluate(
        async ({ alias, bytes, favorite, fileName, isPrivate, mimeType }) => {
            const formData = new FormData();
            formData.append(
                'file',
                new File([Uint8Array.from(bytes)], fileName, { type: mimeType })
            );
            if (alias) formData.append('alias', alias);
            formData.append('is_favorite', favorite ? 'true' : 'false');
            formData.append('is_private', isPrivate ? 'true' : 'false');
            const response = await fetch('/resources/media', { method: 'POST', body: formData });
            return { status: response.status, payload: await response.json() };
        },
        {
            alias: options.alias ?? null,
            bytes: [...file.bytes],
            favorite: !!options.favorite,
            fileName: file.name,
            isPrivate: !!options.isPrivate,
            mimeType: file.mimeType,
        }
    );
    assert.equal(result.status, 201, `${file.name} should create a media resource`);
    return result.payload;
}

export async function createNote(page, body, options) {
    const result = await page.evaluate(
        async ({ alias, favorite, isPrivate, noteBody }) => {
            const response = await fetch('/resources/notes', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({
                    body: noteBody,
                    alias,
                    is_favorite: favorite,
                    is_private: isPrivate,
                }),
            });
            return { status: response.status, payload: await response.json() };
        },
        {
            alias: options.alias ?? null,
            favorite: !!options.favorite,
            isPrivate: !!options.isPrivate,
            noteBody: body,
        }
    );
    assert.equal(result.status, 201, 'note create should succeed');
    return result.payload;
}

export async function listSnapshots(page, id) {
    return page.evaluate(async (resourceId) => {
        const response = await fetch(`/resources/${resourceId}/history?limit=10`);
        const payload = await response.json();
        return payload.snapshots;
    }, id);
}

export async function updateResource(page, id, body, options) {
    const status = await page.evaluate(
        async ({ alias, favorite, isPrivate, noteBody, resourceId }) => {
            const response = await fetch(`/resources/${resourceId}`, {
                method: 'PUT',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({
                    body: noteBody,
                    alias,
                    is_favorite: favorite,
                    is_private: isPrivate,
                }),
            });
            return response.status;
        },
        {
            alias: options.alias ?? null,
            favorite: !!options.favorite,
            isPrivate: !!options.isPrivate,
            noteBody: body,
            resourceId: id,
        }
    );
    assert.equal(status, 200, `resource update should succeed for ${id}`);
}

function historyBodies(media) {
    return [
        '# Orbit Ledger\n\nSecond private draft.',
        `# Orbit Ledger

Shared release with the current chart.

![Orbital chart](${media.image.fileHref})`,
        `# Orbit Ledger

Current shared snapshot stretches across the list card with enough words to stress the timestamp column.

![Orbital chart](${media.image.fileHref})

<video controls src="${media.video.fileHref}"></video>

Follow-up detail keeps the summary ellipsis active.`,
    ];
}

async function recordVideoBytes(page) {
    return page.evaluate(async () => {
        const canvas = document.createElement('canvas');
        canvas.width = 160;
        canvas.height = 90;
        const context = canvas.getContext('2d');
        const stream = canvas.captureStream(8);
        const recorder = new MediaRecorder(stream, { mimeType: 'video/webm;codecs=vp8' });
        const chunks = [];

        function drawFrame(primary, accent) {
            context.fillStyle = '#10161f';
            context.fillRect(0, 0, canvas.width, canvas.height);
            context.fillStyle = primary;
            context.fillRect(18, 18, 56, 56);
            context.fillStyle = accent;
            context.beginPath();
            context.arc(112, 45, 20, 0, Math.PI * 2);
            context.fill();
        }

        recorder.ondataavailable = (event) => {
            if (event.data.size > 0) chunks.push(event.data);
        };
        const stopped = new Promise((resolve) => {
            recorder.onstop = resolve;
        });

        recorder.start();
        drawFrame('#6fb3ff', '#f4c266');
        await new Promise((resolve) => setTimeout(resolve, 180));
        drawFrame('#7dd3a7', '#f27f73');
        await new Promise((resolve) => setTimeout(resolve, 180));
        recorder.stop();
        await stopped;

        const blob = new Blob(chunks, { type: recorder.mimeType });
        return Array.from(new Uint8Array(await blob.arrayBuffer()));
    });
}
