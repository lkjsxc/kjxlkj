import assert from 'node:assert/strict';
import { alpha, contrast, isDark, isLight } from './style-utils.mjs';

export async function expectFlatShell(page, controlNames = []) {
    const colorScheme = await page.evaluate(
        () => getComputedStyle(document.documentElement).colorScheme
    );
    assert.match(colorScheme, /dark/, 'dark mode should be the default');
    await assertDarkSurface(page);
    assert.equal(await page.locator('.shell-rail input[type="search"]').count(), 0);
    assert.equal(await page.locator('.shell-rail h2').count(), 0);
    for (const text of ['RECENT', 'Rich mode', 'Text mode', 'Saving', 'Saved', 'Public index', 'Admin index']) {
        await assertInvisibleText(page, text);
    }
    await assertNoHorizontalOverflow(page);
    for (const name of controlNames) {
        await assertReadable(await namedControl(page, name));
    }
}

export async function expectPublicRoot(page) {
    await expectFlatShell(page);
    await assertVisibleText(page, 'Public notes');
    await assertInvisibleText(page, 'Browse current public notes');
    await assertNoHeaderButtons(page);
    if ((await page.evaluate(() => window.innerWidth)) > 900) {
        await assertWideGrid(page);
    }
}

export async function expectSearchPage(page) {
    await expectFlatShell(page);
    await assertVisibleText(page, 'Find notes');
    await page.getByLabel('Search notes').waitFor({ state: 'visible' });
    await assertNoHeaderButtons(page);
}

export async function expectAdminDashboard(page) {
    await expectFlatShell(page, ['New note', 'Logout']);
    await assertVisibleText(page, 'Admin notes');
    await assertInvisibleText(page, 'Admin browse');
    await assertNoHeaderButtons(page);
    await assertStableMetadata(page, 'Orbit Ledger');
    await assertTopRailCreateAction(page);
}

export async function expectAdminNote(page) {
    await expectFlatShell(page);
    await assertVisibleText(page, 'Public');
    assert.equal(await page.locator('#public-toggle').isChecked(), true);
    assert.equal(await page.locator('#preview-toggle').getAttribute('aria-expanded'), 'false');
    assert.ok(
        (await page.locator('.toastui-editor-defaultUI:visible').count()) +
            (await page.locator('#editor-fallback:visible').count()) > 0
    );
    await assertVisibleText(page, 'Delete note');
    await assertVisibleText(page, 'Created');
    await assertVisibleText(page, 'Updated');
    await assertSingleHistoryCard(page);
    await assertLocalToastUiAssets(page);
    await assertTopRailCreateAction(page);
}

export async function expectGuestNote(page, previousTitle, nextTitle) {
    await expectFlatShell(page);
    await assertSingleHistoryCard(page);
    await assertVisibleText(page, 'Prev');
    await assertVisibleText(page, previousTitle ?? 'No older accessible note.');
    await assertVisibleText(page, 'Next');
    await assertVisibleText(page, nextTitle ?? 'No newer accessible note.');
}

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

export async function assertNoHorizontalOverflow(page) {
    const overflow = await page.evaluate(
        () => document.documentElement.scrollWidth - document.documentElement.clientWidth
    );
    assert.ok(overflow <= 1, `page should not overflow horizontally (saw ${overflow}px)`);
}

async function assertDarkSurface(page) {
    const shell = page.locator('.surface, .index-card').first();
    const style = await shell.evaluate((node) => {
        const computed = getComputedStyle(node);
        return {
            background: computed.backgroundColor,
            backgroundImage: computed.backgroundImage,
            boxShadow: computed.boxShadow,
        };
    });
    assert.ok(isDark(style.background));
    assert.equal(style.backgroundImage, 'none');
    assert.equal(style.boxShadow, 'none');
}

async function assertWideGrid(page) {
    const columns = await page.locator('.public-note-grid .note-row').evaluateAll((nodes) => {
        return new Set(nodes.map((node) => Math.round(node.getBoundingClientRect().left))).size;
    });
    assert.ok(columns > 1, 'wide public browse should use multiple columns');
}

async function assertNoHeaderButtons(page) {
    assert.equal(await page.locator('.page-head .btn').count(), 0);
}

async function assertStableMetadata(page, title) {
    const row = page.locator('.note-row', { has: page.getByText(title, { exact: true }) }).first();
    const heights = await row.locator('.card-meta small').evaluateAll((nodes) =>
        nodes.map((node) => node.getBoundingClientRect().height)
    );
    assert.ok(heights.every((height) => height <= 22), 'timestamps should stay on single lines');
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
        return {
            color: computed.color,
            background: computed.backgroundColor,
            backgroundImage: computed.backgroundImage,
        };
    });
    assert.ok(
        style.backgroundImage !== 'none' ||
            contrast(style.color, style.background) >= 4.2 ||
            (alpha(style.background) < 0.2 && isLight(style.color))
    );
}

async function assertLocalToastUiAssets(page) {
    const assetPaths = await page.evaluate(() =>
        Array.from(document.querySelectorAll('link[href*="toastui-editor"], script[src*="toastui-editor"]')).map(
            (node) => node.getAttribute('href') ?? node.getAttribute('src')
        )
    );
    assert.ok(assetPaths.length >= 3);
    assert.ok(assetPaths.every((path) => path.startsWith('/assets/vendor/toastui/3.2.2/')));
}

async function assertSingleHistoryCard(page) {
    await assertVisibleText(page, 'All history');
    assert.equal(await page.getByText('All history', { exact: true }).count(), 1);
}

async function assertTopRailCreateAction(page) {
    const createControl = page.getByRole('button', { name: 'New note', exact: true }).first();
    if (!(await createControl.count()) || !(await createControl.isVisible())) return;
    const createTop = await createControl.evaluate((node) => node.getBoundingClientRect().top);
    const publicTop = await page
        .getByRole('link', { name: 'Public notes', exact: true })
        .first()
        .evaluate((node) => node.getBoundingClientRect().top);
    assert.ok(createTop < publicTop, 'New note should stay above navigation links');
}
