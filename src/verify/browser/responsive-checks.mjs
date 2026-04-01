import assert from 'node:assert/strict';
import { assertVisibleText, expectClosedDrawer, expectPublicRoot, openDrawer } from './assertions.mjs';
import { assertEditorLayout, openPreview } from './editor-checks.mjs';
import { appUrl, capture, login, newContext } from './support.mjs';

export async function captureCompactScreens(browser, note, desktopFont) {
    const context = await newContext(
        browser,
        { width: 390, height: 844 },
        {
            isMobile: true,
            hasTouch: true,
            userAgent:
                'Mozilla/5.0 (iPhone; CPU iPhone OS 17_0 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.0 Mobile/15E148 Safari/604.1',
        }
    );
    const page = await context.newPage();

    await page.goto(`${appUrl}/`, { waitUntil: 'networkidle' });
    await expectPublicRoot(page);
    await expectClosedDrawer(page);
    assert.equal(await page.evaluate(() => getComputedStyle(document.body).fontFamily), desktopFont);
    await capture(page, 'compact-public-root-closed.png');

    await openDrawer(page);
    await capture(page, 'compact-public-root-open.png');

    await login(page);
    await page.goto(`${appUrl}/admin`, { waitUntil: 'networkidle' });
    await page.goto(`${appUrl}/${note.id}`, { waitUntil: 'networkidle' });
    assert.equal(new URL(page.url()).pathname, `/${note.ref}`);
    await assertVisibleText(page, 'Delete note');
    await assertVisibleText(page, 'All history');
    await expectClosedDrawer(page);
    await capture(page, 'compact-admin-note.png');
    await openPreview(page);
    await assertEditorLayout(page, true);
    await capture(page, 'compact-admin-note-preview.png');
    await context.close();
}
