import assert from 'node:assert/strict';
import { chromium } from 'playwright';
import {
    assertVisibleText,
    expectAdminDashboard,
    expectAdminNote,
    expectClosedDrawer,
    expectGuestNote,
    expectPublicRoot,
    expectSearchPage,
    openDrawer,
} from './assertions.mjs';
import { assertEditorLayout, openPreview, verifyEditorFormatting, verifyUiCreatedDraft } from './editor-checks.mjs';
import { appUrl, capture, login, newContext, prepareEnvironment, prepareState } from './support.mjs';

async function main() {
    await prepareEnvironment();

    const browser = await chromium.launch({ headless: true });
    try {
        const notes = await prepareState(browser);
        await capturePublicScreens(browser, notes);
        await captureAdminScreens(browser, notes.middle);
        await captureCompactScreens(browser, notes.middle);
    } finally {
        await browser.close();
    }

    console.log(JSON.stringify({ command: 'visual-verify', status: 'pass', artifacts: [
        'desktop-public-root.png', 'desktop-search.png', 'desktop-admin-dashboard.png',
        'desktop-admin-note.png', 'desktop-history-index.png', 'desktop-guest-note.png',
        'desktop-login.png',
        'compact-public-root-closed.png', 'compact-public-root-open.png',
        'compact-admin-note.png', 'compact-admin-note-preview.png',
    ] }));
}

async function captureAdminScreens(browser, note) {
    const context = await newContext(browser, { width: 1440, height: 1100 });
    const page = await context.newPage();
    await login(page);

    await page.goto(`${appUrl}/admin`, { waitUntil: 'networkidle' });
    await expectAdminDashboard(page);
    await capture(page, 'desktop-admin-dashboard.png');
    assert.equal(await page.locator('#local-vim-mode').inputValue(), 'default');
    const defaultVim = page.locator('input[name="default_vim_mode"]').first();
    assert.equal(await defaultVim.isChecked(), false, 'dashboard should default Vim mode off');
    await defaultVim.check();
    await Promise.all([
        page.waitForURL('**/admin'),
        page.getByRole('button', { name: 'Save settings', exact: true }).click(),
    ]);
    assert.equal(await page.locator('input[name="default_vim_mode"]').first().isChecked(), true);
    await verifyUiCreatedDraft(page, true);

    await page.goto(`${appUrl}/admin`, { waitUntil: 'networkidle' });
    await page.locator('#local-vim-mode').selectOption('off');
    await page.waitForFunction(() => window.localStorage.getItem('kjxlkj.vim-mode') === 'off');

    await page.goto(`${appUrl}/${note.id}`, { waitUntil: 'networkidle' });
    assert.equal(new URL(page.url()).pathname, `/${note.ref}`);
    await expectAdminNote(page);
    await assertVisibleText(page, 'Vim off');
    await verifyEditorFormatting(browser, page, note, false);
    await capture(page, 'desktop-admin-note.png');

    await page.goto(`${appUrl}/${note.id}/history`, { waitUntil: 'networkidle' });
    assert.equal(new URL(page.url()).pathname, `/${note.ref}/history`);
    await assertVisibleText(page, 'Current note');
    await assertVisibleText(page, 'Revision 3');
    await capture(page, 'desktop-history-index.png');

    await page.goto(`${appUrl}/${note.id}/history/3`, { waitUntil: 'networkidle' });
    assert.equal(new URL(page.url()).pathname, `/${note.ref}/history/3`);
    await assertVisibleText(page, 'Shared release');
    await Promise.all([
        page.waitForURL('**/login'),
        page.getByRole('button', { name: 'Logout', exact: true }).first().click(),
    ]);
    await assertVisibleText(page, 'kjxlkj');
    assert.equal(await page.locator('.auth-card .subtitle').count(), 0);
    await capture(page, 'desktop-login.png');
    await context.close();
}

async function capturePublicScreens(browser, notes) {
    const context = await newContext(browser, { width: 1440, height: 1100 });
    const page = await context.newPage();

    await page.goto(`${appUrl}/`, { waitUntil: 'networkidle' });
    await expectPublicRoot(page);
    await capture(page, 'desktop-public-root.png');

    const browseCard = page.getByRole('link', { name: /View more notes/i }).first();
    assert.equal(await browseCard.getAttribute('href'), '/search');
    await page.goto(`${appUrl}/search?limit=2`, { waitUntil: 'networkidle' });
    await expectSearchPage(page);
    await assertVisibleText(page, 'All notes');
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
    await context.close();
}

async function captureCompactScreens(browser, note) {
    const context = await newContext(browser, { width: 360, height: 844 });
    const page = await context.newPage();

    await page.goto(`${appUrl}/`, { waitUntil: 'networkidle' });
    await expectPublicRoot(page);
    await expectClosedDrawer(page);
    await capture(page, 'compact-public-root-closed.png');

    await openDrawer(page);
    await capture(page, 'compact-public-root-open.png');

    await login(page);
    await page.goto(`${appUrl}/admin`, { waitUntil: 'networkidle' });
    await page.locator('#local-vim-mode').selectOption('off');
    await page.waitForFunction(() => window.localStorage.getItem('kjxlkj.vim-mode') === 'off');
    await page.goto(`${appUrl}/${note.id}`, { waitUntil: 'networkidle' });
    assert.equal(new URL(page.url()).pathname, `/${note.ref}`);
    await expectAdminNote(page);
    await assertVisibleText(page, 'Vim off');
    await expectClosedDrawer(page);
    await capture(page, 'compact-admin-note.png');
    await openPreview(page);
    await assertEditorLayout(page, true);
    await capture(page, 'compact-admin-note-preview.png');
    await context.close();
}

main().catch((error) => {
    console.error(error);
    process.exit(1);
});
