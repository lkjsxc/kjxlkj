import assert from 'node:assert/strict';

export async function assertBrandName(page, expected) {
    const names = await page.locator('.brand-lockup span').evaluateAll((nodes) =>
        nodes
            .filter((node) => {
                const style = getComputedStyle(node);
                const rect = node.getBoundingClientRect();
                return style.display !== 'none' && style.visibility !== 'hidden' && rect.width > 0 && rect.height > 0;
            })
            .map((node) => node.textContent?.trim())
            .filter(Boolean)
    );
    assert.ok(names.includes(expected), `expected visible brand name ${expected}`);
}

export async function assertHead(page, { title, descriptionIncludes, robots, canonical }) {
    if (title) assert.equal(await page.title(), title);
    if (descriptionIncludes) {
        const description = await page.locator('meta[name="description"]').getAttribute('content');
        assert.ok(description?.includes(descriptionIncludes), `expected meta description to include ${descriptionIncludes}`);
    }
    if (robots) {
        assert.equal(await page.locator('meta[name="robots"]').getAttribute('content'), robots);
    }
    const canonicalLocator = page.locator('link[rel="canonical"]');
    if (canonical === null) {
        assert.equal(await canonicalLocator.count(), 0);
    } else if (canonical) {
        assert.equal(await canonicalLocator.count(), 1);
        assert.equal(await canonicalLocator.first().getAttribute('href'), canonical);
    }
}

export async function assertDiscoveryRoutes(page, { sitemapContains }) {
    const payload = await page.evaluate(async () => {
        const robots = await fetch('/robots.txt');
        const sitemap = await fetch('/sitemap.xml');
        return {
            robotsStatus: robots.status,
            robotsText: await robots.text(),
            sitemapStatus: sitemap.status,
            sitemapText: await sitemap.text(),
        };
    });
    assert.equal(payload.robotsStatus, 200);
    assert.equal(payload.sitemapStatus, 200);
    assert.ok(payload.robotsText.includes('Sitemap: '));
    assert.ok(payload.robotsText.includes('Disallow: /search'));
    for (const expected of sitemapContains) {
        assert.ok(payload.sitemapText.includes(expected), `expected sitemap to include ${expected}`);
    }
}
