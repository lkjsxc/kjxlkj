import assert from 'node:assert/strict';
import { execFileSync } from 'node:child_process';
import fs from 'node:fs/promises';
import path from 'node:path';

export const appUrl = process.env.APP_URL ?? 'http://app:8080';
const databaseUrl =
    process.env.DATABASE_URL ?? 'postgres://kjxlkj:kjxlkj@postgres:5432/kjxlkj';
const artifactDir = process.env.ARTIFACT_DIR ?? '/artifacts';
const adminUsername = 'visual-admin';
const adminPassword = 'visual-pass-123';
const timezoneId = 'Asia/Tokyo';

export async function prepareEnvironment() {
    await fs.mkdir(artifactDir, { recursive: true });
    await waitForHealth();
    resetDatabase();
}

export async function prepareState(browser) {
    const context = await newContext(browser, { width: 1440, height: 1100 });
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

export async function newContext(browser, viewport) {
    return browser.newContext({ viewport, timezoneId });
}

export async function login(page) {
    await page.goto(`${appUrl}/login`, { waitUntil: 'networkidle' });
    await page.getByLabel('Username').fill(adminUsername);
    await page.getByLabel('Password', { exact: true }).fill(adminPassword);
    await Promise.all([
        page.waitForURL('**/admin'),
        page.getByRole('button', { name: 'Sign In' }).click(),
    ]);
}

export async function capture(page, name) {
    await page.screenshot({ path: path.join(artifactDir, name), fullPage: true });
}

async function createHistoryNote(page) {
    const note = await createNote(page, '# Orbit Ledger\n\nPrivate draft.', true);
    await updateNote(page, note.id, '# Orbit Ledger\n\nSecond private draft.', true);
    await updateNote(page, note.id, '# Orbit Ledger\n\nShared release.', false);
    await updateNote(page, note.id, '# Orbit Ledger\n\nCurrent shared revision.', false);
    return note;
}

function resetDatabase() {
    execFileSync(
        'psql',
        [
            databaseUrl,
            '-v',
            'ON_ERROR_STOP=1',
            '-c',
            'TRUNCATE sessions, record_revisions, records, admin_user RESTART IDENTITY CASCADE',
        ],
        { stdio: 'inherit' }
    );
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
