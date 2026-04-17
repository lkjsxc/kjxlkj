import assert from 'node:assert/strict';

export async function assertDownloadAndDelivery(page, media, rawHref) {
    const download = page.getByRole('link', { name: 'Download original', exact: true });
    await download.waitFor({ state: 'visible' });
    assert.equal(await download.getAttribute('href'), rawHref);
    assert.equal(await download.getAttribute('download'), media.originalFilename);
    const rawResponse = await fetchResource(page, rawHref, !!media.rawText);
    assert.equal(rawResponse.status, 200);
    assert.equal(rawResponse.contentType?.startsWith(media.contentType), true);
    if (media.rawText) assert.equal(rawResponse.body, media.rawText);
    if (media.family !== 'image') return;
    const displayHref = await page.locator('.media-surface img').getAttribute('src');
    assert.match(displayHref ?? '', /variant=display/);
    const displayResponse = await fetchResource(page, displayHref, false);
    assert.equal(displayResponse.status, 200);
    assert.equal(displayResponse.contentType?.startsWith('image/webp'), true);
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
