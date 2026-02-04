//! Integration tests for core-edit.

use crate::*;
use kjxlkj_core_types::CharOffset;

#[test]
fn edit_operations_composable() {
    let ops = vec![
        EditOp::insert(CharOffset::new(0), "hello"),
        EditOp::insert(CharOffset::new(5), " world"),
    ];
    let batch = EditOp::batch(ops);
    match batch {
        EditOp::Batch(ops) => assert_eq!(ops.len(), 2),
        _ => panic!("expected batch"),
    }
}

#[test]
fn operator_with_register() {
    let op = Operator::new(OperatorKind::Yank).with_register('a');
    assert_eq!(op.register, Some('a'));
}

#[test]
fn motion_inclusive() {
    let m = Motion::char_motion(true, 1).inclusive();
    assert!(m.inclusive);
}
