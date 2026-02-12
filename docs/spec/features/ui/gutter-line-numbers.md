# Gutter Line Numbers

Back: [/docs/spec/features/ui/README.md](/docs/spec/features/ui/README.md)

Normative line-number behavior for buffer windows.

## Core Rule

Every visible buffer row must expose line identity in the gutter.

## Display Contract

| Case | Required Display |
|---|---|
| logical line row | absolute 1-based line number |
| wrapped continuation row | continuation marker plus owning line number |
| current line | highlight using `CursorLineNr` |
| non-current line | highlight using `LineNr` |
| below EOF | filler marker `~` with no logical number |

## Width and Alignment

| Requirement | Detail |
|---|---|
| dynamic width | gutter width expands with max visible absolute line number |
| right alignment | numbers align on least-significant digit |
| no truncation | number text must not clip or overlap text area |
| split-safe | each window computes its own gutter width |

## Interaction Rules

| Trigger | Required Behavior |
|---|---|
| vertical scroll | line numbers update deterministically with top line |
| wrap toggle | continuation numbering remains stable for same content/geometry |
| resize | gutter width recalculates without text overlap |
| split close/open | surviving windows keep deterministic numbering |

## Modes and Variants

- absolute numbering is required baseline
- relative/hybrid numbering may be optional configuration overlays
- optional variants must not disable baseline line identity in debug/test dumps

## Mandatory Verification

| ID | Scenario | Required Assertions |
|---|---|---|
| `UI-01` | static view line numbering | each visible row has expected line identity |
| `UI-02R` | wrapped long line under resize churn | continuation numbering is deterministic |
| `UI-03` | split with different viewport tops | each window shows correct local line numbers |
| `UI-04` | wide-glyph wrapped row | line number and text alignment remain valid |

## Related

- viewport: [/docs/spec/features/ui/viewport.md](/docs/spec/features/ui/viewport.md)
- components: [/docs/spec/ui/components.md](/docs/spec/ui/components.md)
- themes: [/docs/spec/ui/themes.md](/docs/spec/ui/themes.md)
