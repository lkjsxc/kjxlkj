import assert from 'node:assert/strict';
import {
    assertCreateActionBelowHome,
    assertGridHeights,
    assertInvisibleText,
    assertNoHorizontalOverflow,
    assertNoHeaderButtons,
    assertSingleHistoryCard,
    assertStableMetadata,
    assertVisibleText,
    assertWideGrid,
    expectClosedDrawer,
    expectFlatShell,
    openDrawer,
} from './shell-assertions.mjs';

export { assertInvisibleText, assertNoHorizontalOverflow, assertVisibleText, expectClosedDrawer, openDrawer };

export async function expectPublicRoot(
    page,
    { title = 'Home', intro = 'Welcome to', sections = ['Popular notes', 'Recently updated', 'Favorites'] } = {}
) {
    await expectFlatShell(page);
    await page.getByRole('heading', { name: title, exact: true }).first().waitFor({ state: 'visible' });
    if (intro) await assertVisibleText(page, intro);
    for (const section of sections) await assertVisibleText(page, section);
    for (const hidden of ['Popular notes', 'Recently updated', 'Favorites'].filter((item) => !sections.includes(item))) {
        assert.equal(await page.getByRole('heading', { name: hidden, exact: true }).count(), 0);
    }
    assert.equal(await page.getByRole('link', { name: /View more notes/i }).count(), sections.length);
    await page.getByLabel('Quick search').waitFor({ state: 'visible' });
    await page.getByRole('button', { name: '7d', exact: true }).waitFor({ state: 'visible' });
    await page.getByRole('button', { name: '30d', exact: true }).waitFor({ state: 'visible' });
    await page.getByRole('button', { name: '90d', exact: true }).waitFor({ state: 'visible' });
    assert.equal(await page.locator('.stats-grid').count(), 0);
    assert.equal(await page.locator('.page-summary').count(), 0);
    assert.equal(await page.getByText('All time', { exact: true }).count(), 0);
    assert.equal(await page.getByText('Views total', { exact: true }).count(), 0);
    await assertNoHeaderButtons(page);
    if ((await page.evaluate(() => window.innerWidth)) > 900) {
        await assertWideGrid(page);
        await assertGridHeights(page, '.note-grid .note-row');
    }
    await assertSectionOrder(page, sections);
}

export async function expectSearchPage(page, hasQueryCard = false) {
    await expectFlatShell(page);
    await assertVisibleText(page, 'Search');
    await page.getByLabel('Search notes').waitFor({ state: 'visible' });
    if (hasQueryCard) {
        await assertVisibleText(page, 'Query');
    } else {
        assert.equal(await page.getByText('Query', { exact: true }).count(), 0);
    }
    await page.getByLabel('Sort').waitFor({ state: 'visible' });
    assert.equal(await page.locator('.search-sort .visually-hidden').count(), 1);
    assert.equal(await page.locator('.search-sort span:not(.visually-hidden)').count(), 0);
    await assertSearchControlsAligned(page);
    await page.getByRole('button', { name: 'Previous', exact: true }).waitFor({ state: 'visible' });
    await page.getByRole('button', { name: 'Next', exact: true }).waitFor({ state: 'visible' });
    await assertNoHeaderButtons(page);
}

export async function expectAdminDashboard(page) {
    await expectFlatShell(page, ['New note', 'Logout']);
    await assertVisibleText(page, 'Dashboard');
    await assertVisibleText(page, 'Settings');
    await assertVisibleText(page, 'Session timeout');
    await assertVisibleText(page, 'Popular notes');
    await assertVisibleText(page, 'Recently updated');
    await assertVisibleText(page, 'Favorites');
    await page.getByRole('link', { name: 'Open settings', exact: true }).waitFor({ state: 'visible' });
    await assertVisibleText(page, 'Views total');
    await page.locator('[data-favorite-order]').waitFor({ state: 'visible' });
    assert.equal(await page.getByRole('heading', { name: 'Library', exact: true }).count(), 0);
    assert.equal(await page.getByText('Default Vim mode for editors', { exact: true }).count(), 0);
    assert.equal(await page.getByLabel('Home intro Markdown').count(), 0);
    assert.equal(await page.locator('.page-summary').count(), 0);
    await assertNoHeaderButtons(page);
    await assertStableMetadata(page, 'Orbit Ledger');
    await assertCreateActionBelowHome(page);
    await assertSectionOrder(page, ['Settings', 'Popular notes', 'Recently updated', 'Favorites']);
}

export async function expectSettingsPage(page) {
    await expectFlatShell(page, ['New note', 'Logout']);
    await assertVisibleText(page, 'Settings');
    await page.getByLabel('Site name').waitFor({ state: 'visible' });
    await page.getByLabel('Site description').waitFor({ state: 'visible' });
    await page.getByLabel('Public base URL').waitFor({ state: 'visible' });
    await page.getByLabel('Home intro Markdown').waitFor({ state: 'visible' });
    await page.getByLabel('Session timeout (minutes)').waitFor({ state: 'visible' });
    await page.getByLabel('Search page size').waitFor({ state: 'visible' });
    await page.getByLabel('New notes start private').waitFor({ state: 'visible' });
    await assertVisibleText(page, 'Home sections');
    await assertVisibleText(page, 'Sessions');
    await assertVisibleText(page, 'Defaults');
    await page.locator('[data-settings-order-list]').waitFor({ state: 'visible' });
    assert.equal(await page.getByText('Default Vim mode for editors', { exact: true }).count(), 0);
    assert.equal(await page.getByText('Order', { exact: true }).count(), 0);
    assert.equal(await page.locator('.settings-order-pill').count(), 0);
    await assertNoHeaderButtons(page);
}

export async function expectAdminNote(page) {
    await expectFlatShell(page);
    await page.locator('#editor-body').waitFor({ state: 'visible' });
    assert.equal(await page.locator('#public-toggle').isChecked(), true);
    assert.equal(await page.locator('#favorite-toggle').isChecked(), true);
    assert.equal(await page.locator('#preview-toggle').getAttribute('aria-expanded'), 'false');
    assert.equal(await page.locator('.note-head .status-pill').count(), 0);
    assert.equal(await page.locator('.note-head h1').count(), 0);
    await assertVisibleText(page, 'Delete note');
    await assertVisibleText(page, 'URL alias');
    await assertVisibleText(page, 'Canonical URL');
    await assertVisibleText(page, 'Views total');
    await assertVisibleText(page, 'Views 30d');
    await assertSingleHistoryCard(page);
    assert.equal(await page.getByText('Markdown body', { exact: true }).count(), 0);
    assert.equal(await page.locator('script[src*="toastui"],link[href*="toastui"]').count(), 0);
    await assertCreateActionBelowHome(page);
}

export async function expectGuestNote(page, previousTitle, nextTitle) {
    await expectFlatShell(page);
    await assertSingleHistoryCard(page);
    assert.equal(await page.getByText('Views total', { exact: true }).count(), 0);
    await assertVisibleText(page, 'Prev');
    await assertVisibleText(page, previousTitle ?? 'No older accessible note.');
    await assertVisibleText(page, 'Next');
    await assertVisibleText(page, nextTitle ?? 'No newer accessible note.');
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
