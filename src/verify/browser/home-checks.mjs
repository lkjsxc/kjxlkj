import assert from 'node:assert/strict';
import { assertVisibleText } from './assertions.mjs';
import { assertListRailOrder } from './rail-order-checks.mjs';

export async function popularTitles(page) {
    return page
        .locator('.section-block.note-section', {
            has: page.getByRole('heading', { name: 'Popular', exact: true }),
        })
        .locator('.note-row[data-card-title]')
        .evaluateAll((nodes) => nodes.map((node) => node.dataset.cardTitle.trim()));
}

export async function assertHomeBrowseLinks(page) {
    assert.equal(await browseHref(page, 'Popular'), '/search?sort=popular_desc&popular_window=30d');
    assert.equal(await browseHref(page, 'Recently updated'), '/search');
    assert.equal(await browseHref(page, 'Favorites'), '/search?scope=favorites');
}

export async function assertPopularWindowSwitch(page, path, surface) {
    const before = await navigationCount(page);
    await clickWindow(page, surface, '90d', '90d', 'Atlas Entry');
    assert.equal(await browseHref(page, 'Popular'), '/search?sort=popular_desc&popular_window=90d');
    assertStableUrl(page, path, before);
    await clickWindow(page, surface, 'All time', 'all', 'Atlas Entry');
    assert.equal(await browseHref(page, 'Popular'), '/search?sort=popular_desc&popular_window=all');
    if (surface === 'admin') await assertPopularMetricLabel(page, surface, 'Views');
    assertStableUrl(page, path, before);
    await clickWindow(page, surface, '30d', '30d', 'Beacon Log');
    assert.equal(await browseHref(page, 'Popular'), '/search?sort=popular_desc&popular_window=30d');
    assertStableUrl(page, path, before);
}

export async function assertAdminHomeConfiguration(page) {
    await assertVisibleText(page, 'Launchpad');
    await assertVisibleText(page, 'Welcome to Launchpad');
    await assertListRailOrder(page);
    assert.ok(await page.getByText('All time', { exact: true }).count() >= 1);
    assert.equal(await page.getByRole('heading', { name: 'Recently updated', exact: true }).count(), 0);
    const [favoritesTop, popularTop] = await Promise.all([
        page.getByRole('heading', { name: 'Favorites', exact: true }).evaluate((node) => node.getBoundingClientRect().top),
        page.getByRole('heading', { name: 'Popular', exact: true }).evaluate((node) => node.getBoundingClientRect().top),
    ]);
    assert.ok(favoritesTop < popularTop, 'favorites should move ahead of Popular');
    assert.equal(await page.getByRole('link', { name: /View more resources/i }).count(), 2);
}

async function browseHref(page, heading) {
    return page
        .locator('.section-block.note-section', {
            has: page.getByRole('heading', { name: heading, exact: true }),
        })
        .locator('a.note-row-action')
        .getAttribute('href');
}

async function clickWindow(page, surface, label, windowValue, firstTitle) {
    await page.getByRole('button', { name: label, exact: true }).click();
    await page.waitForFunction(
        ({ expectedSurface, expectedTitle, expectedWindow }) => {
            const section = document.querySelector(
                '[data-popular-section][data-popular-surface="' + expectedSurface + '"]'
            );
            if (!section || section.getAttribute('aria-busy') === 'true') return false;
            const title = section.querySelector('.note-row[data-card-title]');
            const active = section.querySelector('[data-popular-window][aria-pressed="true"]');
            return (
                !!title &&
                !!active &&
                title.dataset.cardTitle.trim() === expectedTitle &&
                active.getAttribute('data-popular-window') === expectedWindow &&
                !section.textContent.includes('UTC')
            );
        },
        { expectedSurface: surface, expectedTitle: firstTitle, expectedWindow: windowValue }
    );
}

async function assertPopularMetricLabel(page, surface, label) {
    await page.waitForFunction(
        ({ expectedLabel, expectedSurface }) => {
            const section = document.querySelector(
                '[data-popular-section][data-popular-surface="' + expectedSurface + '"]'
            );
            if (!section) return false;
            const metric = section.querySelector('.note-row .card-meta small span');
            return !!metric && metric.textContent.trim() === expectedLabel;
        },
        { expectedLabel: label, expectedSurface: surface }
    );
}

async function navigationCount(page) {
    return page.evaluate(() => performance.getEntriesByType('navigation').length);
}

async function assertStableUrl(page, path, navigationEntries) {
    const url = new URL(page.url());
    assert.equal(url.pathname, path);
    assert.equal(url.search, '');
    assert.equal(await navigationCount(page), navigationEntries);
}
