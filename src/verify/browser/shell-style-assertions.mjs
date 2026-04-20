import assert from 'node:assert/strict';
import { alpha, contrast, isDark, isLight } from './style-utils.mjs';

export async function assertDarkSurface(page) {
    const shell = page.locator('.surface, .index-card, .settings-row').first();
    const style = await shell.evaluate((node) => {
        const computed = getComputedStyle(node);
        return { background: computed.backgroundColor, backgroundImage: computed.backgroundImage, boxShadow: computed.boxShadow };
    });
    assert.ok(isDark(style.background));
    assert.equal(style.backgroundImage, 'none');
    assert.equal(style.boxShadow, 'none');
}

export async function assertReadableControl(page, name) {
    const locator = await namedControl(page, name);
    await locator.waitFor({ state: 'visible' });
    const style = await locator.evaluate((node) => {
        const computed = getComputedStyle(node);
        return { color: computed.color, background: computed.backgroundColor, backgroundImage: computed.backgroundImage };
    });
    assert.ok(style.backgroundImage !== 'none' || contrast(style.color, style.background) >= 4.2 || (alpha(style.background) < 0.2 && isLight(style.color)));
}

export async function assertBrandSpacing(page) {
    const gap = await page.evaluate(() => {
        const head = document.querySelector('.rail-head');
        const nav = document.querySelector('.rail-section .rail-link');
        if (!head || !nav) return 0;
        return nav.getBoundingClientRect().top - head.getBoundingClientRect().bottom;
    });
    assert.ok(gap >= 10, `brand and primary nav should have visual separation (saw ${gap}px)`);
}

export async function assertBrandIcon(page) {
    assert.equal(await page.locator('link[rel="icon"][href="/assets/site-icon"]').count(), 1);
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
    assert.ok(visibleSources.every((src) => src === '/assets/site-icon'));
}

export async function assertRestrainedMainColumn(page) {
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

export async function assertTightCorners(page) {
    const samples = await page.evaluate(() =>
        ['.btn', '.surface', '.index-card', '.settings-row', '.rail-link', 'input:not([type="checkbox"]):not([type="radio"])', 'select', 'textarea']
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

async function namedControl(page, name) {
    const button = page.getByRole('button', { name, exact: true });
    if (await button.count()) return button.first();
    return page.getByRole('link', { name, exact: true }).first();
}
