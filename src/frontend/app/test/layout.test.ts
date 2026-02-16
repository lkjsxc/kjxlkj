/**
 * Layout contract tests per /docs/spec/technical/testing.md
 *
 * Tests: E2E-12, E2E-19, E2E-25
 */

import { computeLayout, onNoteSelected, toggleMenu } from '../src/layout.js';

// E2E-19: 320px layout remains usable
function testLayout320px(): void {
  const layout = computeLayout(320, false);
  if (!layout.compact) throw new Error('320px must be compact');
  if (!layout.editorFullWidth) throw new Error('320px editor must be full width');
  if (layout.noteListVisible) throw new Error('320px with menu closed: list hidden');
}

// E2E-25: compact mode at <=1280px
function testCompactMode(): void {
  const wide = computeLayout(1920, false);
  if (wide.compact) throw new Error('1920px should not be compact');
  if (!wide.noteListVisible) throw new Error('wide mode: list visible');

  const narrow = computeLayout(1280, false);
  if (!narrow.compact) throw new Error('1280px should be compact');
}

// E2E-25: compact mode closes on select
function testCompactCloseOnSelect(): void {
  const menuOpen = onNoteSelected(true, true);
  if (menuOpen !== false) throw new Error('selecting note in compact should close menu');

  const wideMenuStays = onNoteSelected(false, true);
  if (wideMenuStays !== true) throw new Error('wide mode menu should stay open');
}

// E2E-12: menu toggle
function testMenuToggle(): void {
  if (toggleMenu(false) !== true) throw new Error('toggle false -> true');
  if (toggleMenu(true) !== false) throw new Error('toggle true -> false');
}

// Menu open shows list in compact
function testCompactMenuOpen(): void {
  const layout = computeLayout(768, true);
  if (!layout.noteListVisible) throw new Error('compact with menu open: list visible');
}

testLayout320px();
testCompactMode();
testCompactCloseOnSelect();
testMenuToggle();
testCompactMenuOpen();
console.log('All layout tests passed');
