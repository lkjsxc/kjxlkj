import assert from 'node:assert/strict';

export async function verifyPartialResourceNavigation(page, note, previous) {
    const navigationCount = await page.evaluate(() => performance.getEntriesByType('navigation').length);
    await page.locator('.shell-rail').evaluate((node) => {
        node.scrollTop = 180;
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
    assert.ok(
        await page.locator('.shell-rail').evaluate((node) => node.scrollTop >= 160),
        'rail scroll position should survive the transition'
    );
    await Promise.all([
        page.waitForURL((url) => new URL(url).pathname === `/${note.ref}`),
        page.getByText(note.title, { exact: true }).first().click(),
    ]);
    const body = await page.locator('#editor-body').inputValue();
    assert.ok(body.includes('Partial navigation save check.'), 'dirty note should flush before navigation');
}
