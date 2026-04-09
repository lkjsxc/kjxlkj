import assert from 'node:assert/strict';
import { chromium } from 'playwright';
import { assertVisibleText, expectAdminDashboard, expectAdminNote, expectGuestNote, expectPublicRoot, expectSearchPage, expectSettingsPage } from './assertions.mjs';
import { applySettingsScenario, verifyFavoriteReorder } from './dashboard-checks.mjs';
import { assertBrandName, assertDiscoveryDisabled, assertDiscoveryRoutes, assertHead } from './discoverability-checks.mjs';
import { verifyEditorFormatting, verifyUiCreatedDraft } from './editor-checks.mjs';
import { assertAdminHomeConfiguration, assertHomeBrowseLinks, assertPopularWindowSwitch, popularTitles } from './home-checks.mjs';
import { assertIconAssets } from './icon-checks.mjs';
import { assertMediaSearchFilter, assertPublicMediaPage, verifyUiCreatedMedia } from './media-checks.mjs';
import { captureCompactScreens } from './responsive-checks.mjs';
import { appUrl, capture, login, newContext, prepareEnvironment, prepareState } from './support.mjs';

async function main() {
    await prepareEnvironment();
    const browser = await chromium.launch({ headless: true });
    try {
        const fixtures = await prepareState(browser);
        const desktopFont = await capturePublicScreens(browser, fixtures);
        await captureAdminScreens(browser, fixtures);
        await captureCompactScreens(browser, fixtures.middle, desktopFont);
    } finally {
        await browser.close();
    }
    console.log(JSON.stringify({ command: 'visual-verify', status: 'pass', artifacts: ['desktop-public-root.png', 'desktop-search.png', 'desktop-admin-dashboard.png', 'desktop-admin-note.png', 'desktop-history-index.png', 'desktop-guest-note.png', 'desktop-login.png', 'compact-public-root-closed.png', 'compact-public-root-open.png', 'compact-admin-note.png', 'compact-admin-note-preview.png', 'compact-history-index.png'] }));
}

