import assert from 'node:assert/strict';
import { assertVisibleText } from './assertions.mjs';
import { assertMediaCardGeometry } from './media-card-checks.mjs';
import { buildVideoUpload } from './fixture-api.mjs';
import { appUrl } from './support.mjs';

export async function assertMediaSearchFilter(page, media, hiddenNoteTitle) {
    await page.goto(`${appUrl}/search?kind=media`, { waitUntil: 'networkidle' });
    await page.getByLabel('Kind').selectOption('media');
    const titles = await page
        .locator('.resource-row[data-card-title]')
        .evaluateAll((nodes) => nodes.map((node) => node.dataset.cardTitle.trim()));
    assert.ok(titles.includes(media.image.title));
    assert.ok(titles.includes(media.video.title));
    assert.ok(!titles.includes(hiddenNoteTitle));
    await assertMediaCardGeometry(page, media.video.title);
}

export async function assertPublicMediaPage(page, media) {
    await page.goto(`${appUrl}/${media.ref}`, { waitUntil: 'networkidle' });
    await assertVisibleText(page, media.title);
    await page.locator(media.selector).waitFor({ state: 'visible' });
    assert.equal(await page.getByText('Current file', { exact: true }).count(), 0);
    await assertDownloadAndDelivery(page, media, media.fileHref);
    const robots = await page.locator('meta[name="robots"]').getAttribute('content');
    if (robots === 'index,follow' && media.contentType?.startsWith('image/')) {
        const ogImage = await page.locator('meta[property="og:image"]').getAttribute('content');
        assert.match(ogImage ?? '', /variant=(display|card)/);
        assert.equal(
            await page.locator('meta[name="twitter:card"]').getAttribute('content'),
            'summary_large_image'
        );
    }
    const snapshot = media.snapshots?.[0];
    if (!snapshot) return;
    await page.goto(`${appUrl}/${snapshot.id}`, { waitUntil: 'networkidle' });
    await assertVisibleText(page, media.title);
    await assertVisibleText(page, 'Saved file');
    await page.locator(media.selector).waitFor({ state: 'visible' });
    await assertDownloadAndDelivery(page, media, `/${snapshot.id}/file`);
}

