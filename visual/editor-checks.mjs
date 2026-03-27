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
    await expectEditorFocus(page);
}

export async function verifyEditorFormatting(browser, page, id) {
    const fence = '`'.repeat(3);
    const editor = page.locator('.toastui-editor-ww-container .ProseMirror').first();
    await editor.waitFor({ state: 'visible' });
    await expectEditorFocus(page);
    await appendBlocks(page, fence);
    await openTablePicker(page);
    await waitForEditorStructures(page);
    await page.waitForTimeout(1800);
    await page.reload({ waitUntil: 'networkidle' });
    await waitForEditorStructures(page);
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

async function appendBlocks(page, fence) {
    await moveCursorToEnd(page);
    await page.keyboard.press('Enter');
    await page.keyboard.type('## ');
    await page.keyboard.type('LiveHeading');
    await page.keyboard.press('Enter');
    await moveCursorToEnd(page);
    await page.keyboard.press('Enter');
    await page.keyboard.type('- ');
    await page.keyboard.type('Alpha');
    await page.keyboard.press('Enter');
    await moveCursorToEnd(page);
    await page.keyboard.press('Enter');
    await page.keyboard.press('Enter');
    await page.keyboard.type('> ');
    await page.keyboard.type('Quoted line');
    await page.keyboard.press('Enter');
    await page.keyboard.press('Enter');
    await moveCursorToEnd(page);
    await page.keyboard.press('Enter');
    await page.keyboard.type(fence);
    await page.keyboard.type('txt');
    await page.keyboard.press('Enter');
    await page.keyboard.type('code');
}

async function openTablePicker(page) {
    const tableButton = page.locator('.toastui-editor-toolbar-icons.table').first();
    await tableButton.waitFor({ state: 'visible' });
    await tableButton.click();
    await page.locator('.toastui-editor-popup-add-table').waitFor({ state: 'visible' });
}

async function waitForEditorStructures(page) {
    await page.waitForFunction(
        () =>
            !!document.querySelector('.toastui-editor-ww-container .toastui-editor-contents h2') &&
            document.querySelectorAll('.toastui-editor-ww-container .toastui-editor-contents li').length >= 1 &&
            !!document.querySelector('.toastui-editor-ww-container .toastui-editor-contents blockquote') &&
            !!document.querySelector('.toastui-editor-ww-container .toastui-editor-contents pre') &&
            Array.from(document.querySelectorAll('.toastui-editor-defaultUI-toolbar,.toastui-editor-toolbar')).every(
                (node) => node.scrollWidth - node.clientWidth <= 1 && node.scrollHeight - node.clientHeight <= 1
            ) &&
            Array.from(document.querySelectorAll('.toastui-editor-ww-container .toastui-editor-contents,.toastui-editor-ww-container .ProseMirror')).every(
                (node) => node.scrollHeight - node.clientHeight <= 1
            )
    );
}

async function expectEditorFocus(page) {
    await page.waitForFunction(() => {
        const surface = document.querySelector('.toastui-editor-ww-container .ProseMirror');
        const selection = window.getSelection();
        if (!surface) return false;
        return surface === document.activeElement ||
            surface.contains(document.activeElement) ||
            !!(selection && selection.anchorNode && surface.contains(selection.anchorNode));
    });
}

async function moveCursorToEnd(page) {
    await page.evaluate(() => {
        window.editorInstance.focus();
        window.editorInstance.moveCursorToEnd();
    });
    await expectEditorFocus(page);
}
