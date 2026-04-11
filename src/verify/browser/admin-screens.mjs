import assert from 'node:assert/strict';
import { assertVisibleText, expectAdminDashboard, expectAdminNote, expectSettingsPage } from './assertions.mjs';
import { applySettingsScenario, verifyFavoriteReorder } from './dashboard-checks.mjs';
import { assertBrandName, assertDiscoveryRoutes, assertHead } from './discoverability-checks.mjs';
import { verifyEditorFormatting, verifyUiCreatedDraft } from './editor-checks.mjs';
import { assertAdminHomeConfiguration, assertPopularWindowSwitch } from './home-checks.mjs';
import { verifyUiCreatedMedia } from './media-checks.mjs';
import { verifyPartialResourceNavigation } from './navigation-checks.mjs';
import { appUrl, capture, login, newContext } from './support.mjs';

export async function captureAdminScreens(browser, fixtures) {
    const context = await newContext(browser, { width: 1440, height: 1100 });
    const page = await context.newPage();
    const note = fixtures.middle;
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
    await verifyPartialResourceNavigation(page, note, fixtures.oldest);
    await capture(page, 'desktop-admin-note.png');

    const historyJson = await page.evaluate(async (id) => {
        const response = await fetch(`/resources/${id}/history?limit=2`);
        return response.json();
    }, note.id);
    assert.equal(historyJson.snapshots.length, 2);
    assert.equal(typeof historyJson.snapshots[0]?.id, 'string');
    assert.equal(typeof historyJson.snapshots[0]?.snapshot_number, 'number');
    assert.equal(typeof historyJson.snapshots[1]?.snapshot_number, 'number');
    assert.equal(typeof historyJson.next_cursor, 'string');
    const nextHistoryJson = await page.evaluate(async ({ id, cursor }) => {
        const response = await fetch(
            `/resources/${id}/history?limit=2&direction=next&cursor=${encodeURIComponent(cursor)}`
        );
        return response.json();
    }, { id: note.id, cursor: historyJson.next_cursor });
    assert.equal(typeof nextHistoryJson.snapshots[0]?.snapshot_number, 'number');
    const latestSnapshot = historyJson.snapshots[0];

    await page.goto(`${appUrl}/${note.id}/history?limit=2`, { waitUntil: 'networkidle' });
    assert.equal(new URL(page.url()).pathname, `/${note.ref}/history`);
    await Promise.all([assertVisibleText(page, 'Live note'), assertVisibleText(page, 'Open GitHub')]);
    await assertVisibleText(page, 'Latest saved snapshot');
    await assertVisibleText(page, `Saved snapshot ${historyJson.snapshots[1].snapshot_number}`);
    await assertHead(page, { title: `History: ${note.title} | Launchpad`, descriptionIncludes: `Saved snapshots for ${note.title}.`, robots: 'noindex,nofollow', canonical: null });
    assert.equal(await page.getByRole('button', { name: 'Next', exact: true }).isDisabled(), false);
    await capture(page, 'desktop-history-index.png');
    await Promise.all([
        page.waitForURL((url) => new URL(url).searchParams.get('direction') === 'next'),
        page.getByRole('button', { name: 'Next', exact: true }).click(),
    ]);
    await assertVisibleText(page, `Saved snapshot ${nextHistoryJson.snapshots[0].snapshot_number}`);

    await page.goto(`${appUrl}/${latestSnapshot.id}`, { waitUntil: 'networkidle' });
    assert.equal(new URL(page.url()).pathname, `/${latestSnapshot.id}`);
    await assertHead(page, {
        title: `Saved snapshot ${latestSnapshot.snapshot_number}: ${note.title} | Launchpad`,
        descriptionIncludes: `Saved snapshot ${latestSnapshot.snapshot_number} for ${note.title}.`,
        robots: 'noindex,nofollow',
        canonical: null,
    });
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
