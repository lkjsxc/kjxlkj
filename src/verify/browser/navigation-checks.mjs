import assert from 'node:assert/strict';
import { appUrl } from './support.mjs';

export async function verifyPartialResourceNavigation(page, note, previous) {
    const navigationCount = await page.evaluate(() => performance.getEntriesByType('navigation').length);
    const railScroll = await page.locator('.shell-rail').evaluate((node) => {
        const scrollable = node.scrollHeight > node.clientHeight + 10;
        if (scrollable) node.scrollTop = 180;
        return { scrollable, scrollTop: node.scrollTop };
    });
    await page.locator('#editor-body').evaluate((field) => {
        field.value += '\n\nPartial navigation save check.';
        field.focus();
        field.setSelectionRange(field.value.length, field.value.length);
        field.dispatchEvent(new Event('input', { bubbles: true }));
    });
    await Promise.all([
        page.waitForURL((url) => new URL(url).pathname === `/${previous.ref}`),
        page.getByText(previous.title, { exact: true }).first().click(),
    ]);
    assert.equal(
        await page.evaluate(() => performance.getEntriesByType('navigation').length),
        navigationCount,
        'resource transition should stay inside the current document'
    );
    if (railScroll.scrollable) {
        assert.ok(
            await page.locator('.shell-rail').evaluate((node) => node.scrollTop >= railScroll.scrollTop - 20),
            'rail scroll position should survive the transition'
        );
    }
    await Promise.all([
        page.waitForURL((url) => new URL(url).pathname === `/${note.ref}`),
        page.getByText(note.title, { exact: true }).first().click(),
    ]);
    const body = await page.locator('#editor-body').inputValue();
    assert.ok(body.includes('Partial navigation save check.'), 'dirty note should flush before navigation');
    await Promise.all([
        page.waitForURL((url) => new URL(url).pathname === `/${previous.ref}`),
        page.goBack(),
    ]);
    assert.equal(
        await page.evaluate(() => performance.getEntriesByType('navigation').length),
        navigationCount,
        'browser back should stay inside the current document'
    );
    await Promise.all([
        page.waitForURL((url) => new URL(url).pathname === `/${note.ref}`),
        page.goForward(),
    ]);
    assert.equal(
        await page.evaluate(() => performance.getEntriesByType('navigation').length),
        navigationCount,
        'browser forward should stay inside the current document'
    );
}

export async function verifyRememberedRailNavigation(page, note) {
    await page.goto(`${appUrl}/search?q=orbit`, { waitUntil: 'networkidle' });
    await page.evaluate(() => window.scrollTo(0, 480));
    await page.waitForTimeout(180);
    await Promise.all([
        page.waitForURL((url) => new URL(url).pathname === `/${note.ref}`),
        resourceCard(page, note.title).click(),
    ]);
    const rememberedSearch = await page.evaluate(() =>
        JSON.parse(window.sessionStorage.getItem('kjxlkj:shell-memory:v1') || '{}').search
    );
    await Promise.all([
        page.waitForURL((url) => `${new URL(url).pathname}${new URL(url).search}` === '/search?q=orbit'),
        page.getByRole('link', { name: 'Search', exact: true }).click(),
    ]);
    await page.waitForFunction((target) => window.scrollY >= target - 20, rememberedSearch?.scrollY ?? 0);

    await page.goto(`${appUrl}/`, { waitUntil: 'networkidle' });
    await page.getByRole('button', { name: '1d', exact: true }).click();
    await page.waitForFunction(() =>
        document.querySelector('[data-popular-window][aria-pressed="true"]')?.getAttribute('data-popular-window') === '1d'
    );
    await page.evaluate(() => window.scrollTo(0, 520));
    await page.waitForTimeout(180);
    await Promise.all([
        page.waitForURL((url) => new URL(url).pathname === `/${note.ref}`),
        resourceCard(page, note.title).click(),
    ]);
    const rememberedHome = await page.evaluate(() =>
        JSON.parse(window.sessionStorage.getItem('kjxlkj:shell-memory:v1') || '{}').home
    );
    await Promise.all([
        page.waitForURL((url) => new URL(url).pathname === '/'),
        page.getByRole('link', { name: 'Home', exact: true }).click(),
    ]);
    await page.waitForFunction(({ popularWindow, scrollY }) =>
        window.scrollY >= scrollY - 20 &&
        document.querySelector('[data-popular-window][aria-pressed="true"]')?.getAttribute('data-popular-window') === popularWindow
    , rememberedHome ?? { popularWindow: '30d', scrollY: 0 });
    await Promise.all([
        page.waitForURL((url) => new URL(url).pathname === `/${note.ref}`),
        resourceCard(page, note.title).click(),
    ]);
}

function resourceCard(page, title) {
    return page.locator(`.resource-row[data-card-title="${title}"]`).first();
}