async function captureAdminScreens(browser, fixtures) {
    const context = await newContext(browser, { width: 1440, height: 1100 });
    const page = await context.newPage();
    const note = fixtures.middle;
    const latestSnapshot = note.snapshots.find((item) => item.snapshot_number === 4);
    assert.ok(latestSnapshot, 'expected latest snapshot fixture');
    await page.goto(`${appUrl}/login`, { waitUntil: 'networkidle' });
    await assertHead(page, { title: 'Login | kjxlkj', descriptionIncludes: 'Sign in to manage kjxlkj.', robots: 'noindex,nofollow', canonical: null });
    await login(page);

    await page.goto(`${appUrl}/admin`, { waitUntil: 'networkidle' });
    await expectAdminDashboard(page);
    await assertHead(page, { title: 'Dashboard | kjxlkj', descriptionIncludes: 'Admin dashboard for kjxlkj.', robots: 'noindex,nofollow', canonical: null });
    await assertPopularWindowSwitch(page, '/admin', 'admin');
    await verifyFavoriteReorder(page);
    await capture(page, 'desktop-admin-dashboard.png');
    await Promise.all([
        page.waitForURL('**/admin/settings'),
        page.getByRole('link', { name: 'Open settings', exact: true }).click(),
    ]);
    await expectSettingsPage(page);
    await assertHead(page, { title: 'Settings | kjxlkj', descriptionIncludes: 'Admin settings for kjxlkj.', robots: 'noindex,nofollow', canonical: null });
    await applySettingsScenario(page);
    await assertBrandName(page, 'Launchpad');

    await page.goto(`${appUrl}/`, { waitUntil: 'networkidle' });
    await assertAdminHomeConfiguration(page);
    await assertHead(page, { title: 'Home | Launchpad', descriptionIncludes: 'Launchpad search surface for public resources.', robots: 'noindex,nofollow', canonical: null });
    await verifyUiCreatedDraft(page, false);

    await page.goto(`${appUrl}/${note.id}`, { waitUntil: 'networkidle' });
    assert.equal(new URL(page.url()).pathname, `/${note.ref}`);
    await expectAdminNote(page);
    await assertHead(page, { title: `${note.title} | Launchpad`, descriptionIncludes: 'Current shared snapshot stretches across the list card', robots: 'noindex,nofollow', canonical: null });
    await verifyEditorFormatting(browser, page, note, fixtures);
    await capture(page, 'desktop-admin-note.png');

    const historyJson = await page.evaluate(async (id) => {
        const response = await fetch(`/resources/${id}/history?limit=2`);
        return response.json();
    }, note.id);
    assert.equal(historyJson.snapshots.length, 2); assert.equal(typeof historyJson.snapshots[0]?.id, 'string');
    assert.equal(typeof historyJson.next_cursor, 'string');

    await page.goto(`${appUrl}/${note.id}/history?limit=2`, { waitUntil: 'networkidle' });
    assert.equal(new URL(page.url()).pathname, `/${note.ref}/history`);
    await Promise.all([assertVisibleText(page, 'Live note'), assertVisibleText(page, 'Open GitHub')]);
    await assertVisibleText(page, 'Latest saved snapshot');
    await assertVisibleText(page, 'Saved snapshot 4');
    await assertHead(page, { title: `History: ${note.title} | Launchpad`, descriptionIncludes: `Saved snapshots for ${note.title}.`, robots: 'noindex,nofollow', canonical: null });
    assert.equal(await page.getByRole('button', { name: 'Next', exact: true }).isDisabled(), false);
    await capture(page, 'desktop-history-index.png');
    await Promise.all([
        page.waitForURL((url) => new URL(url).searchParams.get('direction') === 'next'),
        page.getByRole('button', { name: 'Next', exact: true }).click(),
    ]);
    await assertVisibleText(page, 'Saved snapshot 3');

    await page.goto(`${appUrl}/${latestSnapshot.id}`, { waitUntil: 'networkidle' });
    assert.equal(new URL(page.url()).pathname, `/${latestSnapshot.id}`);
    await assertHead(page, { title: `Saved snapshot 4: ${note.title} | Launchpad`, descriptionIncludes: 'Saved snapshot 4 for Orbit Ledger.', robots: 'noindex,nofollow', canonical: null });
    await assertVisibleText(page, 'Current shared snapshot stretches across the list card');
    await verifyUiCreatedMedia(page, note);
    await Promise.all([
        page.waitForURL('**/'),
        page.getByRole('button', { name: 'Logout', exact: true }).first().click(),
    ]);
    await assertVisibleText(page, 'Home');
    await assertHead(page, { title: 'Home | Launchpad', descriptionIncludes: 'Launchpad search surface for public resources.', robots: 'index,follow', canonical: `${appUrl}/` });
    await assertDiscoveryRoutes(page, { sitemapContains: [`${appUrl}/</loc>`, `${appUrl}/${note.ref}</loc>`, `${appUrl}/${fixtures.image.ref}</loc>`] });
    await capture(page, 'desktop-login.png');
    await page.goto(`${appUrl}/${note.ref}`, { waitUntil: 'networkidle' });
    await assertHead(page, { title: `${note.title} | Launchpad`, descriptionIncludes: 'Current shared snapshot stretches across the list card', robots: 'index,follow', canonical: `${appUrl}/${note.ref}` });
    await context.close();
}

