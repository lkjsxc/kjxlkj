import assert from 'node:assert/strict';

export async function assertIconAssets(page) {
    const svg = await inspectIcon(page, '/assets/icon.svg', 128);
    const ico = await inspectIcon(page, '/favicon.ico', 64);
    for (const metrics of [svg, ico]) {
        assert.ok(metrics.corners.every((alpha) => alpha < 0.05), 'icon corners should stay transparent');
        assert.ok(metrics.whiteSample[3] > 0.9, 'icon inner circle should stay opaque');
        assertColorApprox(metrics.whiteSample, [255, 255, 255], 26, 'icon inner circle should stay white');
        assertColorApprox(metrics.topLeft, [13, 71, 161], 24, 'icon top-left should match the reference palette');
        assertColorApprox(metrics.topRight, [30, 136, 229], 24, 'icon top-right should match the reference palette');
        assertColorApprox(metrics.bottomLeft, [79, 195, 247], 24, 'icon bottom-left should match the reference palette');
        assertColorApprox(metrics.bottomRight, [0, 188, 212], 24, 'icon bottom-right should match the reference palette');
        assert.ok(metrics.darkCentroidX > 0.34 && metrics.darkCentroidX < 0.66, 'icon text should stay centered horizontally');
        assert.ok(metrics.darkCentroidY > 0.46 && metrics.darkCentroidY < 0.68, 'icon text should sit slightly below center');
    }
}

function assertColorApprox(actual, expected, tolerance, message) {
    for (let index = 0; index < 3; index += 1) {
        assert.ok(Math.abs(actual[index] - expected[index]) <= tolerance, message);
    }
}

async function inspectIcon(page, src, size) {
    return page.evaluate(
        async ({ imageSrc, imageSize }) => {
            const image = await new Promise((resolve, reject) => {
                const node = new Image();
                node.onload = () => resolve(node);
                node.onerror = reject;
                node.src = imageSrc + '?v=' + Date.now();
            });
            const canvas = document.createElement('canvas');
            canvas.width = imageSize;
            canvas.height = imageSize;
            const context = canvas.getContext('2d');
            context.clearRect(0, 0, imageSize, imageSize);
            context.drawImage(image, 0, 0, imageSize, imageSize);
            const data = context.getImageData(0, 0, imageSize, imageSize).data;
            const sample = (x, y) => {
                const index = (y * imageSize + x) * 4;
                return [
                    data[index],
                    data[index + 1],
                    data[index + 2],
                    data[index + 3] / 255,
                ];
            };
            const alphaAt = (x, y) => data[(y * imageSize + x) * 4 + 3] / 255;
            const lightness = (r, g, b) => r * 0.2126 + g * 0.7152 + b * 0.0722;
            const inset = Math.max(2, Math.floor(imageSize * 0.21875));
            const mid = Math.floor(imageSize / 2);
            let mass = 0;
            let sumX = 0;
            let sumY = 0;
            for (let y = 0; y < imageSize; y += 1) {
                for (let x = 0; x < imageSize; x += 1) {
                    const index = (y * imageSize + x) * 4;
                    const alpha = data[index + 3];
                    if (alpha < 220) continue;
                    if (lightness(data[index], data[index + 1], data[index + 2]) > 40) continue;
                    mass += 1;
                    sumX += x;
                    sumY += y;
                }
            }
            return {
                corners: [
                    alphaAt(0, 0),
                    alphaAt(imageSize - 1, 0),
                    alphaAt(0, imageSize - 1),
                    alphaAt(imageSize - 1, imageSize - 1),
                ],
                whiteSample: sample(mid, inset),
                topLeft: sample(inset, inset),
                topRight: sample(imageSize - inset - 1, inset),
                bottomLeft: sample(inset, imageSize - inset - 1),
                bottomRight: sample(imageSize - inset - 1, imageSize - inset - 1),
                darkCentroidX: mass ? sumX / mass / imageSize : 0,
                darkCentroidY: mass ? sumY / mass / imageSize : 0,
            };
        },
        { imageSrc: src, imageSize: size }
    );
}
