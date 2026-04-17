import assert from 'node:assert/strict';

export async function verifyClipboardMediaPaste(page, note) {
    const uploadPromise = page.waitForResponse((response) => {
        const url = new URL(response.url());
        return url.pathname === `/resources/${note.id}/media-attachments` && response.request().method() === 'POST';
    });
    await page.locator('#editor-body').evaluate((field) => {
        field.focus();
        field.setSelectionRange(field.value.length, field.value.length);
        const data = new DataTransfer();
        data.items.add(
            new File(
                ['<svg xmlns="http://www.w3.org/2000/svg"><rect width="10" height="10" fill="#7ec8ff"/></svg>'],
                'clipboard-paste.svg',
                { type: 'image/svg+xml' }
            )
        );
        const event = new Event('paste', { bubbles: true, cancelable: true });
        Object.defineProperty(event, 'clipboardData', { value: data });
        field.dispatchEvent(event);
    });
    const payload = await (await uploadPromise).json();
    assert.equal(payload.created_media.length, 1);
    assert.ok(payload.current_resource.body.includes(payload.inserted_markdown));
}
