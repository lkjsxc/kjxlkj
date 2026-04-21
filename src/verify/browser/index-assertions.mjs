import assert from 'node:assert/strict';
import { assertListRailOrder } from './rail-order-checks.mjs';
import {
    assertCreateActionBelowHome,
    assertGridHeights,
    assertNoHeaderButtons,
    assertStableMetadata,
    assertVisibleText,
    assertWideGrid,
    expectFlatShell,
} from './shell-assertions.mjs';

export async function expectPublicRoot(
    page,
    { title = 'Home', intro = 'Welcome to', sections = ['Recently updated', 'Favorites', 'Popular'] } = {}
) {
    await expectFlatShell(page);
    await assertVisibleText(page, 'Open GitHub');
    await page.getByRole('heading', { name: title, exact: true }).first().waitFor({ state: 'visible' });
    if (intro) await assertVisibleText(page, intro);
    for (const section of sections) await assertVisibleText(page, section);
    for (const hidden of ['Popular', 'Recently updated', 'Favorites'].filter((item) => !sections.includes(item))) {
        assert.equal(await page.getByRole('heading', { name: hidden, exact: true }).count(), 0);
    }
    assert.equal(await page.getByRole('link', { name: /View more resources/i }).count(), sections.length);
    await page.getByLabel('Quick search resources').waitFor({ state: 'visible' });
    await page.getByRole('button', { name: '7d', exact: true }).waitFor({ state: 'visible' });
    await page.getByRole('button', { name: '30d', exact: true }).waitFor({ state: 'visible' });
    await page.getByRole('button', { name: '90d', exact: true }).waitFor({ state: 'visible' });
    await page.getByRole('button', { name: 'All time', exact: true }).waitFor({ state: 'visible' });
    assert.equal(await page.locator('.stats-grid').count(), 0);
    assert.equal(await page.locator('.page-summary').count(), 0);
    assert.equal(await page.getByText('Views total', { exact: true }).count(), 0);
    await assertNoHeaderButtons(page);
    if ((await page.evaluate(() => window.innerWidth)) > 900) {
        await assertWideGrid(page);
        await assertGridHeights(page, '.resource-grid .resource-row');
    }
    await assertListRailOrder(page);
    await assertSectionOrder(page, sections);
}

export async function expectSearchPage(page, hasQueryCard = false) {
    await expectFlatShell(page);
    await assertVisibleText(page, 'Open GitHub');
    await assertVisibleText(page, 'Search');
    await page.getByLabel('Search resources').waitFor({ state: 'visible' });
    if (hasQueryCard) {
        await assertVisibleText(page, 'Query');
    } else {
        assert.equal(await page.getByText('Query', { exact: true }).count(), 0);
    }
    await page.getByLabel('Sort').waitFor({ state: 'visible' });
    await page.getByLabel('Kind').waitFor({ state: 'visible' });
    assert.equal(await page.locator('.search-sort .visually-hidden').count(), 2);
    assert.equal(await page.locator('.search-sort span:not(.visually-hidden)').count(), 0);
    await assertSearchControlsAligned(page);
    await page.getByRole('button', { name: 'Prev', exact: true }).waitFor({ state: 'visible' });
    await page.getByRole('button', { name: 'Next', exact: true }).waitFor({ state: 'visible' });
    await assertNoHeaderButtons(page);
    await assertListRailOrder(page);
}

export async function expectLivePage(page, isAdmin = false) {
    await expectFlatShell(page, isAdmin ? ['New note', 'Logout'] : []);
    await assertVisibleText(page, 'Open GitHub');
    await assertVisibleText(page, 'Live');
    await assertVisibleText(page, 'Waiting for broadcast');
    await page.locator('[data-live-video]').waitFor({ state: 'visible' });
    if (isAdmin) {
        await page.getByRole('button', { name: 'Start broadcast', exact: true }).waitFor({ state: 'visible' });
        await page.getByRole('button', { name: 'Stop broadcast', exact: true }).waitFor({ state: 'visible' });
    } else {
        await page.getByRole('link', { name: 'Admin sign in', exact: true }).waitFor({ state: 'visible' });
    }
    await assertNoHeaderButtons(page);
    await assertListRailOrder(page);
}

