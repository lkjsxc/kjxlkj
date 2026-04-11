import fs from 'node:fs/promises';
import path from 'node:path';
import {
    buildVideoUpload,
    createHistoryNote,
    createMedia,
    createNote,
    imageUpload,
    updateResource,
} from './fixture-api.mjs';
import { resetDatabase, seedViewAnalytics } from './seed-state.mjs';

export const appUrl = process.env.APP_URL ?? 'http://app:8080';
const databaseUrl =
    process.env.DATABASE_URL ?? 'postgres://kjxlkj:kjxlkj@postgres:5432/kjxlkj';
const artifactDir = process.env.ARTIFACT_DIR ?? '/artifacts';
const setupCode = process.env.SETUP_CODE ?? 'visual-setup-code';
const adminUsername = 'visual-admin';
const adminPassword = 'visual-pass-123';
const timezoneId = 'Asia/Tokyo';

export async function prepareEnvironment() {
    await fs.mkdir(artifactDir, { recursive: true });
    await waitForHealth();
    resetDatabase(databaseUrl);
}

export async function prepareState(browser) {
    const context = await newContext(browser, { width: 1440, height: 1100 });
    const page = await context.newPage();
    await setupAdmin(page);
    await login(page);

    const image = await createMedia(page, imageUpload, {
        alias: 'orbital-chart',
        isPrivate: false,
        favorite: false,
    });
    await updateResource(
        page,
        image.id,
        '# Orbital Chart\n\nPublic image fixture for media pages and markdown embeds.',
        { alias: 'orbital-chart', isPrivate: false, favorite: false }
    );
    const video = await createMedia(page, await buildVideoUpload(page), {
        alias: 'launch-clip',
        isPrivate: false,
        favorite: false,
    });
    await updateResource(
        page,
        video.id,
        '# Launch Clip\n\nPublic video fixture for media pages and markdown embeds.',
        { alias: 'launch-clip', isPrivate: false, favorite: false }
    );

    const oldest = await createNote(page, '# Atlas Entry\n\nOldest public note.', {
        isPrivate: false,
        alias: 'atlas-entry',
        favorite: false,
    });
    const middle = await createHistoryNote(page, {
        image: { fileHref: '/orbital-chart/file' },
        video: { fileHref: '/launch-clip/file' },
    });
    const newest = await createNote(page, '# Beacon Log\n\nNewest public note.', {
        isPrivate: false,
        alias: 'beacon-log',
        favorite: true,
    });

    seedViewAnalytics(databaseUrl, { oldest, middle, newest });
    await context.close();
    return {
        image: mediaFixture(image, 'Orbital Chart', '.media-surface img'),
        video: mediaFixture(video, 'Launch Clip', '.media-surface video'),
        oldest: { id: oldest.id, ref: oldest.alias ?? oldest.id, title: 'Atlas Entry' },
        middle: {
            id: middle.id,
            ref: middle.alias ?? middle.id,
            snapshots: middle.snapshots,
            title: 'Orbit Ledger',
        },
        newest: { id: newest.id, ref: newest.alias ?? newest.id, title: 'Beacon Log' },
    };
}

export async function newContext(browser, viewport, options = {}) {
    return browser.newContext({ viewport, timezoneId, ...options });
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
    await page.getByLabel('Setup code').fill(setupCode);
    await Promise.all([
        page.waitForURL('**/login'),
        page.getByRole('button', { name: 'Create Account' }).click(),
    ]);
}

function mediaFixture(payload, title, selector) {
    const ref = payload.alias ?? payload.id;
    return {
        id: payload.id,
        ref,
        title,
        fileHref: payload.file_href ?? `/${ref}/file`,
        contentType: payload.content_type,
        selector,
    };
}
