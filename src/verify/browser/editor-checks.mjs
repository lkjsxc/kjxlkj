import assert from 'node:assert/strict';
import { assertInvisibleText, assertVisibleText } from './assertions.mjs';
import { assertNoHorizontalOverflow } from './shell-assertions.mjs';
import { appUrl, newContext } from './support.mjs';

export async function verifyUiCreatedDraft(page, expectedPrivate = false) {
    await Promise.all([
        page.waitForURL((url) => new URL(url).pathname !== '/admin'),
        page.getByRole('button', { name: 'New note', exact: true }).first().click(),
    ]);
    await page.locator('#editor-body').waitFor({ state: 'visible' });
    const title = (await page.locator('[data-live-title]').first().textContent()).trim();
    assert.match(title, /^\d{4}-\d{2}-\d{2} \d{2}:\d{2}$/);
    assert.equal(await page.locator('#public-toggle').isChecked(), !expectedPrivate, 'new note visibility should follow settings');
    assert.equal(await page.locator('#preview-toggle').getAttribute('aria-expanded'), 'false', 'preview should start closed');
    await expectEditorFocus(page);
    const savePromise = page.waitForResponse((response) => {
        return response.request().method() === 'PUT' && new URL(response.url()).pathname.startsWith('/resources/');
    });
    const aliasInput = page.locator('#alias-input');
    await aliasInput.click();
    await page.keyboard.type('launchpad-note_v2.release');
    assert.equal(await aliasInput.inputValue(), 'launchpad-note_v2.release');
    assert.equal((await savePromise).status(), 200);
    await page.waitForURL((url) => new URL(url).pathname === '/launchpad-note_v2.release');
    await assertVisibleText(page, '/launchpad-note_v2.release');
}

export async function verifyEditorFormatting(browser, page, note, media) {
    const previewImageSrc = `${media.image.fileHref}?variant=display`;
    const saveRequests = [];
    page.on('requestfinished', (request) => {
        if (request.method() === 'PUT' && new URL(request.url()).pathname === `/resources/${note.id}`) {
            saveRequests.push(Date.now());
        }
    });
    await page.locator('#editor-body').waitFor({ state: 'visible' });
    await expectEditorFocus(page);
    await page.waitForTimeout(1600);
    assert.equal(saveRequests.length, 0, 'idle note should not save before edits');
    await appendMarkdown(page);
    await openPreview(page);
    await waitForPreviewStructures(page);
    await waitForPreviewMedia(page, previewImageSrc, media.video.fileHref);
    await assertContainedVideo(page, '#editor-preview video');
    await assertEditorLayout(page, false);
    await assertAccentLink(page, '#editor-preview a');
    await page.waitForTimeout(1800);
    assert.ok(saveRequests.length >= 1, 'editing should trigger autosave');
    const settledCount = saveRequests.length;
    await page.waitForTimeout(1600);
    assert.equal(saveRequests.length, settledCount, 'autosave should settle once edits are saved');
    await page.reload({ waitUntil: 'networkidle' });
    assert.equal(await page.locator('#preview-toggle').getAttribute('aria-expanded'), 'false', 'preview should reset closed after reload');
    await openPreview(page);
    await waitForPreviewStructures(page);
    await waitForPreviewMedia(page, previewImageSrc, media.video.fileHref);
    const guest = await newContext(browser, { width: 1440, height: 1100 });
    const guestPage = await guest.newPage();
    await guestPage.goto(`${appUrl}/${note.ref}`, { waitUntil: 'networkidle' });
    await guestPage.waitForFunction(
        ({ imageSrc, videoSrc }) =>
            !!document.querySelector('.prose h2') &&
            document.querySelectorAll('.prose li').length >= 1 &&
            !!document.querySelector('.prose blockquote') &&
            !!document.querySelector('.prose pre') &&
            !!document.querySelector('.prose table') &&
            !!document.querySelector('.prose a') &&
            !!document.querySelector(`.prose img[src="${imageSrc}"]`) &&
            !!document.querySelector(`.prose video[src="${videoSrc}"]`),
        { imageSrc: previewImageSrc, videoSrc: media.video.fileHref }
    );
    await assertNoHorizontalOverflow(guestPage);
    await assertContainedVideo(guestPage, '.prose video');
    await assertAccentLink(guestPage, '.prose a');
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
            !document.querySelector('#editor-preview-panel')?.hidden
    );
}

export async function assertEditorLayout(page, compact) {
    await page.waitForFunction((isCompact) => {
        const editor = document.querySelector('#editor-body');
        const preview = document.querySelector('#editor-preview-panel');
        if (!editor || !preview) return false;
        const editorOk = editor.scrollWidth - editor.clientWidth <= 1;
        const previewStyle = getComputedStyle(preview);
        const pageColor = getComputedStyle(document.body).backgroundColor;
        const previewColor = previewStyle.backgroundColor;
        const previewText = previewStyle.color;
        const brightness = (color) => color.match(/\d+/g).slice(0, 3).map(Number).reduce((sum, value, index) => {
            return sum + value * [0.2126, 0.7152, 0.0722][index];
        }, 0);
        const darkPreview = brightness(previewColor) > brightness(pageColor) + 5 &&
            brightness(previewColor) < 90 &&
            brightness(previewText) > 180;
        if (isCompact) return editorOk && previewStyle.position === 'fixed' && darkPreview;
        const sideBySide = previewStyle.position !== 'fixed' &&
            preview.getBoundingClientRect().left >= editor.getBoundingClientRect().right - 4;
        return editorOk && sideBySide && darkPreview;
    }, compact);
}

async function appendMarkdown(page) {
    await moveCursorToEnd(page);
    for (const line of [
        '',
        '## Live Heading',
        '',
        '[Docs](https://example.com/very-long-link-for-wrap-testing)',
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
        '',
        'Inline code `super-long-inline-code-token-for-wrap-checking`.',
    ]) {
        if (line) await page.keyboard.type(line);
        await page.keyboard.press('Enter');
    }
}

async function waitForPreviewStructures(page) {
    await page.waitForFunction(
        () =>
            !!document.querySelector('#editor-preview h2') &&
            document.querySelectorAll('#editor-preview li').length >= 1 &&
            !!document.querySelector('#editor-preview blockquote') &&
            !!document.querySelector('#editor-preview pre') &&
            !!document.querySelector('#editor-preview table')
    );
}

async function waitForPreviewMedia(page, imageSrc, videoSrc) {
    await page.waitForFunction(
        ({ imageSrc, videoSrc }) =>
            !!document.querySelector(`#editor-preview img[src="${imageSrc}"]`) &&
            !!document.querySelector(`#editor-preview video[src="${videoSrc}"]`),
        { imageSrc, videoSrc }
    );
}

async function expectEditorFocus(page) {
    await page.waitForFunction(() => document.activeElement?.id === 'editor-body');
}

async function moveCursorToEnd(page) {
    await page.evaluate(() => {
        const field = document.querySelector('#editor-body');
        field.focus();
        field.setSelectionRange(field.value.length, field.value.length);
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

async function assertContainedVideo(page, selector) {
    const metrics = await page.locator(selector).first().evaluate((node) => {
        const video = node.getBoundingClientRect();
        const parent = node.parentElement.getBoundingClientRect();
        return { videoWidth: Math.round(video.width), parentWidth: Math.round(parent.width) };
    });
    assert.ok(metrics.videoWidth <= metrics.parentWidth, 'video should stay within its prose container');
}
