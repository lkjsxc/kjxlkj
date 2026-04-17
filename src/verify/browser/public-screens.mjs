import assert from 'node:assert/strict';
import { assertVisibleText, expectGuestNote, expectPublicRoot, expectSearchPage } from './assertions.mjs';
import { assertBrandName, assertDiscoveryDisabled, assertHead } from './discoverability-checks.mjs';
import { assertHomeBrowseLinks, assertPopularWindowSwitch, popularTitles } from './home-checks.mjs';
import { assertIconAssets } from './icon-checks.mjs';
import { assertMediaSearchFilter, assertPublicMediaPage } from './media-checks.mjs';
import { appUrl, capture, newContext, submitLogin } from './support.mjs';

export async function capturePublicScreens(browser, notes) {
    const context = await newContext(browser, { width: 1440, height: 1100 });
    const page = await context.newPage();
    const publicSnapshot = notes.middle.snapshots.find((item) => item.snapshot_number === 4);
    const privateSnapshot = notes.middle.snapshots.find((item) => item.snapshot_number === 2);
    assert.ok(publicSnapshot, 'expected public snapshot fixture');
    assert.ok(privateSnapshot, 'expected private snapshot fixture');

    await page.goto(`${appUrl}/`, { waitUntil: 'networkidle' });
    await expectPublicRoot(page);
    await assertBrandName(page, 'kjxlkj');
    await assertHead(page, { title: 'Home | kjxlkj', descriptionIncludes: 'Markdown-first resource system for LLM-operated workflows.', robots: 'noindex,nofollow', canonical: null });
    await assertDiscoveryDisabled(page);
    await assertIconAssets(page);
    assert.equal(await page.getByRole('button', { name: '30d', exact: true }).getAttribute('class'), 'btn btn-primary');
    assert.equal((await popularTitles(page))[0], 'Beacon Log');
    await assertHomeBrowseLinks(page);
    await capture(page, 'desktop-public-root.png');
    await assertPopularWindowSwitch(page, '/', 'home');

    await page.goto(`${appUrl}/search?scope=favorites`, { waitUntil: 'networkidle' });
    await expectSearchPage(page, false);
    await assertHead(page, { title: 'Search | kjxlkj', descriptionIncludes: 'Markdown-first resource system for LLM-operated workflows.', robots: 'noindex,nofollow', canonical: null });
    assert.equal(await page.locator('#search-sort').inputValue(), 'favorite_position_asc');
    assert.equal(await page.getByText('Atlas Entry', { exact: true }).count(), 0);
    await assertVisibleText(page, 'Beacon Log');
    await assertVisibleText(page, 'Orbit Ledger');

    await page.goto(`${appUrl}/search?limit=2`, { waitUntil: 'networkidle' });
    await expectSearchPage(page, false);
    await assertVisibleText(page, 'Resources');
    assert.equal(await page.getByText('Query', { exact: true }).count(), 0);
    await assertVisibleText(page, notes.newest.title);
    await assertVisibleText(page, notes.middle.title);
    assert.equal(await page.locator('#search-sort').inputValue(), 'updated_desc');
    assert.equal(await page.getByRole('button', { name: 'Prev', exact: true }).isDisabled(), true);
    assert.equal(await page.getByRole('button', { name: 'Next', exact: true }).isDisabled(), false);
    await capture(page, 'desktop-search.png');

    await Promise.all([
        page.waitForURL((url) => new URL(url).searchParams.get('direction') === 'next'),
        page.getByRole('button', { name: 'Next', exact: true }).click(),
    ]);
    await assertVisibleText(page, notes.oldest.title);
    assert.equal(await page.getByRole('button', { name: 'Prev', exact: true }).isDisabled(), false);
    await Promise.all([
        page.waitForURL((url) => new URL(url).searchParams.get('direction') === 'prev'),
        page.getByRole('button', { name: 'Prev', exact: true }).click(),
    ]);
    await assertVisibleText(page, notes.newest.title);

    await page.locator('#search-sort').selectOption('title_desc');
    await Promise.all([
        page.waitForURL((url) => new URL(url).searchParams.get('sort') === 'title_desc'),
        page.getByRole('button', { name: 'Search', exact: true }).click(),
    ]);
    await page.waitForLoadState('networkidle');
    const titles = await page.locator('.resource-grid .resource-row[data-card-title]').evaluateAll((nodes) =>
        nodes.map((node) => node.dataset.cardTitle.trim())
    );
    assert.equal(titles[0], notes.image.title);
    assert.equal(await page.locator('#search-sort').inputValue(), 'title_desc');

    await page.goto(`${appUrl}/search?q=orbit`, { waitUntil: 'networkidle' });
    await expectSearchPage(page, true);
    await assertMediaSearchFilter(page, notes, notes.middle.title);

    await page.goto(`${appUrl}/${notes.middle.id}`, { waitUntil: 'networkidle' });
    assert.equal(new URL(page.url()).pathname, `/${notes.middle.ref}`);
    await expectGuestNote(page, notes.oldest.title, notes.newest.title);
    await assertHead(page, { title: `${notes.middle.title} | kjxlkj`, descriptionIncludes: 'Current shared snapshot stretches across the list card', robots: 'noindex,nofollow', canonical: null });
    await capture(page, 'desktop-guest-note.png');

    await page.goto(`${appUrl}/${notes.oldest.ref}`, { waitUntil: 'networkidle' });
    await expectGuestNote(page, notes.video.title, notes.middle.title);

    await page.goto(`${appUrl}/${notes.newest.ref}`, { waitUntil: 'networkidle' });
    await expectGuestNote(page, notes.middle.title, null);
    await assertPublicMediaPage(page, notes.image);
    await assertPublicMediaPage(page, notes.file);
    await assertPublicMediaPage(page, notes.video);

    const publicSnapshotResponse = await page.goto(`${appUrl}/${publicSnapshot.id}`, { waitUntil: 'networkidle' });
    assert.equal(new URL(page.url()).pathname, `/${publicSnapshot.id}`);
    await assertHead(page, { title: `Saved snapshot 4: ${notes.middle.title} | kjxlkj`, descriptionIncludes: 'Saved snapshot 4 for Orbit Ledger.', robots: 'noindex,nofollow', canonical: null });
    const privateSnapshotResponse = await page.goto(`${appUrl}/${privateSnapshot.id}`, { waitUntil: 'networkidle' });
    assert.equal(new URL(page.url()).pathname, `/${privateSnapshot.id}`);
    assert.equal(publicSnapshotResponse?.status(), 200, 'public snapshot should stay guest-readable');
    assert.equal(privateSnapshotResponse?.status(), 404, 'private snapshot should return 404');
    await assertHead(page, { title: 'Not Found | kjxlkj', descriptionIncludes: 'could not be found', robots: 'noindex,nofollow', canonical: null });
    await assertVisibleText(page, 'Resource not found');
    await page.goto(`${appUrl}/search?q=orbit`, { waitUntil: 'networkidle' });
    const signInLink = page.getByRole('link', { name: 'Admin sign in', exact: true });
    assert.match(await signInLink.getAttribute('href'), /return_to=%2Fsearch%3Fq%3Dorbit/);
    await Promise.all([
        page.waitForURL(/\/login\?/),
        signInLink.click(),
    ]);
    await submitLogin(page, '/search?q=orbit');
    const fontFamily = await page.evaluate(() => getComputedStyle(document.body).fontFamily);
    await context.close();
    return fontFamily;
}
