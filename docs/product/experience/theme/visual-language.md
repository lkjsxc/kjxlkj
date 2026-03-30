# Visual Language Contract

## Overall Direction

- The product should feel like a focused note workspace, not a generic admin panel.
- Dense layouts are preferred, but density must feel intentional rather than squeezed.
- Desktop browse pages use capped content widths so panels do not become billboard-wide on large screens.
- Visual hierarchy comes from panel proportions, border contrast, and internal separators rather than gradients or shadow stacks.

## Section Hierarchy

- Home and Search begin with a compact lead strip directly under the page title.
- Lead strips are shorter and visually tighter than regular content sections.
- Regular content sections such as `Recently updated`, `Favorites`, and search results remain full-width within the capped content column.
- Dashboard keeps the same page-width cap, but may use a denser grid of stats and settings panels.

## Card Language

- Note cards should feel slightly roomier than the previous compact pass, but still read as dense browse artifacts.
- Card metadata is separated from the body with spacing and a subtle rule, not by large empty padding.
- Hover and focus states may sharpen borders or shift the card slightly, but must stay restrained.
- Long titles and summaries must still clamp cleanly without changing card height.

## Search and Lead Controls

- Home quick search is a compact utility strip, not a hero panel.
- The search input on Home should stay visually narrower than the recent and favorite sections on wide screens.
- Search workspace controls should read as one coherent toolbar rather than a row of loosely related boxes.
- Primary submit actions stay compact and aligned with adjacent fields instead of becoming tall call-to-action blocks.