export async function verifyUiCreatedMedia(page, note) {
    await page.goto(`${appUrl}/${note.ref}`, { waitUntil: 'networkidle' });
    await page.locator('#editor-body').waitFor({ state: 'visible' });
    await page.locator('#editor-body').evaluate((field) => {
        if (!field.value.endsWith('\n\n  ')) field.value += '\n\n  ';
        field.focus();
        field.setSelectionRange(field.value.length, field.value.length);
        field.dispatchEvent(new Event('input', { bubbles: true }));
    });
    const videoUpload = await buildVideoUpload(page, 'engine-room.webm');
    const uploadPromise = page.waitForResponse((response) => {
        const url = new URL(response.url());
        return url.pathname === `/resources/${note.id}/media-attachments` && response.request().method() === 'POST';
    });
    await page.locator('#upload-media-input').setInputFiles([
        {
            name: 'telemetry-grid.svg',
            mimeType: 'image/svg+xml',
            buffer: Buffer.from(
                '<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><rect width="24" height="24" fill="#111"/><path d="M4 18h16" stroke="#7ec8ff" stroke-width="2"/></svg>',
                'utf8'
            ),
        },
        {
            name: videoUpload.name,
            mimeType: videoUpload.mimeType,
            buffer: videoUpload.bytes,
        },
    ]);
    const uploadResponse = await uploadPromise;
    assert.equal(uploadResponse.status(), 200);
    const payload = await uploadResponse.json();
    assert.equal(payload.created_media.length, 2);
    assert.equal(Object.hasOwn(payload, 'created_notes'), false);
    const body = await page.locator('#editor-body').inputValue();
    for (const media of payload.created_media) {
        assert.ok(body.includes(media.file_href), `note body should embed ${media.file_href}`);
    }
    await assertVisibleText(page, 'Uploaded 2 media items.');

    await page.locator('#editor-body').evaluate((field) => {
        field.value = '# Upload Cursor\n\n漢🙂B';
        const cursor = field.value.lastIndexOf('B');
        field.focus();
        field.setSelectionRange(cursor, cursor);
        field.dispatchEvent(new Event('input', { bubbles: true }));
    });
    const cursorUploadPromise = page.waitForResponse((response) => {
        const url = new URL(response.url());
        return url.pathname === `/resources/${note.id}/media-attachments` && response.request().method() === 'POST';
    });
    await page.getByRole('button', { name: 'Upload media', exact: true }).click();
    await page.locator('#upload-media-input').setInputFiles([{
        name: 'cursor-target.svg',
        mimeType: 'image/svg+xml',
        buffer: Buffer.from('<svg xmlns="http://www.w3.org/2000/svg"><rect width="12" height="12" fill="#7ec8ff"/></svg>', 'utf8'),
    }]);
    const cursorPayload = await (await cursorUploadPromise).json();
    assert.ok(cursorPayload.current_resource.body.includes(cursorPayload.inserted_markdown + 'B'));
    const editorCursor = await page.locator('#editor-body').evaluate((field) => ({
        body: field.value,
        selectionStart: field.selectionStart,
    }));
    const expectedCursor = await page.evaluate(({ body, cursorUtf8 }) => {
        let total = 0;
        for (let index = 0; index < body.length; index += 1) {
            const codePoint = body.codePointAt(index);
            const segment = String.fromCodePoint(codePoint);
            const bytes = new TextEncoder().encode(segment).length;
            if (total + bytes > cursorUtf8) return index;
            total += bytes;
            if (codePoint > 0xFFFF) index += 1;
        }
        return body.length;
    }, { body: cursorPayload.current_resource.body, cursorUtf8: cursorPayload.cursor_utf8 });
    assert.equal(editorCursor.body, cursorPayload.current_resource.body);
    assert.equal(editorCursor.selectionStart, expectedCursor);

    const staleRangeResponse = await page.evaluate(async (noteId) => {
        const body = document.querySelector('#editor-body').value;
        const formData = new FormData();
        formData.append(
            'file',
            new File(
                ['<svg xmlns="http://www.w3.org/2000/svg"><circle r="4"/></svg>'],
                'stale-range.svg',
                { type: 'image/svg+xml' }
            )
        );
        formData.append('body', body);
        formData.append(
            'is_favorite',
            document.querySelector('#favorite-toggle').checked ? 'true' : 'false'
        );
        formData.append(
            'is_private',
            document.querySelector('#public-toggle').checked ? 'false' : 'true'
        );
        formData.append('insert_start', String(new TextEncoder().encode(body).length + 100));
        formData.append('insert_end', String(new TextEncoder().encode(body).length + 100));
        const alias = document.querySelector('#alias-input').value.trim();
        if (alias) formData.append('alias', alias);
        const response = await fetch(`/resources/${noteId}/media-attachments`, {
            method: 'POST',
            body: formData,
        });
        return { status: response.status, payload: await response.json() };
    }, note.id);
    assert.equal(staleRangeResponse.status, 200);
    assert.equal(staleRangeResponse.payload.selection_fallback, true);
    assert.equal(staleRangeResponse.payload.created_media.length, 1);
    assert.equal(Object.hasOwn(staleRangeResponse.payload, 'created_notes'), false);
    assert.ok(
        staleRangeResponse.payload.current_resource.body.endsWith(staleRangeResponse.payload.inserted_markdown),
        'stale upload selection should append embeds to the submitted draft'
    );
}

async function assertDownloadAndDelivery(page, media, rawHref) {
    const download = page.getByRole('link', { name: 'Download original', exact: true });
    await download.waitFor({ state: 'visible' });
    assert.equal(await download.getAttribute('href'), rawHref);
    assert.equal(await download.getAttribute('download'), media.originalFilename);
    const rawResponse = await fetchResource(page, rawHref, !!media.rawText);
    assert.equal(rawResponse.status, 200, 'media file should stream publicly');
    assert.equal(
        rawResponse.contentType?.startsWith(media.contentType),
        true,
        'media file should keep its raw content type'
    );
    if (media.rawText) {
        assert.equal(rawResponse.body, media.rawText, 'raw media download should preserve the uploaded bytes');
    }
    if (!media.contentType?.startsWith('image/')) return;
    const displayHref = await page.locator('.media-surface img').getAttribute('src');
    assert.match(displayHref ?? '', /variant=display/);
    const displayResponse = await fetchResource(page, displayHref, false);
    assert.equal(displayResponse.status, 200, 'display variant should stream publicly');
    assert.equal(
        displayResponse.contentType?.startsWith('image/webp'),
        true,
        'display delivery should use a WebP derivative'
    );
}

async function fetchResource(page, href, includeBody) {
    return page.evaluate(
        async ({ requestHref, readBody }) => {
            const response = await fetch(requestHref);
            return {
                status: response.status,
                contentType: response.headers.get('content-type'),
                body: readBody ? await response.text() : null,
            };
        },
        { requestHref: href, readBody: includeBody }
    );
}
