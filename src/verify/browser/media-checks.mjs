import assert from 'node:assert/strict';
import { assertVisibleText } from './assertions.mjs';
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
    await assertMediaCardGeometry(page);
}

export async function assertPublicMediaPage(page, media) {
    await page.goto(`${appUrl}/${media.ref}`, { waitUntil: 'networkidle' });
    await assertVisibleText(page, media.title);
    await page.locator(media.selector).waitFor({ state: 'visible' });
    const response = await page.evaluate(async (href) => {
        const file = await fetch(href);
        return { status: file.status, contentType: file.headers.get('content-type') };
    }, media.fileHref);
    assert.equal(response.status, 200, 'media file should stream publicly');
    assert.equal(
        response.contentType?.startsWith(media.contentType),
        true,
        'media file should keep its content type'
    );
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
            name: 'engine-room.mp4',
            mimeType: 'video/mp4',
            buffer: Buffer.from('engine-room-video-fixture', 'utf8'),
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

async function assertMediaCardGeometry(page) {
    await page.locator('.resource-row-media .card-cover').first().waitFor({ state: 'visible' });
    const metrics = await page.locator('.resource-row-media').evaluateAll((cards) =>
        cards.map((card) => {
            const cover = card.querySelector('.card-cover').getBoundingClientRect();
            const badges = card.querySelector('.card-badges').getBoundingClientRect();
            const media = card.querySelector('.card-cover-media').getBoundingClientRect();
            return {
                coverBottom: cover.bottom,
                coverHeight: Math.round(cover.height),
                badgesTop: badges.top,
                mediaHeight: Math.round(media.height),
            };
        })
    );
    assert.ok(metrics.length >= 2, 'media search should render media cards');
    for (const item of metrics) {
        assert.equal(item.coverHeight, 104, 'media thumbnail height should be fixed');
        assert.ok(item.mediaHeight >= item.coverHeight - 1, 'media should fill the cover height');
        assert.ok(item.badgesTop >= item.coverBottom, 'badges should sit below the thumbnail');
    }
    const imagePaint = await page.locator('.resource-row-media img.card-cover-media').first().evaluate(
        async (image) => {
            if (!image.complete) {
                await new Promise((resolve, reject) => {
                    image.addEventListener('load', resolve, { once: true });
                    image.addEventListener('error', reject, { once: true });
                });
            }
            const canvas = document.createElement('canvas');
            canvas.width = 16;
            canvas.height = 16;
            const context = canvas.getContext('2d');
            context.drawImage(image, 0, 0, 16, 16);
            const data = context.getImageData(0, 0, 16, 16).data;
            return {
                naturalHeight: image.naturalHeight,
                naturalWidth: image.naturalWidth,
                painted: Array.from(data).some((value, index) => index % 4 !== 3 && value > 0),
            };
        }
    );
    assert.ok(imagePaint.naturalWidth > 0 && imagePaint.naturalHeight > 0, 'image card should load');
    assert.ok(imagePaint.painted, 'image card should render visible pixels');
}
