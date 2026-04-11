import assert from 'node:assert/strict';

export async function assertMediaCardGeometry(page, videoTitle) {
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
        assert.equal(item.coverHeight, 128, 'media thumbnail height should be fixed');
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
    const videoPosterPaint = await page
        .locator(`.resource-row-media[data-card-title="${videoTitle}"] img.card-cover-media`)
        .first()
        .evaluate(async (image) => {
            if (!image.complete) {
                await new Promise((resolve, reject) => {
                    image.addEventListener('load', resolve, { once: true });
                    image.addEventListener('error', reject, { once: true });
                });
            }
            return {
                src: image.currentSrc,
                naturalWidth: image.naturalWidth,
                naturalHeight: image.naturalHeight,
            };
        });
    assert.match(videoPosterPaint.src, /variant=poster/);
    assert.ok(videoPosterPaint.naturalWidth > 0 && videoPosterPaint.naturalHeight > 0, 'video poster should load');
}
