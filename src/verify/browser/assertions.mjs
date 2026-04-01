import assert from 'node:assert/strict';
import {
    assertCreateActionBelowHome,
    assertGridHeights,
    assertInvisibleText,
    assertNoHorizontalOverflow,
    assertNoHeaderButtons,
    assertNoLegacyEditorAssets,
    assertSearchControlsAligned,
    assertSingleHistoryCard,
    assertStableMetadata,
    assertVisibleText,
    assertWideGrid,
    expectClosedDrawer,
    expectFlatShell,
    openDrawer,
} from './shell-assertions.mjs';

export { assertInvisibleText, assertNoHorizontalOverflow, assertVisibleText, expectClosedDrawer, openDrawer };

export async function expectPublicRoot(page) {
    await expectFlatShell(page);
    await assertVisibleText(page, 'Home');
    await assertVisibleText(page, 'Quick search');
    assert.equal(await page.locator('.page-intro.prose').count(), 1);
    await assertVisibleText(page, 'Popular notes');
    await assertVisibleText(page, 'Recently updated');
    await assertVisibleText(page, 'Favorites');
    await assertVisibleText(page, 'View more notes');
    await page.getByLabel('Quick search').waitFor({ state: 'visible' });
    await page.getByRole('link', { name: '7d', exact: true }).waitFor({ state: 'visible' });
    await page.getByRole('link', { name: '30d', exact: true }).waitFor({ state: 'visible' });
    await page.getByRole('link', { name: '90d', exact: true }).waitFor({ state: 'visible' });
    assert.equal(await page.locator('.stats-grid').count(), 0);
    assert.equal(await page.locator('.page-summary').count(), 0);
    assert.equal(await page.getByText('All time', { exact: true }).count(), 0);
    await assertNoHeaderButtons(page);
    if ((await page.evaluate(() => window.innerWidth)) > 900) {
        await assertWideGrid(page);
        await assertGridHeights(page, '.note-grid .note-row');
    }
}

export async function expectSearchPage(page, { hasQueryCard = false, scopeValue = null } = {}) {
    await expectFlatShell(page);
    await assertVisibleText(page, 'Search');
    await assertVisibleText(page, 'Search notes');
    await page.getByLabel('Search notes').waitFor({ state: 'visible' });
    if (hasQueryCard) {
        assert.equal(await hasStateCard(page, 'Query'), true);
    } else {
        assert.equal(await hasStateCard(page, 'Query'), false);
    }
    if (scopeValue) assert.equal(await hasStateCard(page, 'Scope', scopeValue), true);
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
    await page.getByRole('heading', { name: 'Dashboard', exact: true }).waitFor({ state: 'visible' });
    await page.getByRole('heading', { name: 'Settings', exact: true }).waitFor({ state: 'visible' });
    await page.getByRole('heading', { name: 'Popular notes', exact: true }).waitFor({ state: 'visible' });
    await page.getByRole('heading', { name: 'Recently updated', exact: true }).waitFor({ state: 'visible' });
    await page.getByRole('heading', { name: 'Favorites', exact: true }).waitFor({ state: 'visible' });
    await assertVisibleText(page, 'Open settings');
    await assertVisibleText(page, 'Home counts');
    await assertVisibleText(page, 'Views total');
    assert.equal(await page.getByRole('heading', { name: 'Library', exact: true }).count(), 0);
    assert.equal(await page.getByText('Home intro Markdown', { exact: true }).count(), 0);
    assert.equal(await page.locator('[data-favorite-order]').count(), 0);
    assert.equal(await page.locator('.page-summary').count(), 0);
    await assertNoHeaderButtons(page);
    await assertStableMetadata(page, 'Orbit Ledger');
    await assertCreateActionBelowHome(page);
    await assertSectionOrder(page, ['Settings', 'Popular notes', 'Recently updated', 'Favorites']);
}

export async function expectSettingsPage(page) {
    await expectFlatShell(page, ['New note', 'Logout']);
    await page.getByRole('heading', { name: 'Settings', exact: true }).waitFor({ state: 'visible' });
    await page.getByRole('heading', { name: 'Global settings', exact: true }).waitFor({ state: 'visible' });
    await assertVisibleText(page, 'Home intro Markdown');
    await assertVisibleText(page, 'Search page size');
    await assertVisibleText(page, 'New note visibility');
    await page.locator('[data-favorite-order]').waitFor({ state: 'visible' });
    assert.equal(await page.locator('.stats-grid').count(), 0);
    assert.equal(await page.getByText('Default Vim mode for editors', { exact: true }).count(), 0);
}

export async function expectAdminNote(page) {
    await expectFlatShell(page);
    await page.locator('#editor-source').waitFor({ state: 'visible' });
    assert.equal(await page.locator('#public-toggle').isChecked(), true);
    assert.equal(await page.locator('#favorite-toggle').isChecked(), true);
    assert.equal(await page.locator('#preview-toggle').getAttribute('aria-expanded'), 'false');
    assert.equal(await page.locator('.note-head .status-pill').count(), 0);
    await assertVisibleText(page, 'Delete note');
    await assertVisibleText(page, 'URL alias');
    await assertVisibleText(page, 'Canonical URL');
    await assertVisibleText(page, 'Views total');
    await assertVisibleText(page, 'Views 30d');
    await assertSingleHistoryCard(page);
    await assertNoLegacyEditorAssets(page);
    await assertCreateActionBelowHome(page);
}

export async function expectGuestNote(page, previousTitle, nextTitle) {
    await expectFlatShell(page);
    await assertSingleHistoryCard(page);
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

async function hasStateCard(page, label, value = null) {
    return page.evaluate(
        ({ stateLabel, stateValue }) =>
            Array.from(document.querySelectorAll('.search-state-card')).some((card) => {
                const labelText = card.querySelector('small')?.textContent?.trim();
                const valueText = card.querySelector('strong')?.textContent?.trim();
                return labelText === stateLabel && (stateValue === null || valueText === stateValue);
            }),
        { stateLabel: label, stateValue: value }
    );
}
