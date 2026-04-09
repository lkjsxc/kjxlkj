import assert from 'node:assert/strict';
import { assertVisibleText } from './assertions.mjs';
import { appUrl } from './support.mjs';

export async function assertMediaSearchFilter(page, media, hiddenNoteTitle) {
    await page.goto(`${appUrl}/search?kind=media`, { waitUntil: 'networkidle' });
    await page.getByLabel('Kind').selectOption('media');
    await assertVisibleText(page, media.image.title);
    await assertVisibleText(page, media.video.title);
    assert.equal(await page.getByText(hiddenNoteTitle, { exact: true }).count(), 0);
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

export async function verifyUiCreatedMedia(page) {
    await Promise.all([
        page.waitForURL('**/admin/media/new'),
        page.getByRole('link', { name: 'New media', exact: true }).first().click(),
    ]);
    await page.locator('#media-file-input').setInputFiles({
        name: 'telemetry-grid.svg',
        mimeType: 'image/svg+xml',
        buffer: Buffer.from(
            '<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><rect width="24" height="24" fill="#111"/><path d="M4 18h16" stroke="#7ec8ff" stroke-width="2"/></svg>',
            'utf8'
        ),
    });
    await page.locator('#media-alias-input').fill('telemetry-grid');
    const createPromise = page.waitForResponse((response) => {
        return new URL(response.url()).pathname === '/resources/media';
    });
    await page.getByRole('button', { name: 'Create media', exact: true }).click();
    assert.equal((await createPromise).status(), 201);
    await page.waitForURL('**/telemetry-grid');
    await page.locator('.media-surface img').waitFor({ state: 'visible' });
}
