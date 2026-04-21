import { chromium } from 'playwright';
import { captureAdminScreens } from './admin-screens.mjs';
import { capturePublicScreens } from './public-screens.mjs';
import { captureCompactScreens } from './responsive-checks.mjs';
import { prepareEnvironment, prepareState } from './support.mjs';

const ARTIFACTS = [
    'desktop-public-root.png',
    'desktop-live-public.png',
    'desktop-search.png',
    'desktop-live-admin.png',
    'desktop-admin-dashboard.png',
    'desktop-admin-note.png',
    'desktop-history-index.png',
    'desktop-guest-note.png',
    'desktop-login.png',
    'compact-public-root-closed.png',
    'compact-public-root-open.png',
    'compact-admin-note.png',
    'compact-admin-note-preview.png',
    'compact-history-index.png',
];

async function main() {
    await prepareEnvironment();
    const browser = await chromium.launch({ headless: true });
    try {
        const fixtures = await prepareState(browser);
        const desktopFont = await capturePublicScreens(browser, fixtures);
        await captureAdminScreens(browser, fixtures);
        await captureCompactScreens(browser, fixtures.middle, desktopFont);
    } finally {
        await browser.close();
    }
    console.log(JSON.stringify({ command: 'visual-verify', status: 'pass', artifacts: ARTIFACTS }));
}

main().catch((error) => {
    console.error(error);
    process.exit(1);
});
