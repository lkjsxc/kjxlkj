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

export async function resultTitles(page) {
    return page.locator('.note-grid .card-title').evaluateAll((nodes) => nodes.map((node) => node.textContent.trim()));
}

export async function browseCardHrefs(page) {
    return page.locator('.note-row-action').evaluateAll((nodes) => nodes.map((node) => node.getAttribute('href')));
}

export async function configureSettings(page) {
    await page.locator('input[name="home_popular_limit"]').fill('1');
    await page.locator('input[name="home_recent_limit"]').fill('1');
    await page.locator('input[name="home_favorite_limit"]').fill('1');
    await page.locator('select[name="home_favorite_position"]').selectOption('1');
    await page.locator('select[name="home_popular_position"]').selectOption('2');
    await page.locator('select[name="home_recent_position"]').selectOption('3');
    const recent = page.locator('input[name="home_recent_visible"]');
    if (await recent.isChecked()) await recent.uncheck();
    await page.locator('select[name="default_new_note_visibility"]').selectOption('public');
    await Promise.all([
        page.waitForResponse((response) => {
            const url = new URL(response.url());
            return url.pathname === '/settings' && response.request().method() === 'POST';
        }),
        page.getByRole('button', { name: 'Save settings', exact: true }).click(),
    ]);
    await page.waitForLoadState('networkidle');
    assert.equal(await page.locator('select[name="default_new_note_visibility"]').inputValue(), 'public');
    assert.equal(await page.locator('input[name="home_recent_visible"]').isChecked(), false);
}

export async function expectAdminHome(page) {
    await assertVisibleText(page, 'Home');
    await page.locator('form[action="/settings/home-intro"]').waitFor({ state: 'visible' });
    assert.equal(await page.getByText('Recently updated', { exact: true }).count(), 0);
    await assertVisibleText(page, 'All time');
    assert.equal(await sectionCardCount(page, 'Favorites'), 2);
    assert.equal(await sectionCardCount(page, 'Popular notes'), 2);
    assert.equal((await favoriteTitles(page))[0], 'Beacon Log');
    await assertHeadingOrder(page, ['Favorites', 'Popular notes']);
}

export async function saveHomeIntro(page) {
    const textarea = page.locator('form[action="/settings/home-intro"] textarea[name="home_intro_markdown"]');
    await textarea.fill('Refined **Home** intro.\n\nSearch remains the full browse workspace.');
    await Promise.all([
        page.waitForResponse((response) => {
            const url = new URL(response.url());
            return url.pathname === '/settings/home-intro' && response.request().method() === 'POST';
        }),
        page.getByRole('button', { name: 'Save intro', exact: true }).click(),
    ]);
    await page.waitForLoadState('networkidle');
    await assertVisibleText(page, 'Refined');
    await assertVisibleText(page, 'Search remains the full browse workspace.');
}

export async function assertIconAssets(page) {
    const assets = await page.evaluate(async () => {
        const ico = await fetch('/assets/favicon.ico');
        const svg = await fetch('/assets/icon.svg');
        return {
            icoOk: ico.ok,
            icoType: ico.headers.get('content-type'),
            svgOk: svg.ok,
            svgType: svg.headers.get('content-type'),
        };
    });
    assert.equal(assets.icoOk, true);
    assert.equal(assets.svgOk, true);
    assert.match(assets.icoType, /image\/x-icon/);
    assert.match(assets.svgType, /image\/svg\+xml/);
}

async function favoriteTitles(page) {
    return page
        .locator('.section-block.note-section', {
            has: page.getByRole('heading', { name: 'Favorites', exact: true }),
        })
        .locator('.card-title')
        .evaluateAll((nodes) => nodes.map((node) => node.textContent.trim()));
}

async function sectionCardCount(page, title) {
    return page
        .locator('.section-block.note-section,.section-block.home-popular-section,.section-block.home-recent-section', {
            has: page.getByRole('heading', { name: title, exact: true }),
        })
        .locator('.note-row')
        .count();
}

async function assertHeadingOrder(page, titles) {
    const tops = [];
    for (const title of titles) {
        const heading = page.getByRole('heading', { name: title, exact: true }).first();
        await heading.waitFor({ state: 'visible' });
        tops.push(await heading.evaluate((node) => node.getBoundingClientRect().top));
    }
    for (let index = 1; index < tops.length; index += 1) {
        assert.ok(tops[index] > tops[index - 1], 'section order should follow the saved settings');
    }
}
