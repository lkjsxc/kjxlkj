import assert from 'node:assert/strict';

export async function assertGuestEmbeds(page) {
    await page.locator('.prose .local-resource-card[data-card-title="Atlas Entry"]').waitFor({ state: 'visible' });
    await page.locator('.prose .local-resource-card[data-card-title="Orbital Archive"]').waitFor({ state: 'visible' });
    await page.locator('.prose .external-embed-bookmark:has-text("Cached pull request")').waitFor({ state: 'visible' });
    await page.locator('.prose iframe[src*="open.spotify.com/embed/track/abc123"]').waitFor({ state: 'visible' });
    await page.locator('.prose iframe[src*="tiktok.com/player/v1/1234567890"]').waitFor({ state: 'visible' });
    await page.locator('.prose .external-embed-social[data-embed-provider="x"][data-embed-hydrated="true"]').waitFor({ state: 'visible' });
    await page.locator('.prose audio[src="https://example.com/interview.mp3"]').waitFor({ state: 'visible' });
    assert.equal(await page.locator('.prose iframe[src*="google.com/maps/embed"]').count(), 0);
}

export async function assertAdminEmbeds(page) {
    await page.locator('#editor-preview .local-resource-card[data-card-title="Atlas Entry"]').waitFor({ state: 'visible' });
    await page.locator('#editor-preview iframe[src*="google.com/maps/embed/v1/search"]').waitFor({ state: 'visible' });
    await page.locator('#editor-preview .external-embed-social[data-embed-provider="x"][data-embed-hydrated="true"]').waitFor({ state: 'visible' });
}
