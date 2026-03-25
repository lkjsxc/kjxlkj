import assert from 'node:assert/strict';
import { execFileSync } from 'node:child_process';
import fs from 'node:fs/promises';
import path from 'node:path';
import { chromium } from 'playwright';
import { assertVisibleText, expectAdminNote, expectCompactTrigger, expectDarkShell, expectGuestNote } from './assertions.mjs';

const appUrl = process.env.APP_URL ?? 'http://app:8080';
const databaseUrl =
    process.env.DATABASE_URL ?? 'postgres://kjxlkj:kjxlkj@postgres:5432/kjxlkj';
const artifactDir = process.env.ARTIFACT_DIR ?? '/artifacts';
const adminUsername = 'visual-admin';
const adminPassword = 'visual-pass-123';

async function main() {
    await fs.mkdir(artifactDir, { recursive: true });
    await waitForHealth();
    resetDatabase();

    const browser = await chromium.launch({ headless: true });
    try {
        const slugs = await prepareState(browser);
        await captureAdminScreens(browser, slugs.middle.slug);
        await captureGuestScreens(browser, slugs);
        await captureCompactScreens(browser, slugs.middle.slug);
    } finally {
        await browser.close();
    }

    console.log(JSON.stringify({
        command: 'visual-verify',
        status: 'pass',
        artifacts: ['desktop-admin-dashboard.png', 'desktop-admin-note.png', 'desktop-guest-note.png', 'compact-note-closed.png', 'compact-note-open.png'],
    }));
}

async function prepareState(browser) {
    const context = await browser.newContext({ viewport: { width: 1440, height: 1100 } });
    const page = await context.newPage();
    await setupAdmin(page);
    await login(page);

    const oldest = await createNote(page, '# Atlas Entry\n\nOldest public note.', false);
    const middle = await createHistoryNote(page);
    const newest = await createNote(page, '# Beacon Log\n\nNewest public note.', false);

    await context.close();
    return {
        oldest: { slug: oldest.slug, title: 'Atlas Entry' },
        middle: { slug: middle.slug, title: 'Orbit Ledger' },
        newest: { slug: newest.slug, title: 'Beacon Log' },
    };
}

async function captureAdminScreens(browser, slug) {
    const context = await browser.newContext({ viewport: { width: 1440, height: 1100 } });
    const page = await context.newPage();
    await login(page);

    await page.goto(`${appUrl}/admin`, { waitUntil: 'networkidle' });
    await expectDarkShell(page);
    await capture(page, 'desktop-admin-dashboard.png');

    await page.goto(`${appUrl}/${slug}`, { waitUntil: 'networkidle' });
    await page.waitForSelector('#public-toggle');
    await expectAdminNote(page, slug);
    await capture(page, 'desktop-admin-note.png');

    await page.goto(`${appUrl}/${slug}/history/3`, { waitUntil: 'networkidle' });
    await assertVisibleText(page, 'Shared release');
    await context.close();
}

async function captureGuestScreens(browser, slugs) {
    const context = await browser.newContext({ viewport: { width: 1440, height: 1100 } });
    const page = await context.newPage();

    await page.goto(`${appUrl}/${slugs.middle.slug}`, { waitUntil: 'networkidle' });
    await expectGuestNote(page, slugs.oldest.title, slugs.newest.title);
    await capture(page, 'desktop-guest-note.png');

    const publicRevision = await page.goto(`${appUrl}/${slugs.middle.slug}/history/3`, { waitUntil: 'networkidle' });
    assert.equal(publicRevision?.status(), 200, 'public revision should stay guest-readable');

    const privateRevision = await page.goto(`${appUrl}/${slugs.middle.slug}/history/2`, { waitUntil: 'networkidle' });
    assert.equal(privateRevision?.status(), 404, 'private revision should return 404');
    await assertVisibleText(page, 'Note not found');
    await context.close();
}

async function captureCompactScreens(browser, slug) {
    const context = await browser.newContext({ viewport: { width: 390, height: 844 } });
    const page = await context.newPage();

    await page.goto(`${appUrl}/${slug}`, { waitUntil: 'networkidle' });
    await expectCompactTrigger(page, '[data-menu-toggle]');
    await capture(page, 'compact-note-closed.png');

    await page.click('[data-menu-toggle]');
    await page.waitForSelector('.app-shell.drawer-open');
    await expectCompactTrigger(page, '.rail-close');
    await capture(page, 'compact-note-open.png');
    await context.close();
}

async function createHistoryNote(page) {
    const note = await createNote(page, '# Orbit Ledger\n\nPrivate draft.', true);
    await updateNote(page, note.slug, '# Orbit Ledger\n\nSecond private draft.', true);
    await updateNote(page, note.slug, '# Orbit Ledger\n\nShared release.', false);
    await updateNote(page, note.slug, '# Orbit Ledger\n\nCurrent shared revision.', false);
    return note;
}

function resetDatabase() {
    execFileSync('psql', [databaseUrl, '-v', 'ON_ERROR_STOP=1', '-c', 'TRUNCATE sessions, record_revisions, records, admin_user RESTART IDENTITY CASCADE'], { stdio: 'inherit' });
}

async function waitForHealth() {
    for (let attempt = 0; attempt < 30; attempt += 1) {
        try {
            if ((await fetch(`${appUrl}/healthz`)).ok) return;
        } catch {}
        await new Promise((resolve) => setTimeout(resolve, 1000));
    }
    throw new Error('app service did not become healthy in time');
}

async function setupAdmin(page) {
    await page.goto(`${appUrl}/setup`, { waitUntil: 'networkidle' });
    await page.getByLabel('Username').fill(adminUsername);
    await page.getByLabel('Password', { exact: true }).fill(adminPassword);
    await page.getByLabel('Confirm Password').fill(adminPassword);
    await Promise.all([
        page.waitForURL('**/login'),
        page.getByRole('button', { name: 'Create Account' }).click(),
    ]);
}

async function login(page) {
    await page.goto(`${appUrl}/login`, { waitUntil: 'networkidle' });
    await page.getByLabel('Username').fill(adminUsername);
    await page.getByLabel('Password', { exact: true }).fill(adminPassword);
    await Promise.all([
        page.waitForURL('**/admin'),
        page.getByRole('button', { name: 'Sign In' }).click(),
    ]);
}

async function createNote(page, body, isPrivate) {
    return page.evaluate(
        async ({ noteBody, notePrivate }) => {
            const response = await fetch('/records', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({ body: noteBody, is_private: notePrivate }),
            });
            return response.json();
        },
        { noteBody: body, notePrivate: isPrivate }
    );
}

async function updateNote(page, slug, body, isPrivate) {
    const status = await page.evaluate(
        async ({ noteSlug, noteBody, notePrivate }) => {
            const response = await fetch(`/records/${noteSlug}`, {
                method: 'PUT',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({ body: noteBody, is_private: notePrivate }),
            });
            return response.status;
        },
        { noteSlug: slug, noteBody: body, notePrivate: isPrivate }
    );
    assert.equal(status, 200, `note update should succeed for ${slug}`);
}

async function capture(page, name) {
    await page.screenshot({ path: path.join(artifactDir, name), fullPage: true });
}

main().catch((error) => {
    console.error(error);
    process.exit(1);
});
