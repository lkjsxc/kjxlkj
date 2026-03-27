import assert from 'node:assert/strict';
import { chromium } from 'playwright';
import { assertInvisibleText, assertVisibleText, expectAdminDashboard, expectAdminNote, expectClosedDrawer, expectGuestNote, expectPublicRoot, expectSearchPage, openDrawer } from './assertions.mjs';
import { verifyEditorFormatting, verifyUiCreatedDraft } from './editor-checks.mjs';
import { appUrl, capture, login, newContext, prepareEnvironment, prepareState } from './support.mjs';

async function main() {
    await prepareEnvironment();

    const browser = await chromium.launch({ headless: true });
    try {
        const notes = await prepareState(browser);
        await capturePublicScreens(browser, notes);
        await captureAdminScreens(browser, notes.middle.id);
        await captureCompactScreens(browser, notes.middle.id);
    } finally {
        await browser.close();
    }

    console.log(JSON.stringify({ command: 'visual-verify', status: 'pass', artifacts: [
        'desktop-public-root.png', 'desktop-search.png', 'desktop-admin-dashboard.png',
        'desktop-admin-note.png', 'desktop-history-index.png', 'desktop-guest-note.png',
        'compact-public-root-closed.png', 'compact-public-root-open.png', 'compact-admin-note.png',
    ] }));
}

async function captureAdminScreens(browser, id) {
    const context = await newContext(browser, { width: 1440, height: 1100 });
    const page = await context.newPage();
    await login(page);

    await page.goto(`${appUrl}/admin`, { waitUntil: 'networkidle' });
    await expectAdminDashboard(page);
    await capture(page, 'desktop-admin-dashboard.png');
    await verifyUiCreatedDraft(page);

    await page.goto(`${appUrl}/${id}`, { waitUntil: 'networkidle' });
    await expectAdminNote(page);
    await verifyEditorFormatting(browser, page, id);
    await capture(page, 'desktop-admin-note.png');

    await page.goto(`${appUrl}/${id}/history`, { waitUntil: 'networkidle' });
    await assertVisibleText(page, 'Current note');
    await assertVisibleText(page, 'Revision 3');
    await capture(page, 'desktop-history-index.png');

    await page.goto(`${appUrl}/${id}/history/3`, { waitUntil: 'networkidle' });
    await assertVisibleText(page, 'Shared release');
    await context.close();
}

async function capturePublicScreens(browser, notes) {
    const context = await newContext(browser, { width: 1440, height: 1100 });
    const page = await context.newPage();

    await page.goto(`${appUrl}/`, { waitUntil: 'networkidle' });
    await expectPublicRoot(page);
    await capture(page, 'desktop-public-root.png');

    await page.goto(`${appUrl}/search?q=Orbit`, { waitUntil: 'networkidle' });
    await expectSearchPage(page);
    await capture(page, 'desktop-search.png');

    await page.goto(`${appUrl}/${notes.middle.id}`, { waitUntil: 'networkidle' });
    await expectGuestNote(page, notes.oldest.title, notes.newest.title);
    await capture(page, 'desktop-guest-note.png');

    await page.goto(`${appUrl}/${notes.oldest.id}`, { waitUntil: 'networkidle' });
    await expectGuestNote(page, null, notes.middle.title);

    await page.goto(`${appUrl}/${notes.newest.id}`, { waitUntil: 'networkidle' });
    await expectGuestNote(page, notes.middle.title, null);

    const publicRevision = await page.goto(`${appUrl}/${notes.middle.id}/history/3`, { waitUntil: 'networkidle' });
    const privateRevision = await page.goto(`${appUrl}/${notes.middle.id}/history/2`, { waitUntil: 'networkidle' });
    assert.equal(publicRevision?.status(), 200, 'public revision should stay guest-readable');
    assert.equal(privateRevision?.status(), 404, 'private revision should return 404');
    await assertVisibleText(page, 'Note not found');
    await context.close();
}

async function captureCompactScreens(browser, id) {
    const context = await newContext(browser, { width: 360, height: 844 });
    const page = await context.newPage();

    await page.goto(`${appUrl}/`, { waitUntil: 'networkidle' });
    await expectPublicRoot(page);
    await expectClosedDrawer(page);
    await capture(page, 'compact-public-root-closed.png');

    await openDrawer(page);
    await capture(page, 'compact-public-root-open.png');

    await login(page);
    await page.goto(`${appUrl}/${id}`, { waitUntil: 'networkidle' });
    await expectAdminNote(page);
    await expectClosedDrawer(page);
    await capture(page, 'compact-admin-note.png');
    await context.close();
}

main().catch((error) => {
    console.error(error);
    process.exit(1);
});
