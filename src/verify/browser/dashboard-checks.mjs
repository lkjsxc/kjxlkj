import assert from 'node:assert/strict';
import { appUrl } from './support.mjs';

export async function verifyFavoriteReorder(page) {
    const items = page.locator('[data-favorite-order] .favorite-order-item');
    const initialTitles = await items.locator('.favorite-order-title').evaluateAll((nodes) =>
        nodes.map((node) => node.textContent.trim())
    );
    assert.deepEqual(initialTitles, ['Orbit Ledger', 'Beacon Log']);
    await Promise.all([
        page.waitForResponse((response) => {
            const url = new URL(response.url());
            return url.pathname === '/records/favorites/order' && response.request().method() === 'PUT';
        }),
        items.nth(1).dragTo(items.nth(0)),
    ]);
    const reorderedTitles = await items.locator('.favorite-order-title').evaluateAll((nodes) =>
        nodes.map((node) => node.textContent.trim())
    );
    assert.deepEqual(reorderedTitles, ['Beacon Log', 'Orbit Ledger']);
    await page.reload({ waitUntil: 'networkidle' });
    const persistedTitles = await page.locator('[data-favorite-order] .favorite-order-title').evaluateAll((nodes) =>
        nodes.map((node) => node.textContent.trim())
    );
    assert.deepEqual(persistedTitles, ['Beacon Log', 'Orbit Ledger']);
    await page.goto(`${appUrl}/`, { waitUntil: 'networkidle' });
    const favoriteTitles = await page
        .locator('.section-block.note-section', {
            has: page.getByRole('heading', { name: 'Favorites', exact: true }),
        })
        .locator('.card-title')
        .evaluateAll((nodes) => nodes.map((node) => node.textContent.trim()));
    assert.equal(favoriteTitles[0], 'Beacon Log');
    await page.goto(`${appUrl}/admin`, { waitUntil: 'networkidle' });
}

export async function applySettingsScenario(page) {
    assert.equal(await page.locator('input[name="home_popular_limit"]').inputValue(), '5');
    assert.equal(await page.locator('input[name="home_recent_limit"]').inputValue(), '5');
    assert.equal(await page.locator('input[name="home_favorite_limit"]').inputValue(), '5');
    assert.equal(await page.getByLabel('New notes start private').isChecked(), true);
    await page.getByLabel('Home intro Markdown').fill('# Launchpad\n\nWelcome to **Launchpad**.');
    await reorderHomeSections(page);
    await page.locator('input[name="home_recent_visible"]').uncheck();
    await page.getByLabel('New notes start private').uncheck();
    const responsePromise = page.waitForResponse((response) => {
        const url = new URL(response.url());
        return url.pathname === '/admin/settings' && response.request().method() === 'POST';
    });
    await page.getByRole('button', { name: 'Save settings', exact: true }).click();
    assert.equal((await responsePromise).status(), 303);
    await page.waitForLoadState('networkidle');
    assert.equal(await page.getByLabel('New notes start private').isChecked(), false);
    assert.deepEqual(await settingsOrder(page), ['Favorites', 'Popular notes', 'Recently updated']);
}

async function reorderHomeSections(page) {
    const rows = page.locator('[data-settings-order-item]');
    assert.deepEqual(await settingsOrder(page), ['Popular notes', 'Recently updated', 'Favorites']);
    await rows.nth(2).dragTo(rows.nth(0));
}

async function settingsOrder(page) {
    return page.locator('[data-settings-order-item] .settings-row-label').evaluateAll((nodes) =>
        nodes.map((node) => node.textContent.trim())
    );
}
