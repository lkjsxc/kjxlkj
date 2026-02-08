//! Tests for Action enum.

use crate::action::Action;
use crate::Motion;

#[test]
fn action_variants_exist() {
    let _ = Action::MoveCursor(Motion::Left, 1);
    let _ = Action::InsertChar('a');
    let _ = Action::Quit;
    let _ = Action::Resize(80, 24);
    let _ = Action::Nop;
}
