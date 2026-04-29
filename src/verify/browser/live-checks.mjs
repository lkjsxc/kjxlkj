import assert from 'node:assert/strict';
import { appUrl, newContext } from './support.mjs';

export async function installLiveMediaMocks(context) {
    await context.addInitScript(() => {
        function videoTrack() {
            const canvas = document.createElement('canvas');
            canvas.width = 1280;
            canvas.height = 720;
            const ctx = canvas.getContext('2d');
            let frame = 0;
            function paint() {
                ctx.fillStyle = '#142033';
                ctx.fillRect(0, 0, canvas.width, canvas.height);
                ctx.fillStyle = '#7ec8ff';
                ctx.fillRect(120 + (frame % 180), 120, 360, 220);
                frame += 1;
            }
            paint();
            setInterval(paint, 100);
            const track = canvas.captureStream(30).getVideoTracks()[0];
            track.applyConstraints = async (constraints) => { track._constraints = constraints; };
            return track;
        }
        function audioTrack() {
            const Audio = window.AudioContext || window.webkitAudioContext;
            const ctx = new Audio();
            return ctx.createMediaStreamDestination().stream.getAudioTracks()[0];
        }
        const media = navigator.mediaDevices || {};
        media.getDisplayMedia = async () => new MediaStream([videoTrack()]);
        media.getUserMedia = async (constraints) => {
            const tracks = [];
            if (constraints?.video) tracks.push(videoTrack());
            if (constraints?.audio) tracks.push(audioTrack());
            return new MediaStream(tracks);
        };
        media.enumerateDevices = async () => [
            { kind: 'videoinput', deviceId: 'mock-camera', label: 'Mock Camera' },
        ];
        Object.defineProperty(navigator, 'mediaDevices', { value: media, configurable: true });
    });
}

export async function configureLiveDefaults(page) {
    assert.equal(await page.getByLabel('Live/Default_source').inputValue(), 'screen');
    assert.equal(await page.getByLabel('Live/Default_quality').inputValue(), '1080');
    assert.equal(await page.getByLabel('Live/Default_fps').inputValue(), '60');
    assert.equal(await page.getByLabel('Live/Microphone_default').isChecked(), false);
    await page.getByLabel('Live/Default_source').selectOption('camera');
    await page.getByLabel('Live/Default_quality').selectOption('1440');
    await page.getByLabel('Live/Default_fps').selectOption('45');
    await page.getByLabel('Live/Microphone_default').check();
}

export async function verifyLiveBroadcastLifecycle(browser, adminPage) {
    assert.equal(await adminPage.locator('[data-live-source]').inputValue(), 'camera');
    assert.equal(await adminPage.locator('[data-live-height]').inputValue(), '1440');
    assert.equal(await adminPage.locator('[data-live-fps]').inputValue(), '45');
    assert.equal(await adminPage.locator('[data-live-mic]').isChecked(), true);
    await adminPage.getByRole('button', { name: 'Start broadcast', exact: true }).click();
    await adminPage.getByText('Broadcasting live', { exact: true }).waitFor({ state: 'visible' });
    await adminPage.getByText('0 viewers', { exact: true }).waitFor({ state: 'visible' });

    const guestContext = await newContext(browser, { width: 900, height: 700 });
    await installLiveMediaMocks(guestContext);
    const viewer = await guestContext.newPage();
    await viewer.goto(`${appUrl}/live`, { waitUntil: 'networkidle' });
    await adminPage.getByText('1 viewer', { exact: true }).waitFor({ state: 'visible' });
    await assertViewerMediaPlaying(viewer);

    await adminPage.locator('[data-live-source]').selectOption('screen');
    await adminPage.locator('[data-live-height]').selectOption('720');
    await adminPage.locator('[data-live-fps]').selectOption('30');
    await adminPage.locator('[data-live-mic]').uncheck();
    await adminPage.getByText('Screen active.', { exact: true }).waitFor({ state: 'visible' });

    await Promise.all([
        adminPage.waitForURL((url) => new URL(url).pathname === '/'),
        adminPage.getByRole('link', { name: 'Home', exact: true }).click(),
    ]);
    await assertViewerEnded(viewer);
    await guestContext.close();
}

async function assertViewerMediaPlaying(page) {
    await page.waitForFunction(() => {
        const video = document.querySelector('[data-live-video]');
        return video && video.readyState >= HTMLMediaElement.HAVE_CURRENT_DATA && video.videoWidth > 0;
    }, null, { timeout: 15000 });
    const firstTime = await page.locator('[data-live-video]').evaluate((video) => video.currentTime);
    await page.waitForTimeout(1200);
    const secondTime = await page.locator('[data-live-video]').evaluate((video) => video.currentTime);
    assert.ok(secondTime > firstTime, 'viewer video currentTime should advance');
    await page.waitForFunction(async () => {
        const peer = window.kjxlkjLive?.peer;
        if (!peer?.getStats) return true;
        const stats = await peer.getStats();
        for (const report of stats.values()) {
            if (report.type !== 'inbound-rtp' || report.isRemote) continue;
            if ((report.bytesReceived || 0) > 0 || (report.framesDecoded || 0) > 0) return true;
        }
        return false;
    }, null, { timeout: 15000 });
}

async function assertViewerEnded(page) {
    try {
        await page.getByText('Broadcast ended.', { exact: true }).waitFor({ state: 'visible' });
    } catch (error) {
        const state = await page.locator('[data-live-state]').textContent().catch(() => '');
        const detail = await page.locator('[data-live-detail]').textContent().catch(() => '');
        throw new Error(`viewer did not show broadcast ended; state=${state}; detail=${detail}`);
    }
}
