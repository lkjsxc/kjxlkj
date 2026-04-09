import assert from 'node:assert/strict';

export async function assertListRailOrder(page) {
    const newNoteTop = await controlTop(page, 'New note');
    const newMediaTop = await controlTop(page, 'New media');
    const githubTop = await requiredControlTop(page, 'Open GitHub');
    const logoutTop = await controlTop(page, 'Logout');
    const signInTop = await controlTop(page, 'Admin sign in');

    if (newNoteTop !== null) assert.ok(newNoteTop < githubTop, 'New note should stay above Open GitHub');
    if (newNoteTop !== null && newMediaTop !== null) assert.ok(newNoteTop < newMediaTop, 'New note should stay above New media');
    if (newMediaTop !== null) assert.ok(newMediaTop < githubTop, 'New media should stay above Open GitHub');
    if (logoutTop !== null) assert.ok(githubTop < logoutTop, 'Open GitHub should stay above Logout');
    if (signInTop !== null) assert.ok(githubTop < signInTop, 'Open GitHub should stay above Admin sign in');
    assert.ok(!(logoutTop !== null && signInTop !== null), 'list rail should not expose both Logout and Admin sign in');
}

async function requiredControlTop(page, label) {
    const top = await controlTop(page, label);
    assert.notEqual(top, null, `${label} should be visible`);
    return top;
}

async function controlTop(page, label) {
    const locator = await namedControl(page, label);
    if (!locator) return null;
    return locator.evaluate((node) => node.getBoundingClientRect().top);
}

async function namedControl(page, name) {
    const button = page.getByRole('button', { name, exact: true });
    if ((await button.count()) && (await button.first().isVisible())) return button.first();
    const link = page.getByRole('link', { name, exact: true });
    if ((await link.count()) && (await link.first().isVisible())) return link.first();
    return null;
}
