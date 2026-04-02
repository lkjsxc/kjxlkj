import assert from 'node:assert/strict';
import { assertVisibleText } from './assertions.mjs';

export async function popularTitles(page) {
    return page
        .locator('.section-block.note-section', {
            has: page.getByRole('heading', { name: 'Popular notes', exact: true }),
        })
        .locator('.card-title')
        .evaluateAll((nodes) => nodes.map((node) => node.textContent.trim()));
}

export async function assertHomeBrowseLinks(page) {
    assert.equal(await browseHref(page, 'Popular notes'), '/search?sort=popular_desc&popular_window=30d');
    assert.equal(await browseHref(page, 'Recently updated'), '/search');
    assert.equal(await browseHref(page, 'Favorites'), '/search?scope=favorites');
}

export async function assertAdminHomeConfiguration(page) {
    await assertVisibleText(page, 'Launchpad');
    await assertVisibleText(page, 'Welcome to Launchpad');
    assert.ok(await page.getByText('All time', { exact: true }).count() >= 1);
    assert.equal(await page.getByRole('heading', { name: 'Recently updated', exact: true }).count(), 0);
    const [favoritesTop, popularTop] = await Promise.all([
        page.getByRole('heading', { name: 'Favorites', exact: true }).evaluate((node) => node.getBoundingClientRect().top),
        page.getByRole('heading', { name: 'Popular notes', exact: true }).evaluate((node) => node.getBoundingClientRect().top),
    ]);
    assert.ok(favoritesTop < popularTop, 'favorites should move ahead of popular notes');
    assert.equal(await page.getByRole('link', { name: /View more notes/i }).count(), 2);
}

async function browseHref(page, heading) {
    return page
        .locator('.section-block.note-section', {
            has: page.getByRole('heading', { name: heading, exact: true }),
        })
        .locator('a.note-row-action')
        .getAttribute('href');
}
