import assert from 'node:assert/strict';

export async function verifyDeleteArming(page) {
    const button = page.getByRole('button', { name: 'Delete note', exact: true });
    await button.click();
    const armed = page.getByRole('button', { name: 'Press again to delete', exact: true });
    await armed.waitFor();
    await page.waitForTimeout(4100);
    await button.waitFor();
    await button.click();
    await armed.waitFor();
    const deleteResponse = page.waitForResponse((response) => {
        return response.request().method() === 'DELETE' &&
            new URL(response.url()).pathname.startsWith('/resources/');
    });
    await Promise.all([
        page.waitForURL((url) => new URL(url).pathname === '/'),
        deleteResponse,
        armed.click(),
    ]);
    assert.equal(new URL(page.url()).pathname, '/');
}
