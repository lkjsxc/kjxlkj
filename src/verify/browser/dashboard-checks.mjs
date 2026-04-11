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
            return url.pathname === '/resources/favorites/order' && response.request().method() === 'PUT';
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
        .locator('.section-block.resource-section', {
            has: page.getByRole('heading', { name: 'Favorites', exact: true }),
        })
        .locator('.resource-row[data-card-title]')
        .evaluateAll((nodes) => nodes.map((node) => node.dataset.cardTitle.trim()));
    assert.equal(favoriteTitles[0], 'Beacon Log');
    await page.goto(`${appUrl}/admin`, { waitUntil: 'networkidle' });
}

export async function applySettingsScenario(page) {
    assert.equal(await page.getByLabel('Site name').inputValue(), 'kjxlkj');
    assert.equal(await page.getByLabel('Public base URL').inputValue(), '');
    assert.equal(await page.locator('input[name="home_popular_limit"]').inputValue(), '5');
    assert.equal(await page.locator('input[name="home_recent_limit"]').inputValue(), '5');
    assert.equal(await page.locator('input[name="home_favorite_limit"]').inputValue(), '5');
    assert.equal(await page.getByLabel('Session timeout (minutes)').inputValue(), '1440');
    assert.equal(await page.getByLabel('New resources start private').isChecked(), false);
    assert.equal(await page.getByText('Order', { exact: true }).count(), 0);
    assert.equal(await page.locator('.settings-order-pill').count(), 0);
    assert.deepEqual(await settingsOrder(page), ['Recently updated', 'Favorites', 'Popular']);
    await page.getByLabel('Site name').fill('Launchpad');
    await page.getByLabel('Site description').fill('Launchpad search surface for public resources.');
    await page.getByLabel('Public base URL').fill(appUrl);
    await page.getByLabel('Home intro Markdown').fill('# Launchpad\n\nWelcome to **Launchpad**.');
    await page.getByLabel('Session timeout (minutes)').fill('720');
    await reorderHomeSections(page);
    await page.locator('input[name="home_recent_visible"]').uncheck();
    const responsePromise = page.waitForResponse((response) => {
        const url = new URL(response.url());
        return url.pathname === '/admin/settings' && response.request().method() === 'POST';
    });
    await page.getByRole('button', { name: 'Save settings', exact: true }).click();
    assert.equal((await responsePromise).status(), 303);
    await page.waitForLoadState('networkidle');
    const discovery = await page.evaluate(async () => {
        const robots = await fetch('/robots.txt');
        const sitemap = await fetch('/sitemap.xml');
        return { robotsStatus: robots.status, sitemapStatus: sitemap.status };
    });
    assert.equal(await page.getByLabel('Site name').inputValue(), 'Launchpad');
    assert.equal(await page.getByLabel('Public base URL').inputValue(), appUrl);
    assert.equal(await page.getByLabel('Session timeout (minutes)').inputValue(), '720');
    assert.equal(await page.getByLabel('New resources start private').isChecked(), false);
    assert.equal(discovery.robotsStatus, 200);
    assert.equal(discovery.sitemapStatus, 200);
    assert.deepEqual(await settingsOrder(page), ['Favorites', 'Popular', 'Recently updated']);
}

export async function verifySettingsLeaveGuard(page) {
    await page.getByLabel('Site name').fill('Unsaved Launchpad');
    const linkDialog = page.waitForEvent('dialog', { timeout: 1500 }).catch(() => null);
    const linkClick = page.getByRole('link', { name: 'Home', exact: true }).click();
    const leaveLinkDialog = await linkDialog;
    if (leaveLinkDialog) {
        assert.match(leaveLinkDialog.message(), /Leave settings without saving/i);
        await leaveLinkDialog.dismiss();
    }
    await linkClick;
    await page.waitForTimeout(300);
    assert.equal(new URL(page.url()).pathname, '/admin/settings');
    const backDialog = page.waitForEvent('dialog', { timeout: 1500 }).catch(() => null);
    const backAttempt = page.goBack();
    const leaveBackDialog = await backDialog;
    if (leaveBackDialog) {
        assert.match(leaveBackDialog.message(), /Leave settings without saving/i);
        await leaveBackDialog.dismiss();
    }
    await backAttempt;
    await page.waitForTimeout(300);
    assert.equal(new URL(page.url()).pathname, '/admin/settings');
    await page.getByLabel('Site name').fill('kjxlkj');
}

async function reorderHomeSections(page) {
    assert.deepEqual(await settingsOrder(page), ['Recently updated', 'Favorites', 'Popular']);
    await page.getByRole('button', { name: 'Move Recently updated down', exact: true }).click();
    await page.getByRole('button', { name: 'Move Recently updated down', exact: true }).click();
}

async function settingsOrder(page) {
    return page.locator('[data-settings-order-item] .settings-row-label').evaluateAll((nodes) =>
        nodes.map((node) => node.textContent.trim())
    );
}
