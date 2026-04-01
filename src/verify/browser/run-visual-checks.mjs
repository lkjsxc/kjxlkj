import assert from 'node:assert/strict';
import { chromium } from 'playwright';
import {
    assertVisibleText,
    expectAdminDashboard,
    expectAdminNote,
    expectGuestNote,
    expectPublicRoot,
    expectSearchPage,
    expectSettingsPage,
} from './assertions.mjs';
import { verifyFavoriteReorder } from './dashboard-checks.mjs';
import { verifyEditorFormatting, verifyUiCreatedDraft } from './editor-checks.mjs';
import { captureCompactScreens } from './responsive-checks.mjs';
import { appUrl, capture, login, newContext, prepareEnvironment, prepareState } from './support.mjs';
import {
    assertIconAssets,
    browseCardHrefs,
    configureSettings,
    expectAdminHome,
    popularTitles,
    resultTitles,
    saveHomeIntro,
} from './visual-helpers.mjs';
async function main() {
    await prepareEnvironment();
    const browser = await chromium.launch({ headless: true });
    try {
        const notes = await prepareState(browser);
        const desktopFont = await capturePublicScreens(browser, notes);
        await captureAdminScreens(browser, notes.middle);
        await captureCompactScreens(browser, notes.middle, desktopFont);
    } finally {
        await browser.close();
    }
    console.log(JSON.stringify({ command: 'visual-verify', status: 'pass', artifacts: ['desktop-public-root.png', 'desktop-search.png', 'desktop-admin-dashboard.png', 'desktop-settings.png', 'desktop-admin-home.png', 'desktop-admin-note.png', 'desktop-history-index.png', 'desktop-guest-note.png', 'desktop-login.png', 'compact-public-root-closed.png', 'compact-public-root-open.png', 'compact-admin-note.png', 'compact-admin-note-preview.png'] }));
}
async function captureAdminScreens(browser, note) {
    const context = await newContext(browser, { width: 1440, height: 1100 });
    const page = await context.newPage();
    await login(page);
    await page.goto(`${appUrl}/admin`, { waitUntil: 'networkidle' });
    await expectAdminDashboard(page);
    await capture(page, 'desktop-admin-dashboard.png');
    await Promise.all([
        page.waitForURL('**/settings'),
        page.getByRole('link', { name: 'Open settings', exact: true }).click(),
    ]);
    await expectSettingsPage(page);
    await verifyFavoriteReorder(page);
    await configureSettings(page);
    await capture(page, 'desktop-settings.png');
    await page.goto(`${appUrl}/`, { waitUntil: 'networkidle' });
    await expectAdminHome(page);
    await saveHomeIntro(page);
    await capture(page, 'desktop-admin-home.png');
    await verifyUiCreatedDraft(page, true);
    await page.goto(`${appUrl}/${note.id}`, { waitUntil: 'networkidle' });
    await expectAdminNote(page);
    const notePath = await verifyEditorFormatting(browser, page, note);
    await capture(page, 'desktop-admin-note.png');
    const historyJson = await page.evaluate(async (id) => {
        const response = await fetch(`/records/${id}/history?limit=2`);
        return response.json();
    }, note.id);
    assert.equal(historyJson.revisions.length, 2);
    assert.equal(typeof historyJson.next_cursor, 'string');
    await page.goto(`${appUrl}${notePath}/history?limit=2`, { waitUntil: 'networkidle' });
    assert.equal(new URL(page.url()).pathname, `${notePath}/history`);
    await assertVisibleText(page, 'Current note');
    await assertVisibleText(page, 'Revision 3');
    assert.equal(await page.getByRole('button', { name: 'Next', exact: true }).isDisabled(), false);
    await capture(page, 'desktop-history-index.png');
    await Promise.all([
        page.waitForURL((url) => new URL(url).searchParams.get('direction') === 'next'),
        page.getByRole('button', { name: 'Next', exact: true }).click(),
    ]);
    await assertVisibleText(page, 'Revision 1');
    await page.goto(`${appUrl}${notePath}/history/3`, { waitUntil: 'networkidle' });
    assert.equal(new URL(page.url()).pathname, `${notePath}/history/3`);
    await assertVisibleText(page, 'Shared release');
    await Promise.all([
        page.waitForURL('**/'),
        page.getByRole('button', { name: 'Logout', exact: true }).first().click(),
    ]);
    await assertVisibleText(page, 'Home');
    await capture(page, 'desktop-login.png');
    await context.close();
}
async function capturePublicScreens(browser, notes) {
    const context = await newContext(browser, { width: 1440, height: 1100 });
    const page = await context.newPage();
    await page.goto(`${appUrl}/`, { waitUntil: 'networkidle' });
    await expectPublicRoot(page);
    await assertIconAssets(page);
    assert.equal(await page.getByRole('link', { name: '30d', exact: true }).getAttribute('class'), 'btn btn-primary');
    assert.equal((await popularTitles(page))[0], 'Beacon Log');
    assert.deepEqual(await browseCardHrefs(page), [
        '/search?scope=popular&popular_window=30d&sort=popular',
        '/search?sort=updated_desc',
        '/search?scope=favorites&sort=favorite_order',
    ]);
    await capture(page, 'desktop-public-root.png');
    await Promise.all([
        page.waitForURL((url) => new URL(url).searchParams.get('popular_window') === '90d'),
        page.getByRole('link', { name: '90d', exact: true }).click(),
    ]);
    assert.equal((await popularTitles(page))[0], 'Atlas Entry');
    await Promise.all([
        page.waitForURL((url) => new URL(url).searchParams.get('popular_window') === '30d'),
        page.getByRole('link', { name: '30d', exact: true }).click(),
    ]);
    await page.goto(`${appUrl}/search?limit=2`, { waitUntil: 'networkidle' });
    await expectSearchPage(page);
    await assertVisibleText(page, 'Notes');
    await assertVisibleText(page, notes.newest.title);
    await assertVisibleText(page, notes.middle.title);
    assert.equal(await page.locator('#search-sort').inputValue(), 'updated_desc');
    assert.equal(await page.getByRole('button', { name: 'Previous', exact: true }).isDisabled(), true);
    assert.equal(await page.getByRole('button', { name: 'Next', exact: true }).isDisabled(), false);
    await capture(page, 'desktop-search.png');
    await Promise.all([
        page.waitForURL((url) => new URL(url).searchParams.get('direction') === 'next'),
        page.getByRole('button', { name: 'Next', exact: true }).click(),
    ]);
    await assertVisibleText(page, notes.oldest.title);
    assert.equal(await page.getByRole('button', { name: 'Previous', exact: true }).isDisabled(), false);
    await Promise.all([
        page.waitForURL((url) => new URL(url).searchParams.get('direction') === 'prev'),
        page.getByRole('button', { name: 'Previous', exact: true }).click(),
    ]);
    await assertVisibleText(page, notes.newest.title);
    await page.locator('#search-sort').selectOption('title_desc');
    await Promise.all([
        page.waitForURL((url) => new URL(url).searchParams.get('sort') === 'title_desc'),
        page.getByRole('button', { name: 'Search', exact: true }).click(),
    ]);
    await page.waitForLoadState('networkidle');
    const titles = await page.locator('.note-grid .card-title').evaluateAll((nodes) =>
        nodes.map((node) => node.textContent.trim())
    );
    assert.equal(titles[0], 'Orbit Ledger');
    await page.goto(`${appUrl}/search?q=orbit`, { waitUntil: 'networkidle' });
    await expectSearchPage(page, { hasQueryCard: true });
    await page.goto(`${appUrl}/search?scope=favorites`, { waitUntil: 'networkidle' });
    await expectSearchPage(page, { scopeValue: 'Favorites' });
    assert.equal(await page.locator('#search-sort').inputValue(), 'favorite_order');
    assert.deepEqual((await resultTitles(page)).slice(0, 2), ['Orbit Ledger', 'Beacon Log']);
    await page.goto(`${appUrl}/search?scope=popular&popular_window=90d`, { waitUntil: 'networkidle' });
    await expectSearchPage(page, { scopeValue: 'Popular 90d' });
    assert.equal(await page.locator('#search-sort').inputValue(), 'popular');
    assert.equal((await resultTitles(page))[0], 'Atlas Entry');
    await Promise.all([
        page.waitForURL((url) => new URL(url).searchParams.get('popular_window') === '30d'),
        page.getByRole('link', { name: '30d', exact: true }).click(),
    ]);
    assert.equal((await resultTitles(page))[0], 'Beacon Log');
    await page.goto(`${appUrl}/${notes.middle.id}`, { waitUntil: 'networkidle' });
    assert.equal(new URL(page.url()).pathname, `/${notes.middle.ref}`);
    await expectGuestNote(page, notes.oldest.title, notes.newest.title);
    await capture(page, 'desktop-guest-note.png');
    await page.goto(`${appUrl}/${notes.oldest.ref}`, { waitUntil: 'networkidle' });
    await expectGuestNote(page, null, notes.middle.title);
    await page.goto(`${appUrl}/${notes.newest.ref}`, { waitUntil: 'networkidle' });
    await expectGuestNote(page, notes.middle.title, null);
    const publicRevision = await page.goto(`${appUrl}/${notes.middle.id}/history/3`, { waitUntil: 'networkidle' });
    assert.equal(new URL(page.url()).pathname, `/${notes.middle.ref}/history/3`);
    const privateRevision = await page.goto(`${appUrl}/${notes.middle.id}/history/2`, { waitUntil: 'networkidle' });
    assert.equal(new URL(page.url()).pathname, `/${notes.middle.ref}/history/2`);
    assert.equal(publicRevision?.status(), 200, 'public revision should stay guest-readable');
    assert.equal(privateRevision?.status(), 404, 'private revision should return 404');
    await assertVisibleText(page, 'Note not found');
    const fontFamily = await page.evaluate(() => getComputedStyle(document.body).fontFamily);
    await context.close();
    return fontFamily;
}
main().catch((error) => {
    console.error(error);
    process.exit(1);
});
