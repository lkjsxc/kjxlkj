import assert from 'node:assert/strict';
import { chromium } from 'playwright';
import { assertInvisibleText, assertVisibleText, expectAdminDashboard, expectAdminNote, expectClosedDrawer, expectGuestNote, expectPublicRoot, expectSearchPage, openDrawer } from './assertions.mjs';
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

async function verifyUiCreatedDraft(page) {
    await Promise.all([
        page.waitForURL((url) => new URL(url).pathname !== '/admin'),
        page.getByRole('button', { name: 'New note', exact: true }).first().click(),
    ]);
    await page.locator('[data-live-title]').first().waitFor({ state: 'visible' });
    const title = (await page.locator('[data-live-title]').first().textContent()).trim();
    assert.match(title, /^\d{4}-\d{2}-\d{2} \d{2}:\d{2}$/);
    assert.equal(
        await page.locator('#public-toggle').isChecked(),
        false,
        'new notes should default to private drafts'
    );
}

async function verifyEditorFormatting(browser, page, id) {
    const fence = '`'.repeat(3);
    const editor = page.locator('.toastui-editor-ww-container .ProseMirror').first();
    await editor.waitFor({ state: 'visible' });
    await page.evaluate(() => { window.editorInstance.focus(); window.editorInstance.moveCursorToEnd(); });
    await page.keyboard.press('Enter');
    await page.waitForTimeout(250);
    await page.keyboard.type('## ');
    await page.waitForTimeout(250);
    await page.keyboard.type('LiveHeading');
    await page.keyboard.press('Enter');
    await page.waitForTimeout(250);
    await page.evaluate(() => { window.editorInstance.focus(); window.editorInstance.moveCursorToEnd(); });
    await page.keyboard.press('Enter');
    await page.waitForTimeout(250);
    await page.keyboard.type('- ');
    await page.waitForTimeout(250);
    await page.keyboard.type('Alpha');
    await page.keyboard.press('Enter');
    await page.waitForTimeout(250);
    await page.evaluate(() => { window.editorInstance.focus(); window.editorInstance.moveCursorToEnd(); });
    await page.keyboard.press('Enter');
    await page.keyboard.press('Enter');
    await page.waitForTimeout(250);
    await page.keyboard.type('> ');
    await page.waitForTimeout(250);
    await page.keyboard.type('Quoted line');
    await page.keyboard.press('Enter');
    await page.keyboard.press('Enter');
    await page.waitForTimeout(250);
    await page.evaluate(() => { window.editorInstance.focus(); window.editorInstance.moveCursorToEnd(); });
    await page.keyboard.press('Enter');
    await page.waitForTimeout(250);
    await page.keyboard.type(fence);
    await page.keyboard.type('txt');
    await page.keyboard.press('Enter');
    await page.waitForTimeout(250);
    await page.keyboard.type('code');
    await page.waitForFunction(
        () =>
            !!document.querySelector('.toastui-editor-ww-container .toastui-editor-contents h2') &&
            document.querySelectorAll('.toastui-editor-ww-container .toastui-editor-contents li').length >= 1 &&
            !!document.querySelector('.toastui-editor-ww-container .toastui-editor-contents blockquote') &&
            !!document.querySelector('.toastui-editor-ww-container .toastui-editor-contents pre') &&
            Array.from(document.querySelectorAll('.toastui-editor-defaultUI-toolbar,.toastui-editor-toolbar')).every(
                (node) => node.scrollWidth - node.clientWidth <= 1 && node.scrollHeight - node.clientHeight <= 1
            )
    );
    await page.waitForTimeout(900);
    await page.reload({ waitUntil: 'networkidle' });
    await expectAdminNote(page);
    await page.waitForFunction(
        () =>
            !!document.querySelector('.toastui-editor-ww-container .toastui-editor-contents h2') &&
            document.querySelectorAll('.toastui-editor-ww-container .toastui-editor-contents li').length >= 1 &&
            !!document.querySelector('.toastui-editor-ww-container .toastui-editor-contents blockquote') &&
            !!document.querySelector('.toastui-editor-ww-container .toastui-editor-contents pre')
    );
    const guest = await newContext(browser, { width: 1440, height: 1100 });
    const guestPage = await guest.newPage();
    await guestPage.goto(`${appUrl}/${id}`, { waitUntil: 'networkidle' });
    await guestPage.waitForFunction(
        () =>
            !!document.querySelector('.prose h2') &&
            document.querySelectorAll('.prose li').length >= 1 &&
            !!document.querySelector('.prose blockquote') &&
            !!document.querySelector('.prose pre')
    );
    await assertVisibleText(guestPage, 'Alpha');
    await assertInvisibleText(guestPage, '* Alpha');
    await guest.close();
}

main().catch((error) => {
    console.error(error);
    process.exit(1);
});
