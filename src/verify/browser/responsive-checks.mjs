import assert from 'node:assert/strict';
import { assertVisibleText, expectClosedDrawer, expectPublicRoot, openDrawer } from './assertions.mjs';
import { assertBrandName, assertHead } from './discoverability-checks.mjs';
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
    await expectPublicRoot(page, {
        title: 'Launchpad',
        intro: 'Welcome to Launchpad',
        sections: ['Favorites', 'Popular'],
    });
    await assertBrandName(page, 'Launchpad');
    await assertHead(page, { title: 'Home | Launchpad', descriptionIncludes: 'Launchpad search surface for public notes.', robots: 'index,follow', canonical: `${appUrl}/` });
    await expectClosedDrawer(page);
    assert.equal(await page.evaluate(() => getComputedStyle(document.body).fontFamily), desktopFont);
    await capture(page, 'compact-public-root-closed.png');

    await openDrawer(page);
    await capture(page, 'compact-public-root-open.png');

    await login(page);
    await page.goto(`${appUrl}/admin`, { waitUntil: 'networkidle' });
    await page.goto(`${appUrl}/${note.id}`, { waitUntil: 'networkidle' });
    assert.equal(new URL(page.url()).pathname, `/${note.ref}`);
    await assertHead(page, { title: `${note.title} | Launchpad`, descriptionIncludes: 'Current shared revision stretches across the list card', robots: 'noindex,nofollow', canonical: null });
    await assertVisibleText(page, 'Delete note');
    await assertVisibleText(page, 'Open GitHub');
    await assertVisibleText(page, 'History');
    await expectClosedDrawer(page);
    await assertHorizontalTimeline(page);
    await capture(page, 'compact-admin-note.png');
    await openPreview(page);
    await assertEditorLayout(page, true);
    await capture(page, 'compact-admin-note-preview.png');
    await context.close();
}

async function assertHorizontalTimeline(page) {
    const metrics = await page.locator('.timeline-slot').evaluateAll((nodes) =>
        nodes.map((node) => {
            const rect = node.getBoundingClientRect();
            return { top: Math.round(rect.top), left: Math.round(rect.left) };
        })
    );
    assert.equal(metrics.length, 2, 'expected two timeline slots');
    assert.ok(Math.abs(metrics[0].top - metrics[1].top) <= 4, 'compact timeline should stay on one row');
    assert.ok(metrics[1].left > metrics[0].left, 'compact timeline should flow horizontally');
}
