import assert from 'node:assert/strict';
import { assertInvisibleText, assertVisibleText } from './assertions.mjs';
import { assertNoHorizontalOverflow } from './shell-assertions.mjs';
import { appUrl, newContext } from './support.mjs';

export async function verifyUiCreatedDraft(page, expectedPublic = false) {
    await Promise.all([
        page.waitForURL((url) => !['/', '/admin', '/settings'].includes(new URL(url).pathname)),
        page.getByRole('button', { name: 'New note', exact: true }).first().click(),
    ]);
    await page.locator('#editor-source').waitFor({ state: 'visible' });
    const title = (await page.locator('[data-live-title]').first().textContent()).trim();
    assert.match(title, /^\d{4}-\d{2}-\d{2} \d{2}:\d{2}$/);
    assert.equal(await page.locator('#public-toggle').isChecked(), expectedPublic);
    await page.getByRole('button', { name: 'Show preview', exact: true }).waitFor({ state: 'visible' });
    assert.equal(await page.locator('#preview-toggle').getAttribute('aria-expanded'), 'false');
    assert.equal((await page.locator('#preview-state').textContent()).trim(), 'Closed');
    await expectEditorFocus(page);
}

export async function verifyEditorFormatting(browser, page, note) {
    const saveRequests = [];
    page.on('requestfinished', (request) => {
        if (request.method() === 'PUT' && new URL(request.url()).pathname === `/records/${note.id}`) {
            saveRequests.push(Date.now());
        }
    });
    await page.locator('#editor-source').waitFor({ state: 'visible' });
    await expectEditorFocus(page);
    await assertEditorChrome(page);
    await page.waitForTimeout(900);
    assert.equal(saveRequests.length, 0, 'idle note should not save before edits');
    await appendMarkdown(page);
    await openPreview(page);
    await waitForPreviewStructures(page);
    await assertEditorLayout(page, false);
    await assertAccentLink(page, '.editor-preview a');
    await page.waitForTimeout(1400);
    assert.ok(saveRequests.length >= 1, 'editing should trigger autosave');
    const settledCount = saveRequests.length;
    await page.waitForTimeout(900);
    assert.equal(saveRequests.length, settledCount, 'autosave should settle once edits are saved');
    await page.reload({ waitUntil: 'networkidle' });
    const expectedAlias = 'orbit-ledger_v2.test';
    assert.equal(await page.locator('#preview-toggle').getAttribute('aria-expanded'), 'false');
    assert.equal(await page.locator('#alias-input').inputValue(), expectedAlias);
    assert.equal(new URL(page.url()).pathname, `/${expectedAlias}`);
    await openPreview(page);
    await waitForPreviewStructures(page);
    await verifyGuestRender(browser, `/${expectedAlias}`);
    return `/${expectedAlias}`;
}

export async function openPreview(page) {
    const toggle = page.locator('#preview-toggle');
    await toggle.waitFor({ state: 'visible' });
    await toggle.click();
    await page.waitForFunction(
        () =>
            document.querySelector('#preview-toggle')?.getAttribute('aria-expanded') === 'true' &&
            !!document.querySelector('#editor-preview:not([hidden])')
    );
}

export async function assertEditorLayout(page, compact) {
    await page.waitForFunction((isCompact) => {
        const preview = document.querySelector('#editor-preview');
        const editor = document.querySelector('#editor-source');
        const shell = document.querySelector('#editor-shell');
        const backdrop = document.querySelector('#preview-backdrop');
        if (!preview || !editor || !shell || !backdrop) return false;
        const previewStyle = getComputedStyle(preview);
        if (isCompact) return previewStyle.position === 'fixed' && !backdrop.hidden && shell.classList.contains('preview-compact');
        const sideBySide = previewStyle.position !== 'fixed' &&
            preview.getBoundingClientRect().left >= editor.getBoundingClientRect().right - 4;
        return sideBySide && backdrop.hidden && !shell.classList.contains('preview-compact');
    }, compact);
}

async function appendMarkdown(page) {
    await page.locator('#alias-input').fill('orbit ledger_v2.test');
    await page.locator('#alias-input').blur();
    await moveCursorToEnd(page);
    await page.locator('#editor-source').type(
        '\n## Live Heading\n\n[Docs](https://example.com/very-long-link-for-wrap-testing)\n\n- Alpha\n\n> Quoted line\n\n```txt\ncode\n```\n\n| Name | Value |\n| --- | --- |\n| A | 1 |\n\nInline code `super-long-inline-code-token-for-wrap-checking`.\n'
    );
}

async function waitForPreviewStructures(page) {
    await page.waitForFunction(
        () =>
            !!document.querySelector('#editor-preview h2') &&
            document.querySelectorAll('#editor-preview li').length >= 1 &&
            !!document.querySelector('#editor-preview blockquote') &&
            !!document.querySelector('#editor-preview pre') &&
            !!document.querySelector('#editor-preview table') &&
            !!document.querySelector('#editor-preview a')
    );
}

async function expectEditorFocus(page) {
    await page.waitForFunction(() => document.activeElement?.id === 'editor-source');
}

async function moveCursorToEnd(page) {
    await page.evaluate(() => {
        const field = document.getElementById('editor-source');
        if (!field) return;
        field.focus();
        const length = field.value.length;
        field.setSelectionRange(length, length);
    });
    await expectEditorFocus(page);
}

async function assertAccentLink(page, selector) {
    const style = await page.locator(selector).first().evaluate((node) => {
        const computed = getComputedStyle(node);
        return { color: computed.color, decoration: computed.textDecorationLine };
    });
    assert.notEqual(style.color, 'rgb(242, 242, 239)');
    assert.ok(style.decoration.includes('underline'));
}

async function verifyGuestRender(browser, path) {
    const guest = await newContext(browser, { width: 1440, height: 1100 });
    const guestPage = await guest.newPage();
    await guestPage.goto(`${appUrl}${path}`, { waitUntil: 'networkidle' });
    await guestPage.waitForFunction(
        () =>
            !!document.querySelector('.prose h2') &&
            document.querySelectorAll('.prose li').length >= 1 &&
            !!document.querySelector('.prose blockquote') &&
            !!document.querySelector('.prose pre') &&
            !!document.querySelector('.prose table') &&
            !!document.querySelector('.prose a')
    );
    await assertNoHorizontalOverflow(guestPage);
    await assertAccentLink(guestPage, '.prose a');
    await assertVisibleText(guestPage, 'Alpha');
    await assertInvisibleText(guestPage, '* Alpha');
    await guest.close();
}

async function assertEditorChrome(page) {
    const legacyCount = await page
        .locator('.toastui-editor-defaultUI-toolbar,.toastui-editor-toolbar,#local-vim-mode,[data-vim-mode-state]')
        .count();
    assert.equal(legacyCount, 0);
    await assertInvisibleText(page, 'Vim normal');
    await assertInvisibleText(page, 'Vim off');
}
