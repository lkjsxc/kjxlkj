import assert from 'node:assert/strict';
import {
    assertCreateActionBelowHome,
    assertSingleHistoryCard,
    assertVisibleText,
    expectFlatShell,
} from './shell-assertions.mjs';

export async function expectAdminNote(page) {
    await expectFlatShell(page);
    await assertVisibleText(page, 'Open GitHub');
    await page.locator('#editor-body').waitFor({ state: 'visible' });
    assert.equal(await page.locator('#public-toggle').isChecked(), true);
    assert.equal(await page.locator('#favorite-toggle').isChecked(), true);
    assert.equal(await page.locator('#preview-toggle').getAttribute('aria-expanded'), 'false');
    assert.equal(await page.locator('.resource-head .status-pill').count(), 0);
    assert.equal(await page.locator('.resource-head h1').count(), 0);
    await assertVisibleText(page, 'Delete note');
    await assertVisibleText(page, 'URL alias');
    await assertVisibleText(page, 'Canonical URL');
    await assertVisibleText(page, 'Upload media');
    await assertVisibleText(page, 'Views total');
    await assertVisibleText(page, 'Views 30d');
    await assertSingleHistoryCard(page);
    await assertLiveNoteMetadata(page, page.locator('.current-resource-card'));
    await assertRailOrder(page, ['History', 'Open GitHub', 'Delete note']);
    assert.equal(await page.getByText('Markdown body', { exact: true }).count(), 0);
    assert.equal(await page.locator('script[src*="toastui"],link[href*="toastui"]').count(), 0);
    await assertCreateActionBelowHome(page);
}

export async function expectGuestNote(page, previousTitle, nextTitle) {
    await expectFlatShell(page);
    await assertVisibleText(page, 'Open GitHub');
    await assertSingleHistoryCard(page);
    await assertLiveNoteMetadata(page, page.locator('.current-resource-card'));
    await assertRailOrder(page, ['History', 'Open GitHub', 'Admin sign in']);
    assert.equal(await page.getByText('Views total', { exact: true }).count(), 0);
    await assertVisibleText(page, 'Prev');
    await assertVisibleText(page, previousTitle ?? 'No older accessible resource.');
    await assertVisibleText(page, 'Next');
    await assertVisibleText(page, nextTitle ?? 'No newer accessible resource.');
}

async function assertLiveNoteMetadata(page, card) {
    await card.waitFor({ state: 'visible' });
    await card.getByText('Created', { exact: true }).waitFor({ state: 'visible' });
    await card.getByText('Updated', { exact: true }).waitFor({ state: 'visible' });
}

async function assertRailOrder(page, labels) {
    const tops = [];
    for (const label of labels) {
        const control = page.getByText(label, { exact: true }).first();
        await control.waitFor({ state: 'visible' });
        tops.push(await control.evaluate((node) => node.getBoundingClientRect().top));
    }
    for (let index = 1; index < tops.length; index += 1) {
        assert.ok(tops[index] > tops[index - 1], 'rail controls should stay in the documented order');
    }
}
