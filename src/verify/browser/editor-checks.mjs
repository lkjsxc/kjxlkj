import assert from 'node:assert/strict';
import { assertInvisibleText, assertVisibleText } from './assertions.mjs';
import { appUrl, newContext } from './support.mjs';

export async function verifyUiCreatedDraft(page) {
    await Promise.all([
        page.waitForURL((url) => new URL(url).pathname !== '/admin'),
        page.getByRole('button', { name: 'New note', exact: true }).first().click(),
    ]);
    await page.locator('[data-live-title]').first().waitFor({ state: 'visible' });
    const title = (await page.locator('[data-live-title]').first().textContent()).trim();
    assert.match(title, /^\d{4}-\d{2}-\d{2} \d{2}:\d{2}$/);
    assert.equal(await page.locator('#public-toggle').isChecked(), false, 'new notes should default to private drafts');
    await page.getByRole('button', { name: 'Show preview', exact: true }).waitFor({ state: 'visible' });
    assert.equal(
        await page.locator('#preview-toggle').getAttribute('aria-expanded'),
        'false',
        'preview should start closed'
    );
    await expectEditorFocus(page);
}

export async function verifyEditorFormatting(browser, page, note, vimEnabled = false) {
    const saveRequests = [];
    page.on('requestfinished', (request) => {
        if (request.method() === 'PUT' && new URL(request.url()).pathname === `/records/${note.id}`) {
            saveRequests.push(Date.now());
        }
    });
    await page.locator('.toastui-editor-md-container .ProseMirror').first().waitFor({ state: 'visible' });
    await expectEditorFocus(page);
    await verifyVimMode(page, vimEnabled);
    await page.waitForTimeout(1600);
    assert.equal(saveRequests.length, 0, 'idle note should not save before edits');
    await appendMarkdown(page, vimEnabled);
    await openPreview(page);
    await waitForPreviewStructures(page);
    await assertEditorLayout(page, false);
    await page.waitForTimeout(1800);
    assert.ok(saveRequests.length >= 1, 'editing should trigger autosave');
    const settledCount = saveRequests.length;
    await page.waitForTimeout(1600);
    assert.equal(saveRequests.length, settledCount, 'autosave should settle once edits are saved');
    await page.reload({ waitUntil: 'networkidle' });
    assert.equal(
        await page.locator('#preview-toggle').getAttribute('aria-expanded'),
        'false',
        'preview should reset closed after reload'
    );
    if (vimEnabled) await page.getByText('Vim normal', { exact: true }).waitFor({ state: 'visible' });
    await openPreview(page);
    await waitForPreviewStructures(page);
    const guest = await newContext(browser, { width: 1440, height: 1100 });
    const guestPage = await guest.newPage();
    await guestPage.goto(`${appUrl}/${note.ref}`, { waitUntil: 'networkidle' });
    await guestPage.waitForFunction(
        () =>
            !!document.querySelector('.prose h2') &&
            document.querySelectorAll('.prose li').length >= 1 &&
            !!document.querySelector('.prose blockquote') &&
            !!document.querySelector('.prose pre') &&
            !!document.querySelector('.prose table')
    );
    await assertVisibleText(guestPage, 'Alpha');
    await assertInvisibleText(guestPage, '* Alpha');
    await guest.close();
}

export async function openPreview(page) {
    const toggle = page.locator('#preview-toggle');
    await toggle.waitFor({ state: 'visible' });
    await toggle.click();
    await page.waitForFunction(
        () =>
            document.querySelector('#preview-toggle')?.getAttribute('aria-expanded') === 'true' &&
            !!document.querySelector('.toastui-editor-md-preview')
    );
}

export async function assertEditorLayout(page, compact) {
    await page.waitForFunction((isCompact) => {
        const toolbar = document.querySelector('.toastui-editor-defaultUI-toolbar,.toastui-editor-toolbar');
        const preview = document.querySelector('.toastui-editor-md-preview');
        const editor = document.querySelector('.toastui-editor-md-container > .toastui-editor');
        const scroll = document.querySelector('.toastui-editor-md-container .ProseMirror');
        if (!toolbar || !preview || !editor || !scroll) return false;
        const previewStyle = getComputedStyle(preview);
        const toolbarOk = toolbar.scrollWidth - toolbar.clientWidth <= 1;
        const scrollOk = scroll.scrollHeight - scroll.clientHeight <= 1;
        if (isCompact) return toolbarOk && previewStyle.position === 'fixed' && scrollOk;
        const sideBySide = previewStyle.position !== 'fixed' &&
            preview.getBoundingClientRect().left >= editor.getBoundingClientRect().right - 4;
        return toolbarOk && sideBySide && scrollOk;
    }, compact);
}

async function appendMarkdown(page, vimEnabled) {
    if (vimEnabled) await ensureInsertMode(page);
    await moveCursorToEnd(page);
    for (const line of [
        '',
        '## Live Heading',
        '',
        '- Alpha',
        '',
        '> Quoted line',
        '',
        '```txt',
        'code',
        '```',
        '',
        '| Name | Value |',
        '| --- | --- |',
        '| A | 1 |',
    ]) {
        if (line) await page.keyboard.type(line);
        await page.keyboard.press('Enter');
    }
    await page.locator('.toastui-editor-toolbar-icons.table').first().waitFor({ state: 'visible' });
}

async function verifyVimMode(page, enabled) {
    const state = page.locator('[data-vim-mode-state]').first();
    await state.waitFor({ state: 'visible' });
    if (!enabled) {
        assert.equal((await state.textContent()).trim(), 'Vim off');
        return;
    }
    assert.equal((await state.textContent()).trim(), 'Vim normal');
    await page.keyboard.press('i');
    await page.getByText('Vim insert', { exact: true }).waitFor({ state: 'visible' });
    await page.keyboard.press('Escape');
    await page.getByText('Vim normal', { exact: true }).waitFor({ state: 'visible' });
}

async function ensureInsertMode(page) {
    const state = page.locator('[data-vim-mode-state]').first();
    await state.waitFor({ state: 'visible' });
    if ((await state.textContent()).trim() === 'Vim normal') {
        await page.keyboard.press('i');
        await page.getByText('Vim insert', { exact: true }).waitFor({ state: 'visible' });
    }
}

async function waitForPreviewStructures(page) {
    await page.waitForFunction(
        () =>
            !!document.querySelector('.toastui-editor-md-preview .toastui-editor-contents h2') &&
            document.querySelectorAll('.toastui-editor-md-preview .toastui-editor-contents li').length >= 1 &&
            !!document.querySelector('.toastui-editor-md-preview .toastui-editor-contents blockquote') &&
            !!document.querySelector('.toastui-editor-md-preview .toastui-editor-contents pre') &&
            !!document.querySelector('.toastui-editor-md-preview .toastui-editor-contents table')
    );
}

async function expectEditorFocus(page) {
    await page.waitForFunction(() => {
        const surface = document.querySelector('.toastui-editor-md-container .ProseMirror');
        return !!surface && !!document.activeElement?.closest('.ProseMirror');
    });
}

async function moveCursorToEnd(page) {
    await page.evaluate(() => {
        window.editorInstance.focus();
        window.editorInstance.moveCursorToEnd();
    });
    await expectEditorFocus(page);
}
