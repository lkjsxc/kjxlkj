import assert from 'node:assert/strict';
import { execFileSync } from 'node:child_process';
import fs from 'node:fs/promises';
import path from 'node:path';
import { chromium } from 'playwright';
import { assertVisibleText, expectAdminNote, expectDarkShell, expectGuestNote, expectPublicRoot, expectStackedShell } from './assertions.mjs';

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
        const notes = await prepareState(browser);
        await capturePublicScreens(browser, notes);
        await captureAdminScreens(browser, notes.middle.id);
        await captureCompactScreens(browser, notes.middle.id);
    } finally {
        await browser.close();
    }

    console.log(JSON.stringify({
        command: 'visual-verify',
        status: 'pass',
        artifacts: ['desktop-public-root.png', 'desktop-admin-dashboard.png', 'desktop-admin-note.png', 'desktop-guest-note.png', 'compact-public-root.png', 'compact-admin-note.png'],
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
        oldest: { id: oldest.id, title: 'Atlas Entry' },
        middle: { id: middle.id, title: 'Orbit Ledger' },
        newest: { id: newest.id, title: 'Beacon Log' },
    };
}

async function captureAdminScreens(browser, id) {
    const context = await browser.newContext({ viewport: { width: 1440, height: 1100 } });
    const page = await context.newPage();
    await login(page);

    await page.goto(`${appUrl}/admin`, { waitUntil: 'networkidle' });
    await expectDarkShell(page);
    await capture(page, 'desktop-admin-dashboard.png');

    await page.goto(`${appUrl}/${id}`, { waitUntil: 'networkidle' });
    await page.waitForSelector('#public-toggle');
    await expectAdminNote(page);
    await capture(page, 'desktop-admin-note.png');

    await page.goto(`${appUrl}/${id}/history/3`, { waitUntil: 'networkidle' });
    await assertVisibleText(page, 'Shared release');
    await context.close();
}

async function capturePublicScreens(browser, notes) {
    const context = await browser.newContext({ viewport: { width: 1440, height: 1100 } });
    const page = await context.newPage();

    await page.goto(`${appUrl}/`, { waitUntil: 'networkidle' });
    await expectPublicRoot(page);
    await capture(page, 'desktop-public-root.png');

    await page.goto(`${appUrl}/${notes.middle.id}`, { waitUntil: 'networkidle' });
    await expectGuestNote(page, notes.oldest.title, notes.newest.title);
    await capture(page, 'desktop-guest-note.png');

    const publicRevision = await page.goto(`${appUrl}/${notes.middle.id}/history/3`, { waitUntil: 'networkidle' });
    assert.equal(publicRevision?.status(), 200, 'public revision should stay guest-readable');

    const privateRevision = await page.goto(`${appUrl}/${notes.middle.id}/history/2`, { waitUntil: 'networkidle' });
    assert.equal(privateRevision?.status(), 404, 'private revision should return 404');
    await assertVisibleText(page, 'Note not found');
    await context.close();
}

async function captureCompactScreens(browser, id) {
    const context = await browser.newContext({ viewport: { width: 390, height: 844 } });
    const page = await context.newPage();

    await page.goto(`${appUrl}/`, { waitUntil: 'networkidle' });
    await expectPublicRoot(page);
    await expectStackedShell(page);
    await capture(page, 'compact-public-root.png');

    await login(page);
    await page.goto(`${appUrl}/${id}`, { waitUntil: 'networkidle' });
    await expectAdminNote(page);
    await expectStackedShell(page);
    await capture(page, 'compact-admin-note.png');
    await context.close();
}

async function createHistoryNote(page) {
    const note = await createNote(page, '# Orbit Ledger\n\nPrivate draft.', true);
    await updateNote(page, note.id, '# Orbit Ledger\n\nSecond private draft.', true);
    await updateNote(page, note.id, '# Orbit Ledger\n\nShared release.', false);
    await updateNote(page, note.id, '# Orbit Ledger\n\nCurrent shared revision.', false);
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

async function updateNote(page, id, body, isPrivate) {
    const status = await page.evaluate(
        async ({ noteId, noteBody, notePrivate }) => {
            const response = await fetch(`/records/${noteId}`, {
                method: 'PUT',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({ body: noteBody, is_private: notePrivate }),
            });
            return response.status;
        },
        { noteId: id, noteBody: body, notePrivate: isPrivate }
    );
    assert.equal(status, 200, `note update should succeed for ${id}`);
}

async function capture(page, name) {
    await page.screenshot({ path: path.join(artifactDir, name), fullPage: true });
}

main().catch((error) => {
    console.error(error);
    process.exit(1);
});
