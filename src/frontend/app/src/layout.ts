/**
 * Layout and responsive behavior per /docs/spec/ui/layout-and-interaction.md
 *
 * Implements:
 * - E2E-12: compact top-right menu behavior
 * - E2E-19: 320px layout remains usable, no horizontal scroll
 * - E2E-25: compact mode at <=1280px, closes on select
 */

import { MENU_COMPACT_BREAKPOINT, isCompactMode } from './state.js';

/** Layout regions per /docs/spec/ui/layout-and-interaction.md */
export interface LayoutConfig {
  readonly compact: boolean;
  readonly noteListVisible: boolean;
  readonly menuOpen: boolean;
  readonly editorFullWidth: boolean;
}

/** Compute layout config from window width and menu state */
export function computeLayout(windowWidth: number, menuOpen: boolean): LayoutConfig {
  const compact = isCompactMode(windowWidth);
  return {
    compact,
    // In compact mode: note list visible only when menu is open
    noteListVisible: !compact || menuOpen,
    menuOpen,
    // E2E-19: at 320px editor takes full width, no panel
    editorFullWidth: windowWidth <= 480 || (compact && !menuOpen),
  };
}

/**
 * Apply responsive CSS classes to root element.
 * Per E2E-19: no horizontal scrollbar at 320px.
 */
export function applyLayoutClasses(root: HTMLElement, config: LayoutConfig): void {
  root.classList.toggle('compact-mode', config.compact);
  root.classList.toggle('menu-open', config.menuOpen);
  root.classList.toggle('editor-full', config.editorFullWidth);
  root.classList.toggle('note-list-visible', config.noteListVisible);
}

/**
 * E2E-25: in compact mode, selecting a note closes the menu.
 * Returns updated menu state.
 */
export function onNoteSelected(compact: boolean, currentMenuOpen: boolean): boolean {
  if (compact && currentMenuOpen) {
    return false; // close menu after selection
  }
  return currentMenuOpen;
}

/** E2E-12: menu toggle handler */
export function toggleMenu(current: boolean): boolean {
  return !current;
}

/**
 * CSS custom properties for layout.
 * Per /docs/spec/ui/layout-and-interaction.md.
 */
export const LAYOUT_CSS = `
  .compact-mode { --sidebar-width: 0; }
  .compact-mode.menu-open { --sidebar-width: 260px; }
  .editor-full { --editor-width: 100%; }
  .note-list-visible .note-list { display: block; }
  .compact-mode .note-list { display: none; }
  .compact-mode.menu-open .note-list { display: block; position: absolute; z-index: 10; }
  body { overflow-x: hidden; max-width: 100vw; }
`;
