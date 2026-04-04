import assert from 'node:assert/strict';
import { alpha, contrast, isDark, isLight } from './style-utils.mjs';

export async function expectClosedDrawer(page) {
    const toggle = page.locator('[data-menu-toggle]');
    await toggle.waitFor({ state: 'visible' });
    assert.equal(await toggle.getAttribute('aria-expanded'), 'false');
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
    await assertNoHorizontalOverflow(page);
}

export async function assertNoHorizontalOverflow(page) {
    const overflow = await page.evaluate(() => document.documentElement.scrollWidth - document.documentElement.clientWidth);
    assert.ok(overflow <= 1, `page should not overflow horizontally (saw ${overflow}px)`);
}

export async function assertVisibleText(page, text) {
    await page.getByText(text, { exact: false }).first().waitFor({ state: 'visible' });
}

export async function assertInvisibleText(page, text) {
    const locator = page.getByText(text, { exact: false });
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
    for (const name of controlNames) await assertReadable(await namedControl(page, name));
}

export async function assertWideGrid(page) {
    const columns = await page.locator('.note-grid .note-row').evaluateAll((nodes) =>
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
    const row = page.locator('.note-row', { has: page.getByText(title, { exact: true }) }).first();
    const heights = await row.locator('.card-meta small').evaluateAll((nodes) =>
        nodes.map((node) => node.getBoundingClientRect().height)
    );
    assert.ok(heights.every((height) => height <= 24), 'timestamps should stay compact');
}

export async function assertSingleHistoryCard(page) {
    await assertVisibleText(page, 'All history');
    assert.equal(await page.getByText('All history', { exact: true }).count(), 1);
}

export async function assertCreateActionBelowHome(page) {
    const createControl = page.getByRole('button', { name: 'New note', exact: true }).first();
    if (!(await createControl.count()) || !(await createControl.isVisible())) return;
    const createTop = await createControl.evaluate((node) => node.getBoundingClientRect().top);
    const homeTop = await page.getByRole('link', { name: 'Home', exact: true }).first().evaluate((node) => node.getBoundingClientRect().top);
    assert.ok(createTop > homeTop, 'New note should sit below primary navigation');
}

async function assertDarkSurface(page) {
    const shell = page.locator('.surface, .index-card').first();
    const style = await shell.evaluate((node) => {
        const computed = getComputedStyle(node);
        return { background: computed.backgroundColor, backgroundImage: computed.backgroundImage, boxShadow: computed.boxShadow };
    });
    assert.ok(isDark(style.background));
    assert.equal(style.backgroundImage, 'none');
    assert.equal(style.boxShadow, 'none');
}

async function namedControl(page, name) {
    const button = page.getByRole('button', { name, exact: true });
    if (await button.count()) return button.first();
    return page.getByRole('link', { name, exact: true }).first();
}

async function assertReadable(locator) {
    await locator.waitFor({ state: 'visible' });
    const style = await locator.evaluate((node) => {
        const computed = getComputedStyle(node);
        return { color: computed.color, background: computed.backgroundColor, backgroundImage: computed.backgroundImage };
    });
    assert.ok(style.backgroundImage !== 'none' || contrast(style.color, style.background) >= 4.2 || (alpha(style.background) < 0.2 && isLight(style.color)));
}

async function assertBrandSpacing(page) {
    const gap = await page.evaluate(() => {
        const head = document.querySelector('.rail-head');
        const nav = document.querySelector('.rail-section .rail-link');
        if (!head || !nav) return 0;
        return nav.getBoundingClientRect().top - head.getBoundingClientRect().bottom;
    });
    assert.ok(gap >= 10, `brand and primary nav should have visual separation (saw ${gap}px)`);
}

async function assertBrandIcon(page) {
    assert.equal(await page.locator('link[rel="icon"][href="/favicon.ico"]').count(), 1);
    const marks = page.locator('.brand-mark');
    const visibleSources = await marks.evaluateAll((nodes) =>
        nodes
            .filter((node) => {
                const style = getComputedStyle(node);
                const rect = node.getBoundingClientRect();
                return style.display !== 'none' && style.visibility !== 'hidden' && rect.width > 0 && rect.height > 0;
            })
            .map((node) => node.getAttribute('src'))
    );
    assert.ok(visibleSources.length >= 1, 'at least one visible brand icon should render');
    assert.ok(visibleSources.every((src) => src === '/assets/icon.svg'));
}

async function assertRestrainedMainColumn(page) {
    const metrics = await page.evaluate(() => {
        const column = document.querySelector('.page-column');
        const head = document.querySelector('.page-head');
        const next = head?.nextElementSibling;
        return {
            viewportWidth: window.innerWidth,
            columnWidth: column?.getBoundingClientRect().width ?? 0,
            verticalGap: next ? next.getBoundingClientRect().top - head.getBoundingClientRect().bottom : 0,
        };
    });
    if (metrics.viewportWidth > 1200) {
        assert.ok(metrics.columnWidth <= 1062, `main column should stay restrained (saw ${metrics.columnWidth}px)`);
    }
    if (metrics.verticalGap) {
        assert.ok(metrics.verticalGap <= 36, `page-head gap should stay compact (saw ${metrics.verticalGap}px)`);
    }
}

async function assertTightCorners(page) {
    const samples = await page.evaluate(() =>
        ['.btn', '.surface', '.index-card', '.rail-link', 'input:not([type="checkbox"]):not([type="radio"])', 'select', 'textarea']
            .map((selector) => {
                const node = [...document.querySelectorAll(selector)].find((item) => {
                    const style = getComputedStyle(item);
                    const rect = item.getBoundingClientRect();
                    return style.display !== 'none' && style.visibility !== 'hidden' && rect.width > 0 && rect.height > 0;
                });
                if (!node) return null;
                const style = getComputedStyle(node);
                return { selector, radii: [style.borderTopLeftRadius, style.borderTopRightRadius, style.borderBottomRightRadius, style.borderBottomLeftRadius] };
            })
            .filter(Boolean)
    );
    for (const sample of samples) {
        const largest = Math.max(...sample.radii.map((value) => Number.parseFloat(value) || 0));
        assert.ok(largest <= 4.1, `${sample.selector} should keep tight corners (saw ${largest}px)`);
    }
}
