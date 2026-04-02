# Branding and Icon Contract

## Brand Mark

- The shell brand remains `kjxlkj`.
- The product also uses one compact icon mark derived from the reference artwork in `tmp`.
- The icon mark follows the four-solid-color composition from `tmp/icon.svg`.
- The icon mark uses exactly four solid fills: `#0D47A1`, `#1E88E5`, `#4FC3F7`, and `#00BCD4`.
- The icon text is exactly `kjx`.
- The icon text is optically centered using the manually corrected position from the local reference work.
- The outer silhouette is a full circle with transparent pixels outside it.
- The four color blocks are clipped by that outer circle.
- The inner circle is white.
- Pixels outside the circular frame remain transparent.
- The icon has no gradient treatment and no black outer circle or ring.

## Rendering Rules

- Visible shell branding uses a sharp non-ICO asset.
- Production favicon delivery uses `favicon.ico`.
- The generated `.ico` must render cleanly at 16px, 32px, and 48px.
- The icon must keep its text centered and legible at favicon scale.
- The SVG and generated ICO must share the same final composition rather than diverging by medium.
- The icon appears in the HTML document head through `/favicon.ico`.
- The shell brand may pair the icon with the `kjxlkj` wordmark in desktop and compact headers.
- Visual verification must inspect the finished icon asset rather than assuming the SVG scales correctly.
