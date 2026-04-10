import assert from 'node:assert/strict';
import {
    assertBrandIcon,
    assertBrandSpacing,
    assertDarkSurface,
    assertReadableControl,
    assertRestrainedMainColumn,
    assertTightCorners,
} from './shell-style-assertions.mjs';

export async function expectClosedDrawer(page) {
    const toggle = page.locator('[data-menu-toggle]');
    await toggle.waitFor({ state: 'visible' });
    assert.equal(await toggle.getAttribute('aria-expanded'), 'false');
    assert.equal(await page.locator('[data-menu-panel]').getAttribute('inert'), '');
    await page.waitForFunction(() => {
        const node = document.querySelector('[data-menu-panel]');
        return !!node && node.getBoundingClientRect().right < 20;
    });
}

export async function openDrawer(page) {
    const toggle = page.locator('[data-menu-toggle]');
    await toggle.click();
    await page.waitForFunction(() => document.body.classList.contains('rail-open'));
    await page.waitForFunction(() => {
        const node = document.querySelector('[data-menu-panel]');
        return !!node && node.getBoundingClientRect().right > 200;
    });
    assert.equal(await toggle.getAttribute('aria-expanded'), 'true');
    assert.equal(await page.locator('[data-menu-panel]').getAttribute('inert'), null);
    await assertNoHorizontalOverflow(page);
}

export async function assertNoHorizontalOverflow(page) {
    const overflow = await page.evaluate(() => document.documentElement.scrollWidth - document.documentElement.clientWidth);
    assert.ok(overflow <= 1, `page should not overflow horizontally (saw ${overflow}px)`);
}

export async function assertVisibleText(page, text) {
    const visibleCount = await page.getByText(text, { exact: false }).evaluateAll((nodes) =>
        nodes.filter((node) => {
            const style = window.getComputedStyle(node);
            const rect = node.getBoundingClientRect();
            return style.display !== 'none' && style.visibility !== 'hidden' && rect.width > 0 && rect.height > 0;
        }).length
    );
    assert.ok(visibleCount > 0, `"${text}" should be visible`);
}
export async function assertInvisibleText(page, text) {
    const locator = page.getByText(text, { exact: true });
    const visibleCount = await locator.evaluateAll((nodes) =>
        nodes.filter((node) => {
            const style = window.getComputedStyle(node);
            const rect = node.getBoundingClientRect();
            return style.display !== 'none' && style.visibility !== 'hidden' && rect.width > 0 && rect.height > 0;
        }).length
    );
    assert.equal(visibleCount, 0, `"${text}" should stay hidden`);
}

export async function expectFlatShell(page, controlNames = []) {
    await assertDarkSurface(page);
    await assertInvisibleText(page, 'Rich mode');
    await assertInvisibleText(page, 'Text mode');
    await assertInvisibleText(page, 'Saving');
    await assertInvisibleText(page, 'Saved');
    await assertNoHorizontalOverflow(page);
    await assertBrandIcon(page);
    await assertRestrainedMainColumn(page);
    await assertTightCorners(page);
    assert.equal(await page.locator('.shell-rail input[type="search"]').count(), 0);
    assert.equal(await page.locator('.shell-rail h2').count(), 0);
    if ((await page.evaluate(() => window.innerWidth)) > 900) await assertBrandSpacing(page);
    for (const name of controlNames) await assertReadableControl(page, name);
}

export async function assertWideGrid(page) {
    const columns = await page.locator('.resource-grid .resource-row').evaluateAll((nodes) =>
        new Set(nodes.map((node) => Math.round(node.getBoundingClientRect().left))).size
    );
    assert.ok(columns > 1, 'wide home sections should use multiple columns');
}

export async function assertGridHeights(page, selector) {
    const heights = await page.locator(selector).evaluateAll((nodes) =>
        nodes.map((node) => Math.round(node.getBoundingClientRect().height))
    );
    assert.ok(Math.max(...heights) - Math.min(...heights) <= 4, 'grid cards should keep consistent heights');
}

export async function assertNoHeaderButtons(page) {
    assert.equal(await page.locator('.page-head .btn').count(), 0);
}

export async function assertStableMetadata(page, title) {
    const row = page.locator('.resource-row', { has: page.getByText(title, { exact: true }) }).first();
    const heights = await row.locator('.card-meta small').evaluateAll((nodes) =>
        nodes.map((node) => node.getBoundingClientRect().height)
    );
    assert.ok(heights.every((height) => height <= 24), 'timestamps should stay compact');
}

export async function assertSingleHistoryCard(page) {
    await assertVisibleText(page, 'History');
    assert.equal(await page.getByText('History', { exact: true }).count(), 1);
}

export async function assertCreateActionBelowHome(page) {
    const createControl = page.getByRole('button', { name: 'New note', exact: true }).first();
    if (!(await createControl.count()) || !(await createControl.isVisible())) return;
    const createTop = await createControl.evaluate((node) => node.getBoundingClientRect().top);
    const homeTop = await page.getByRole('link', { name: 'Home', exact: true }).first().evaluate((node) => node.getBoundingClientRect().top);
    assert.ok(createTop > homeTop, 'New note should sit below primary navigation');
}
