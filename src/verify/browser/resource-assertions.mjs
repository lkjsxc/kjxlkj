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
    await assertVisibleText(page, 'Views 1d');
    await assertVisibleText(page, 'Views 30d');
    await assertSingleHistoryCard(page);
    await assertLiveNoteMetadata(page, page.locator('.note-live-strip'));
    await assertNoteStrip(page);
    await assertRailOrder(page, ['Open GitHub', 'Delete note']);
    assert.equal(await page.locator('.current-resource-card').count(), 0);
    assert.equal(await page.locator('[data-live-alias]').count(), 0);
    assert.equal(await page.getByText('Markdown body', { exact: true }).count(), 0);
    assert.equal(await page.locator('script[src*="toastui"],link[href*="toastui"]').count(), 0);
    await assertCreateActionBelowHome(page);
}

export async function expectGuestNote(page, previousTitle, nextTitle) {
    await expectFlatShell(page);
    await assertVisibleText(page, 'Open GitHub');
    await assertSingleHistoryCard(page);
    await assertLiveNoteMetadata(page, page.locator('.note-live-strip'));
    await assertNoteStrip(page);
    await assertRailOrder(page, ['Open GitHub', 'Admin sign in']);
    assert.equal(await page.getByText('Views total', { exact: true }).count(), 0);
    await assertVisibleText(page, 'Prev');
    await assertVisibleText(page, previousTitle ?? 'No older accessible resource.');
    await assertVisibleText(page, 'Next');
    await assertVisibleText(page, nextTitle ?? 'No newer accessible resource.');
    assert.equal(await page.locator('.current-resource-card').count(), 0);
    assert.equal(await page.locator('[data-live-alias]').count(), 0);
}

async function assertLiveNoteMetadata(page, card) {
    await card.waitFor({ state: 'visible' });
    await card.getByText('Created', { exact: true }).waitFor({ state: 'visible' });
    await card.getByText('Updated', { exact: true }).waitFor({ state: 'visible' });
}

async function assertNoteStrip(page) {
    const metrics = await page.locator('.note-nav-strip .note-nav-card').evaluateAll((nodes) =>
        nodes.map((node) => {
            const rect = node.getBoundingClientRect();
            return { top: Math.round(rect.top), left: Math.round(rect.left) };
        })
    );
    assert.equal(metrics.length, 3, 'expected Prev, History, and Next note cards');
    assert.ok(metrics.every((item) => Math.abs(item.top - metrics[0].top) <= 4));
    assert.ok(metrics[1].left > metrics[0].left && metrics[2].left > metrics[1].left);
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
