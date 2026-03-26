import assert from 'node:assert/strict';

export async function expectDarkShell(page, buttonNames = []) {
    const colorScheme = await page.evaluate(
        () => getComputedStyle(document.documentElement).colorScheme
    );
    assert.match(colorScheme, /dark/, 'dark mode should be the default');

    const shell = page.locator('.surface, .index-card, .hero-panel').first();
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
    assert.equal(await page.locator('[data-menu-toggle]').count(), 0, 'drawer controls should not render');

    for (const name of buttonNames) {
        await assertReadable(await namedControl(page, name));
    }
}

export async function expectPublicRoot(page) {
    await expectDarkShell(page);
    await assertVisibleText(page, 'Public index');
    await page.locator('.shell-rail').waitFor({ state: 'visible' });
    await page.getByLabel('Search notes').waitFor({ state: 'visible' });
}

export async function expectAdminNote(page) {
    await expectDarkShell(page, ['New note', 'Logout']);
    await assertVisibleText(page, 'Rich mode');
    await assertVisibleText(page, 'Text mode');
    await assertVisibleText(page, 'Public');
    assert.equal(
        await page.locator('#public-toggle').isChecked(),
        true,
        'admin note should show public checkbox state'
    );
    await assertVisibleText(page, 'Delete note');
    await assertVisibleText(page, 'Created');
    await assertVisibleText(page, 'Updated');
    await assertVisibleText(page, 'Prev');
    await assertVisibleText(page, 'Next');
}

export async function expectGuestNote(page, previousTitle, nextTitle) {
    await expectDarkShell(page);
    await assertVisibleText(page, 'History');
    await assertVisibleText(page, 'All revisions');
    await assertVisibleText(page, 'Prev');
    await assertVisibleText(page, previousTitle);
    await assertVisibleText(page, 'Next');
    await assertVisibleText(page, nextTitle);
}

export async function expectStackedShell(page) {
    const rail = page.locator('.shell-rail');
    const main = page.locator('.shell-main');
    await rail.waitFor({ state: 'visible' });
    await main.waitFor({ state: 'visible' });
    const layout = await Promise.all([
        rail.evaluate((node) => node.getBoundingClientRect()),
        main.evaluate((node) => node.getBoundingClientRect()),
    ]);
    assert.ok(layout[0].top <= layout[1].top, 'rail should appear before the main pane on narrow screens');
    assert.ok(layout[0].left <= layout[1].left + 1, 'stacked rail should stay aligned to the main content edge');
}

export async function assertVisibleText(page, text) {
    await page.getByText(text, { exact: false }).first().waitFor({ state: 'visible' });
}

async function namedControl(page, name) {
    const button = page.getByRole('button', { name, exact: true });
    if (await button.count()) return button;
    return page.getByRole('link', { name, exact: true });
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

function alpha(color) {
    return parseColor(color)[3] ?? 1;
}

function contrast(foreground, background) {
    const fg = parseColor(foreground);
    const bg = parseColor(background);
    const light = luminance(fg);
    const dark = luminance(bg);
    return (Math.max(light, dark) + 0.05) / (Math.min(light, dark) + 0.05);
}

function luminance([red, green, blue]) {
    return [red, green, blue]
        .map((value) => {
            const channel = value / 255;
            return channel <= 0.03928
                ? channel / 12.92
                : ((channel + 0.055) / 1.055) ** 2.4;
        })
        .reduce((total, value, index) => total + value * [0.2126, 0.7152, 0.0722][index], 0);
}

function isDark(color) {
    return luminance(parseColor(color)) < 0.08;
}

function isLight(color) {
    return luminance(parseColor(color)) > 0.35;
}

function parseColor(color) {
    const values = color.match(/[\d.]+/g)?.map(Number);
    if (!values || values.length < 3) throw new Error(`could not parse color: ${color}`);
    return values;
}