async function capturePublicScreens(browser, notes) {
    const context = await newContext(browser, { width: 1440, height: 1100 });
    const page = await context.newPage();
    const publicSnapshot = notes.middle.snapshots.find((item) => item.snapshot_number === 4);
    const privateSnapshot = notes.middle.snapshots.find((item) => item.snapshot_number === 2);
    assert.ok(publicSnapshot, 'expected public snapshot fixture');
    assert.ok(privateSnapshot, 'expected private snapshot fixture');

    await page.goto(`${appUrl}/`, { waitUntil: 'networkidle' });
    await expectPublicRoot(page);
    await assertBrandName(page, 'kjxlkj');
    await assertHead(page, { title: 'Home | kjxlkj', descriptionIncludes: 'Markdown-first resource system for LLM-operated workflows.', robots: 'noindex,nofollow', canonical: null });
    await assertDiscoveryDisabled(page);
    await assertIconAssets(page);
    assert.equal(await page.getByRole('button', { name: '30d', exact: true }).getAttribute('class'), 'btn btn-primary');
    assert.equal((await popularTitles(page))[0], 'Beacon Log');
    await assertHomeBrowseLinks(page);
    await capture(page, 'desktop-public-root.png');
    await assertPopularWindowSwitch(page, '/', 'home');

    await page.goto(`${appUrl}/search?scope=favorites`, { waitUntil: 'networkidle' });
    await expectSearchPage(page, false);
    await assertHead(page, { title: 'Search | kjxlkj', descriptionIncludes: 'Markdown-first resource system for LLM-operated workflows.', robots: 'noindex,nofollow', canonical: null });
    assert.equal(await page.locator('#search-sort').inputValue(), 'favorite_position_asc'); assert.equal(await page.getByText('Atlas Entry', { exact: true }).count(), 0);
    await assertVisibleText(page, 'Beacon Log');
    await assertVisibleText(page, 'Orbit Ledger');

    await page.goto(`${appUrl}/search?limit=2`, { waitUntil: 'networkidle' });
    await expectSearchPage(page, false);
    await assertVisibleText(page, 'Resources');
    assert.equal(await page.getByText('Query', { exact: true }).count(), 0);
    await assertVisibleText(page, notes.newest.title);
    await assertVisibleText(page, notes.middle.title);
    assert.equal(await page.locator('#search-sort').inputValue(), 'updated_desc');
    assert.equal(await page.getByRole('button', { name: 'Prev', exact: true }).isDisabled(), true);
    assert.equal(await page.getByRole('button', { name: 'Next', exact: true }).isDisabled(), false);
    await capture(page, 'desktop-search.png');

    await Promise.all([
        page.waitForURL((url) => new URL(url).searchParams.get('direction') === 'next'),
        page.getByRole('button', { name: 'Next', exact: true }).click(),
    ]);
    await assertVisibleText(page, notes.oldest.title);
    assert.equal(await page.getByRole('button', { name: 'Prev', exact: true }).isDisabled(), false);
    await Promise.all([
        page.waitForURL((url) => new URL(url).searchParams.get('direction') === 'prev'),
        page.getByRole('button', { name: 'Prev', exact: true }).click(),
    ]);
    await assertVisibleText(page, notes.newest.title);

    await page.locator('#search-sort').selectOption('title_desc');
    await Promise.all([
        page.waitForURL((url) => new URL(url).searchParams.get('sort') === 'title_desc'),
        page.getByRole('button', { name: 'Search', exact: true }).click(),
    ]);
    await page.waitForLoadState('networkidle');
    const titles = await page.locator('.resource-grid .resource-row[data-card-title]').evaluateAll((nodes) =>
        nodes.map((node) => node.dataset.cardTitle.trim())
    );
    assert.equal(titles[0], notes.image.title);

    await page.goto(`${appUrl}/search?q=orbit`, { waitUntil: 'networkidle' });
    await expectSearchPage(page, true);
    await assertMediaSearchFilter(page, notes, notes.middle.title);

    await page.goto(`${appUrl}/${notes.middle.id}`, { waitUntil: 'networkidle' });
    assert.equal(new URL(page.url()).pathname, `/${notes.middle.ref}`);
    await expectGuestNote(page, notes.oldest.title, notes.newest.title);
    await assertHead(page, { title: `${notes.middle.title} | kjxlkj`, descriptionIncludes: 'Current shared snapshot stretches across the list card', robots: 'noindex,nofollow', canonical: null });
    await assertVisibleText(page, 'Oldest public note.');
    await assertVisibleText(page, 'Newest public note.');
    await capture(page, 'desktop-guest-note.png');

    await page.goto(`${appUrl}/${notes.oldest.ref}`, { waitUntil: 'networkidle' });
    await expectGuestNote(page, notes.video.title, notes.middle.title);

    await page.goto(`${appUrl}/${notes.newest.ref}`, { waitUntil: 'networkidle' });
    await expectGuestNote(page, notes.middle.title, null);
    await assertPublicMediaPage(page, notes.image);
    await assertPublicMediaPage(page, notes.video);

    const publicSnapshotResponse = await page.goto(`${appUrl}/${publicSnapshot.id}`, { waitUntil: 'networkidle' });
    assert.equal(new URL(page.url()).pathname, `/${publicSnapshot.id}`);
    await assertHead(page, { title: `Saved snapshot 4: ${notes.middle.title} | kjxlkj`, descriptionIncludes: 'Saved snapshot 4 for Orbit Ledger.', robots: 'noindex,nofollow', canonical: null });
    const privateSnapshotResponse = await page.goto(`${appUrl}/${privateSnapshot.id}`, { waitUntil: 'networkidle' });
    assert.equal(new URL(page.url()).pathname, `/${privateSnapshot.id}`);
    assert.equal(publicSnapshotResponse?.status(), 200, 'public snapshot should stay guest-readable');
    assert.equal(privateSnapshotResponse?.status(), 404, 'private snapshot should return 404');
    await assertHead(page, { title: 'Not Found | kjxlkj', descriptionIncludes: 'could not be found', robots: 'noindex,nofollow', canonical: null });
    await assertVisibleText(page, 'Resource not found');
    const fontFamily = await page.evaluate(() => getComputedStyle(document.body).fontFamily);
    await context.close();
    return fontFamily;
}
main().catch((error) => {
    console.error(error);
    process.exit(1);
});