export async function expectAdminDashboard(page) {
    await expectFlatShell(page, ['New note', 'Logout']);
    await assertVisibleText(page, 'Open GitHub');
    await assertVisibleText(page, 'Dashboard');
    await assertVisibleText(page, 'Settings');
    await assertVisibleText(page, 'Session timeout');
    await assertVisibleText(page, 'Popular');
    await assertVisibleText(page, 'Recently updated');
    await assertVisibleText(page, 'Favorites');
    await page.getByRole('link', { name: 'Open settings', exact: true }).waitFor({ state: 'visible' });
    await page.getByRole('link', { name: 'Manage order', exact: true }).waitFor({ state: 'visible' });
    await assertVisibleText(page, 'Views total');
    assert.equal(await page.locator('[data-favorite-order]').count(), 0);
    assert.equal(await page.getByRole('heading', { name: 'Library', exact: true }).count(), 0);
    assert.equal(await page.getByText('Default Vim mode for editors', { exact: true }).count(), 0);
    assert.equal(await page.getByLabel('Home/Hero_markdown').count(), 0);
    assert.equal(await page.locator('.page-summary').count(), 0);
    await assertNoHeaderButtons(page);
    await assertStableMetadata(page, 'Orbit Ledger');
    await assertCreateActionBelowHome(page);
    await assertListRailOrder(page);
    await assertSectionOrder(page, ['Settings', 'Popular', 'Recently updated', 'Favorites']);
}

export async function expectSettingsPage(page) {
    await expectFlatShell(page, ['New note', 'Logout']);
    await assertVisibleText(page, 'Open GitHub');
    await assertVisibleText(page, 'Settings');
    await page.getByLabel('Search settings').waitFor({ state: 'visible' });
    await page.getByLabel('Site_identity/Site_name').waitFor({ state: 'visible' });
    await page.getByLabel('Site_identity/Site_description').waitFor({ state: 'visible' });
    await page.getByLabel('Site_identity/Public_base_URL').waitFor({ state: 'visible' });
    await page.getByRole('button', { name: 'Upload icon', exact: true }).waitFor({ state: 'visible' });
    await page.getByLabel('Home/Hero_markdown').waitFor({ state: 'visible' });
    await page.getByLabel('Session/Timeout_minutes').waitFor({ state: 'visible' });
    await page.getByLabel('Search/Results_per_page').waitFor({ state: 'visible' });
    await page.getByLabel('Media/WebP_quality').waitFor({ state: 'visible' });
    await page.getByLabel('Resources/New_resources_start_private').waitFor({ state: 'visible' });
    await assertVisibleText(page, 'Home/Section_order');
    await assertVisibleText(page, 'Favorites');
    await assertVisibleText(page, 'Session/Timeout_minutes');
    await assertVisibleText(page, 'Search');
    await assertVisibleText(page, 'Media');
    await assertVisibleText(page, 'Resources/New_resources_start_private');
    await assertVisibleText(page, 'Password');
    assert.equal(await page.locator('.settings-section .section-head').count(), 0);
    await page.locator('[data-settings-order-list]').waitFor({ state: 'visible' });
    await page.locator('[data-favorite-order]').waitFor({ state: 'visible' });
    assert.equal(await page.getByText('Default Vim mode for editors', { exact: true }).count(), 0);
    assert.equal(await page.getByRole('heading', { name: 'Defaults', exact: true }).count(), 0);
    assert.equal(await page.getByRole('button', { name: /Move .* (up|down)/ }).count(), 0);
    assert.equal(await page.getByText('Order', { exact: true }).count(), 0);
    assert.equal(await page.locator('.settings-order-pill').count(), 0);
    await assertNoHeaderButtons(page);
    await assertListRailOrder(page);
}

async function assertSectionOrder(page, titles) {
    const tops = [];
    for (const title of titles) {
        const heading = page.getByRole('heading', { name: title, exact: true }).first();
        await heading.waitFor({ state: 'visible' });
        tops.push(await heading.evaluate((node) => node.getBoundingClientRect().top));
    }
    for (let index = 1; index < tops.length; index += 1) {
        assert.ok(tops[index] > tops[index - 1], 'dashboard sections should stack vertically');
    }
}

async function assertSearchControlsAligned(page) {
    const metrics = await page.evaluate(() => {
        const select = document.querySelector('#search-sort');
        const button = document.querySelector('.search-section button[type="submit"]');
        if (!select || !button) return null;
        return {
            top: Math.abs(select.getBoundingClientRect().top - button.getBoundingClientRect().top),
            bottom: Math.abs(select.getBoundingClientRect().bottom - button.getBoundingClientRect().bottom),
        };
    });
    assert.ok(metrics && metrics.top <= 1 && metrics.bottom <= 1, 'search sort and button should align vertically');
}
