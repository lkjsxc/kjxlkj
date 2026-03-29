import assert from 'node:assert/strict';
import {
    assertCreateActionBelowHome,
    assertGridHeights,
    assertInvisibleText,
    assertLocalToastUiAssets,
    assertNoHeaderButtons,
    assertSingleHistoryCard,
    assertStableMetadata,
    assertVisibleText,
    assertWideGrid,
    expectClosedDrawer,
    expectFlatShell,
    openDrawer,
} from './shell-assertions.mjs';

export { assertInvisibleText, assertVisibleText, expectClosedDrawer, openDrawer };

export async function expectPublicRoot(page) {
    await expectFlatShell(page);
    await assertVisibleText(page, 'Home');
    await assertVisibleText(page, 'Recently updated');
    await assertVisibleText(page, 'Favorites');
    await assertVisibleText(page, 'View more notes');
    await page.getByLabel('Quick search').waitFor({ state: 'visible' });
    assert.equal(await page.locator('.stats-grid').count(), 0);
    assert.equal(await page.locator('.page-summary').count(), 0);
    await assertNoHeaderButtons(page);
    if ((await page.evaluate(() => window.innerWidth)) > 900) {
        await assertWideGrid(page);
        await assertGridHeights(page, '.note-grid .note-row');
    }
}

export async function expectSearchPage(page) {
    await expectFlatShell(page);
    await assertVisibleText(page, 'Search');
    await page.getByLabel('Search notes').waitFor({ state: 'visible' });
    await page.getByLabel('Sort').waitFor({ state: 'visible' });
    await assertNoHeaderButtons(page);
}

export async function expectAdminDashboard(page) {
    await expectFlatShell(page, ['New note', 'Logout']);
    await assertVisibleText(page, 'Dashboard');
    await assertVisibleText(page, 'Settings');
    await assertVisibleText(page, 'Default Vim mode for editors');
    await assertVisibleText(page, 'This browser');
    assert.equal(await page.getByRole('heading', { name: 'Library', exact: true }).count(), 0);
    assert.equal(
        await page
            .getByRole('heading', { name: 'Local editor preferences', exact: true })
            .count(),
        0
    );
    assert.equal(await page.locator('.page-summary').count(), 0);
    await assertNoHeaderButtons(page);
    await assertStableMetadata(page, 'Orbit Ledger');
    await assertCreateActionBelowHome(page);
}

export async function expectAdminNote(page) {
    await expectFlatShell(page);
    assert.equal(await page.locator('#public-toggle').isChecked(), true);
    assert.equal(await page.locator('#favorite-toggle').isChecked(), true);
    assert.equal(await page.locator('#preview-toggle').getAttribute('aria-expanded'), 'false');
    assert.equal(await page.locator('.note-head .status-pill').count(), 0);
    await assertVisibleText(page, 'Delete note');
    await assertVisibleText(page, 'URL alias');
    await assertVisibleText(page, 'Canonical URL');
    await assertSingleHistoryCard(page);
    await assertLocalToastUiAssets(page);
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
