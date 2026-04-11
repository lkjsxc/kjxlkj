# Social Card Contract

## Scope

- Public live resource pages may emit Open Graph and Twitter card metadata.
- Setup, login, admin, settings, search, history, snapshot, and not-found pages do not emit social-card image metadata.
- Social-card URLs are absolute and derive from `public_base_url`.

## Shared Fields

- Public live resource pages emit `og:title`, `og:description`, `og:type`, and `og:url`.
- Public live resource pages emit `twitter:title` and `twitter:description`.
- Text-only public pages use `twitter:card=summary`.
- Pages with an image preview use `twitter:card=summary_large_image`.

## Media Preview Images

- Public image media pages emit `og:image` and `twitter:image` pointing to an absolute WebP derivative URL.
- Public image media pages prefer `variant=display` for the social-card image.
- Vector image notes still emit rasterized WebP social-card images instead of pointing crawlers at raw `SVG`.
- Public video media pages may emit a still-image card URL derived from `variant=card`.
- If no browser-friendly derivative image exists, the page omits social-card image fields instead of pointing crawlers at a browser-unfriendly original binary.
