export function alpha(color) {
    return parseColor(color)[3] ?? 1;
}

export function contrast(foreground, background) {
    const fg = parseColor(foreground);
    const bg = parseColor(background);
    const light = luminance(fg);
    const dark = luminance(bg);
    return (Math.max(light, dark) + 0.05) / (Math.min(light, dark) + 0.05);
}

export function isDark(color) {
    return luminance(parseColor(color)) < 0.08;
}

export function isLight(color) {
    return luminance(parseColor(color)) > 0.35;
}

function luminance([red, green, blue]) {
    return [red, green, blue]
        .map((value) => {
            const channel = value / 255;
            return channel <= 0.03928
                ? channel / 12.92
                : ((channel + 0.055) / 1.055) ** 2.4;
        })
        .reduce((total, value, index) => total + value * [0.2126, 0.7152, 0.0722][index], 0);
}

function parseColor(color) {
    const values = color.match(/[\d.]+/g)?.map(Number);
    if (!values || values.length < 3) throw new Error(`could not parse color: ${color}`);
    return values;
}
