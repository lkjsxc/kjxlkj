import assert from 'node:assert/strict';
import {
    assertCreateActionBelowHome,
    assertVisibleText,
    expectFlatShell,
} from './shell-assertions.mjs';

export async function expectAdminNote(page, previousTitle, nextTitle) {
    await expectLiveResourceShell(page, {
        admin: true,
        deleteLabel: 'Delete note',
        previousTitle,
        nextTitle,
    });
    await page.locator('#editor-body').waitFor({ state: 'visible' });
    assert.equal(await page.locator('#public-toggle').isChecked(), true);
    assert.equal(await page.locator('#favorite-toggle').isChecked(), true);
    assert.equal(await page.locator('#preview-toggle').getAttribute('aria-expanded'), 'false');
    await assertHeaderPills(page, ['Note', 'Favorite', 'Public']);
    assert.equal(await page.locator('.resource-head h1').count(), 0);
    await assertVisibleText(page, 'URL alias');
    await assertVisibleText(page, 'Canonical URL');
    await assertVisibleText(page, 'Upload media');
    await assertVisibleText(page, 'Views total');
    await assertVisibleText(page, 'Views 1d');
    await assertVisibleText(page, 'Views 30d');
    assert.equal(await page.getByText('Markdown body', { exact: true }).count(), 0);
    assert.equal(await page.locator('script[src*="toastui"],link[href*="toastui"]').count(), 0);
    await assertCreateActionBelowHome(page);
}

export async function expectGuestNote(page, previousTitle, nextTitle, favorite = true) {
    await expectLiveResourceShell(page, {
        admin: false,
        deleteLabel: null,
        previousTitle,
        nextTitle,
    });
    await assertHeaderPills(page, favorite ? ['Note', 'Favorite', 'Public'] : ['Note', 'Public']);
    assert.equal(await page.getByText('Views total', { exact: true }).count(), 0);
}

export async function expectAdminMedia(page, previousTitle, nextTitle) {
    await expectLiveResourceShell(page, {
        admin: true,
        deleteLabel: 'Delete media',
        previousTitle,
        nextTitle,
    });
    await page.locator('#editor-body').waitFor({ state: 'visible' });
    assert.ok((await headerPills(page)).some((item) => item === 'Image' || item === 'File' || item === 'Video'));
    assert.ok((await headerPills(page)).includes('Public'));
    await assertVisibleText(page, 'File URL');
    await assertVisibleText(page, 'File metadata');
    await assertVisibleText(page, 'URL alias');
    await assertVisibleText(page, 'Canonical URL');
    assert.equal(await page.locator('#upload-media-trigger').count(), 0);
}

export async function expectGuestMedia(page, previousTitle, nextTitle) {
    await expectLiveResourceShell(page, {
        admin: false,
        deleteLabel: null,
        previousTitle,
        nextTitle,
    });
    assert.ok((await headerPills(page)).some((item) => item === 'Image' || item === 'File' || item === 'Video'));
    assert.ok((await headerPills(page)).includes('Public'));
}

async function expectLiveResourceShell(page, options) {
    await expectFlatShell(page);
    await assertVisibleText(page, 'Open GitHub');
    await assertLiveResourceMetadata(page);
    await assertResourceStrip(page, options.admin ? 3 : 2);
    await assertRailOrder(page, options.admin ? ['Open GitHub', options.deleteLabel] : ['Open GitHub', 'Admin sign in']);
    await assertVisibleText(page, 'Prev');
    assert.equal(await page.getByText('History', { exact: true }).count(), options.admin ? 1 : 0);
    await assertVisibleText(page, 'Next');
    if (options.previousTitle !== undefined) {
        await assertVisibleText(page, options.previousTitle ?? 'No older accessible resource.');
    }
    if (options.nextTitle !== undefined) {
        await assertVisibleText(page, options.nextTitle ?? 'No newer accessible resource.');
    }
    assert.equal(await page.locator('.current-resource-card').count(), 0);
    assert.equal(await page.locator('[data-live-alias]').count(), 0);
}

async function assertLiveResourceMetadata(page) {
    const card = page.locator('.resource-head .page-meta');
    await card.waitFor({ state: 'visible' });
    await page.locator('.resource-head .status-pill').first().waitFor({ state: 'visible' });
    await card.getByText('Created', { exact: true }).waitFor({ state: 'visible' });
    await card.getByText('Updated', { exact: true }).waitFor({ state: 'visible' });
    const alignment = await card.evaluate((node) => {
        const rect = node.getBoundingClientRect();
        const badges = node.querySelector('.page-badges')?.getBoundingClientRect();
        return { metaLeft: Math.round(rect.left), badgesLeft: Math.round(badges?.left ?? rect.left) };
    });
    assert.ok(Math.abs(alignment.metaLeft - alignment.badgesLeft) <= 4, 'resource metadata should stay left-aligned');
}

async function assertResourceStrip(page, expectedCount) {
    const metrics = await page.locator('.resource-nav-strip .resource-nav-card').evaluateAll((nodes) =>
        nodes.map((node) => {
            const rect = node.getBoundingClientRect();
            return {
                top: Math.round(rect.top),
                left: Math.round(rect.left),
                width: Math.round(rect.width),
                height: Math.round(rect.height),
            };
        })
    );
    const widths = await page.evaluate(() => {
        const strip = document.querySelector('.resource-nav-strip');
        const surface = document.querySelector('.resource-surface');
        return {
            strip: strip ? Math.round(strip.getBoundingClientRect().width) : 0,
            surface: surface ? Math.round(surface.getBoundingClientRect().width) : 0,
        };
    });
    assert.equal(metrics.length, expectedCount, 'unexpected resource navigation card count');
    assert.ok(metrics.every((item) => Math.abs(item.top - metrics[0].top) <= 4));
    assert.ok(metrics[1].left > metrics[0].left);
    if (expectedCount === 3) assert.ok(metrics[2].left > metrics[1].left);
    assert.ok(metrics.every((item) => Math.abs(item.width - metrics[0].width) <= 4));
    assert.ok(metrics.every((item) => Math.abs(item.height - metrics[0].height) <= 8));
    assert.ok(widths.strip <= widths.surface + 4, 'timeline strip should not exceed the content surface width');
    assert.ok(Math.abs(widths.strip - widths.surface) <= 4, 'timeline strip should align to the content surface width');
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

async function headerPills(page) {
    return page.locator('.resource-head .status-pill').evaluateAll((nodes) =>
        nodes.map((node) => node.textContent.trim())
    );
}

async function assertHeaderPills(page, expected) {
    const pills = await headerPills(page);
    for (const label of expected) assert.ok(pills.includes(label), `expected header pill ${label}`);
}
