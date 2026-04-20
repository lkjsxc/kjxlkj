import assert from 'node:assert/strict';
import { appUrl } from './support.mjs';

const uploadedIconSvg = `<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 64 64" data-icon-fixture="settings-upload">
<rect width="64" height="64" rx="14" fill="#10161f"/>
<circle cx="32" cy="32" r="20" fill="#7ec8ff"/>
</svg>`;
export async function verifyFavoriteReorder(page) {
    const siteName = page.getByLabel('Site name');
    const items = page.locator('[data-favorite-order] .favorite-order-item');
    const initialTitles = await favoriteTitles(items);
    assert.deepEqual(initialTitles, ['Orbit Ledger', 'Beacon Log']);
    await siteName.fill('Favorite Draft');
    await Promise.all([favoriteOrderResponse(page), items.nth(1).dragTo(items.nth(0))]);
    assert.equal(await siteName.inputValue(), 'Favorite Draft');
    await siteName.fill('kjxlkj');
    const reorderedTitles = await favoriteTitles(items);
    assert.deepEqual(reorderedTitles, ['Beacon Log', 'Orbit Ledger']);
    await page.reload({ waitUntil: 'networkidle' });
    assert.deepEqual(await favoriteTitles(items), ['Beacon Log', 'Orbit Ledger']);
    await page.goto(`${appUrl}/admin`, { waitUntil: 'networkidle' });
    assert.equal(await page.locator('[data-favorite-order]').count(), 0);
    await page.getByRole('link', { name: 'Manage order', exact: true }).waitFor({ state: 'visible' });
    await page.goto(`${appUrl}/`, { waitUntil: 'networkidle' });
    assert.deepEqual(await homeFavoriteTitles(page), ['Beacon Log', 'Orbit Ledger']);
    await page.goto(`${appUrl}/search?scope=favorites&sort=favorite_position_asc`, { waitUntil: 'networkidle' });
    assert.deepEqual(await gridTitles(page), ['Beacon Log', 'Orbit Ledger']);
    await page.goto(`${appUrl}/admin/settings`, { waitUntil: 'networkidle' });
}
export async function verifySettingsSearch(page) {
    const input = page.getByLabel('Search settings');
    await input.fill('webp');
    await expectVisibleSection(page, 'Media', true);
    await expectVisibleSection(page, 'Search', false);
    await expectVisibleSection(page, 'New resources', false);
    await input.fill('future uploads');
    await expectVisibleSection(page, 'Media', true);
    await input.fill('site icon');
    await expectVisibleSection(page, 'Site icon', true);
    await input.fill('zzz-no-match');
    await page.getByText('No matching settings.', { exact: true }).waitFor({ state: 'visible' });
    await expectVisibleSection(page, 'Site identity', false);
    await input.fill('');
    await expectVisibleSection(page, 'Site identity', true);
    assert.equal(await page.getByText('No matching settings.', { exact: true }).isHidden(), true);
}
export async function verifySiteIconControls(page) {
    const siteName = page.getByLabel('Site name');
    const prompt = page.evaluate(() => {
        return new Promise((resolve) => {
            document
                .querySelector('[data-site-icon-input]')
                ?.addEventListener('click', () => resolve(true), { once: true });
        });
    });
    await page.getByRole('button', { name: 'Upload icon', exact: true }).click();
    assert.equal(await prompt, true);
    await siteName.fill('Icon Draft');
    await Promise.all([
        iconResponse(page, '/admin/site-icon'),
        page.locator('[data-site-icon-input]').setInputFiles({
            name: 'settings-upload.svg',
            mimeType: 'image/svg+xml',
            buffer: Buffer.from(uploadedIconSvg, 'utf8'),
        }),
    ]);
    await page.getByText('Icon uploaded.', { exact: true }).waitFor({ state: 'visible' });
    assert.equal(await siteName.inputValue(), 'Icon Draft');
    assert.match(await iconBody(page), /data-icon-fixture="settings-upload"/);
    assert.equal(await page.locator('[data-site-icon-reset]').isHidden(), false);
    await Promise.all([
        iconResponse(page, '/admin/site-icon/reset'),
        page.getByRole('button', { name: 'Reset icon', exact: true }).click(),
    ]);
    await page.getByText('Icon reset.', { exact: true }).waitFor({ state: 'visible' });
    assert.equal(await siteName.inputValue(), 'Icon Draft');
    assert.doesNotMatch(await iconBody(page), /data-icon-fixture="settings-upload"/);
    assert.equal(await page.locator('[data-site-icon-reset]').isHidden(), true);
    await siteName.fill('kjxlkj');
}
export async function applySettingsScenario(page) {
    assert.equal(await page.getByLabel('Site name').inputValue(), 'kjxlkj');
    assert.equal(await page.getByLabel('Public base URL').inputValue(), '');
    assert.equal(await page.getByLabel('Search page size').inputValue(), '20');
    assert.equal(await page.getByLabel('Media WebP quality').inputValue(), '82');
    assert.equal(await page.locator('input[name="home_popular_limit"]').inputValue(), '5');
    assert.equal(await page.locator('input[name="home_recent_limit"]').inputValue(), '5');
    assert.equal(await page.locator('input[name="home_favorite_limit"]').inputValue(), '5');
    assert.equal(await page.getByLabel('Session timeout (minutes)').inputValue(), '1440');
    assert.equal(await page.getByLabel('New resources start private').isChecked(), false);
    assert.equal(await page.getByRole('button', { name: /Move .* (up|down)/ }).count(), 0);
    assert.deepEqual(await settingsOrder(page), ['Recently updated', 'Favorites', 'Popular']);
    await page.getByLabel('Site name').fill('Launchpad');
    await page.getByLabel('Site description').fill('Launchpad search surface for public resources.');
    await page.getByLabel('Public base URL').fill(appUrl);
    await page.getByLabel('Home intro Markdown').fill('# Launchpad\n\nWelcome to **Launchpad**.');
    await page.getByLabel('Session timeout (minutes)').fill('720');
    await page.getByLabel('Search page size').fill('12');
    await page.getByLabel('Media WebP quality').fill('67');
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
    assert.equal(await page.getByLabel('Search page size').inputValue(), '12');
    assert.equal(await page.getByLabel('Media WebP quality').inputValue(), '67');
    assert.equal(await page.getByLabel('Session timeout (minutes)').inputValue(), '720');
    assert.equal(await page.getByLabel('New resources start private').isChecked(), false);
    assert.equal(discovery.robotsStatus, 200);
    assert.equal(discovery.sitemapStatus, 200);
    assert.deepEqual(await settingsOrder(page), ['Favorites', 'Recently updated', 'Popular']);
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
    const items = page.locator('[data-settings-order-item]');
    await items.nth(0).dragTo(items.nth(2));
    await items.nth(1).dragTo(items.nth(2));
}
async function settingsOrder(page) {
    return page.locator('[data-settings-order-item] .settings-row-label').evaluateAll((nodes) =>
        nodes.map((node) => node.textContent.trim())
    );
}
async function favoriteTitles(items) {
    return items.locator('.favorite-order-title').evaluateAll((nodes) =>
        nodes.map((node) => node.textContent.trim())
    );
}
async function homeFavoriteTitles(page) {
    return page
        .locator('.section-block.resource-section', {
            has: page.getByRole('heading', { name: 'Favorites', exact: true }),
        })
        .locator('.resource-row[data-card-title]')
        .evaluateAll((nodes) => nodes.map((node) => node.dataset.cardTitle.trim()));
}
async function gridTitles(page) {
    return page.locator('.resource-grid .resource-row[data-card-title]').evaluateAll((nodes) =>
        nodes.map((node) => node.dataset.cardTitle.trim())
    );
}
async function expectVisibleSection(page, name, visible) {
    assert.equal(await page.locator('[data-settings-row]', { hasText: name }).first().isVisible(), visible);
}
async function iconResponse(page, pathname) {
    const response = await page.waitForResponse((item) => {
        const url = new URL(item.url());
        return url.pathname === pathname && item.request().method() === 'POST';
    });
    assert.equal(response.status(), 200);
}
async function favoriteOrderResponse(page) {
    const response = await page.waitForResponse((item) => {
        const url = new URL(item.url());
        return url.pathname === '/resources/favorites/order' && item.request().method() === 'PUT';
    });
    assert.equal(response.status(), 204);
}
async function iconBody(page) {
    return page.evaluate(async () => (await (await fetch('/assets/site-icon')).text()));
}
