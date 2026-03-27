import assert from 'node:assert/strict';
import { alpha, contrast, isDark, isLight } from './style-utils.mjs';

export async function expectFlatShell(page, buttonNames = []) {
    const colorScheme = await page.evaluate(
        () => getComputedStyle(document.documentElement).colorScheme
    );
    assert.match(colorScheme, /dark/, 'dark mode should be the default');

    const shell = page.locator('.surface, .index-card').first();
    const surface = await shell.evaluate((node) => {
        const style = getComputedStyle(node);
        return {
            background: style.backgroundColor,
            backgroundImage: style.backgroundImage,
            boxShadow: style.boxShadow,
        };
    });
    assert.ok(isDark(surface.background), 'surfaces should be dark by default');
    assert.equal(surface.backgroundImage, 'none', 'surface fills should stay flat');
    assert.equal(surface.boxShadow, 'none', 'surface depth should not rely on shadows');
    assert.equal(
        await page.locator('.shell-rail input[type="search"]').count(),
        0,
        'rail search inputs should not render'
    );
    await assertInvisibleText(page, 'RECENT');
    await assertInvisibleText(page, 'Rich mode');
    await assertInvisibleText(page, 'Text mode');
    await assertInvisibleText(page, 'Saving');
    await assertInvisibleText(page, 'Saved');
    await assertInvisibleText(page, 'flat notes for LLMs');
    await assertNoHorizontalOverflow(page);

    for (const name of buttonNames) {
        await assertReadable(await namedControl(page, name));
    }
}

export async function expectPublicRoot(page) {
    await expectFlatShell(page);
    await assertVisibleText(page, 'Public index');
    await assertVisibleText(page, 'Public notes');
}

export async function expectSearchPage(page) {
    await expectFlatShell(page);
    await assertVisibleText(page, 'Find notes');
    await page.getByLabel('Search notes').waitFor({ state: 'visible' });
}

export async function expectAdminDashboard(page) {
    await expectFlatShell(page, ['Search', 'New note', 'Logout']);
    await assertVisibleText(page, 'Admin index');
    await assertVisibleText(page, 'Admin notes');
    await assertTopRailCreateAction(page);
}

export async function expectAdminNote(page) {
    await expectFlatShell(page);
    await assertVisibleText(page, 'Public');
    assert.equal(
        await page.locator('#public-toggle').isChecked(),
        true,
        'admin note should show public checkbox state'
    );
    const editableCount =
        (await page.locator('.toastui-editor-defaultUI:visible').count()) +
        (await page.locator('#editor-fallback:visible').count());
    assert.ok(editableCount > 0, 'admin note should show an editable workspace');
    await assertVisibleText(page, 'Delete note');
    await assertVisibleText(page, 'Created');
    await assertVisibleText(page, 'Updated');
    await assertVisibleText(page, 'All revisions');
    await assertLocalToastUiAssets(page);
    await assertTopRailCreateAction(page);
}

export async function expectGuestNote(page, previousTitle, nextTitle) {
    await expectFlatShell(page);
    await assertVisibleText(page, 'All revisions');
    await assertVisibleText(page, 'Prev');
    await assertVisibleText(page, previousTitle);
    await assertVisibleText(page, 'Next');
    await assertVisibleText(page, nextTitle);
}

export async function expectClosedDrawer(page) {
    const toggle = page.locator('[data-menu-toggle]');
    await toggle.waitFor({ state: 'visible' });
    assert.equal(await toggle.getAttribute('aria-expanded'), 'false', 'drawer should start closed');
    await page.waitForFunction(() => {
        const node = document.querySelector('[data-menu-panel]');
        return !!node && node.getBoundingClientRect().right < 20;
    });
    const right = await page
        .locator('[data-menu-panel]')
        .evaluate((node) => node.getBoundingClientRect().right);
    assert.ok(right < 20, 'closed drawer should stay off canvas');
}

export async function openDrawer(page) {
    const toggle = page.locator('[data-menu-toggle]');
    await toggle.click();
    await page.waitForFunction(() => document.body.classList.contains('rail-open'));
    await page.waitForFunction(() => {
        const node = document.querySelector('[data-menu-panel]');
        return !!node && node.getBoundingClientRect().right > 200;
    });
    assert.equal(await toggle.getAttribute('aria-expanded'), 'true', 'drawer should open');
    const right = await page
        .locator('[data-menu-panel]')
        .evaluate((node) => node.getBoundingClientRect().right);
    assert.ok(right > 200, 'opened drawer should slide into view');
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
            (alpha(style.background) < 0.2 && isLight(style.color)),
        'button text should remain readable'
    );
}

async function assertLocalToastUiAssets(page) {
    const assetPaths = await page.evaluate(() =>
        Array.from(
            document.querySelectorAll(
                'link[href*="toastui-editor"], script[src*="toastui-editor"]'
            )
        ).map((node) => node.getAttribute('href') ?? node.getAttribute('src'))
    );
    assert.ok(assetPaths.length >= 3, 'admin note should reference local Toast UI assets');
    assert.ok(
        assetPaths.every((path) => path.startsWith('/assets/vendor/toastui/3.2.2/')),
        'Toast UI assets should be served from local versioned routes'
    );
}

async function assertTopRailCreateAction(page) {
    const createControl = page.getByRole('button', { name: 'New note', exact: true }).first();
    if (!(await createControl.count()) || !(await createControl.isVisible())) return;
    const createTop = await createControl.evaluate((node) => node.getBoundingClientRect().top);
    const navigateTop = await page
        .getByText('Navigate', { exact: true })
        .first()
        .evaluate((node) => node.getBoundingClientRect().top);
    assert.ok(createTop < navigateTop, 'New note should appear above Navigate in the rail');
}
