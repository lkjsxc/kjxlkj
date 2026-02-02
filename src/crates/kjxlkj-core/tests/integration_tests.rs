//! Integration tests for kjxlkj-core.

use kjxlkj_core::{Action, CoreTask};
use kjxlkj_core_edit::{Motion, MotionKind};
use kjxlkj_core_mode::{Intent, IntentKind};
use kjxlkj_core_types::Mode;
use std::time::Duration;

#[tokio::test]
async fn test_mode_switching() {
    let (task, handle) = CoreTask::new();
    let task_handle = tokio::spawn(task.run());

    // Start in Normal mode
    let snapshot = handle.snapshot();
    assert_eq!(snapshot.mode, Mode::Normal);

    // Switch to Insert mode
    let intent = Intent::change_mode(Mode::Insert);
    let _ = handle.send(Action::Intent(intent)).await;
    tokio::time::sleep(Duration::from_millis(10)).await;

    let snapshot = handle.snapshot();
    assert_eq!(snapshot.mode, Mode::Insert);

    // Switch back to Normal
    let intent = Intent::change_mode(Mode::Normal);
    let _ = handle.send(Action::Intent(intent)).await;
    tokio::time::sleep(Duration::from_millis(10)).await;

    let snapshot = handle.snapshot();
    assert_eq!(snapshot.mode, Mode::Normal);

    let _ = handle.send(Action::Quit).await;
    let _ = task_handle.await;
}

#[tokio::test]
async fn test_text_insertion() {
    let (task, handle) = CoreTask::new();
    let task_handle = tokio::spawn(task.run());

    // Insert text
    let intent = Intent::new(IntentKind::InsertText {
        text: "Hello".to_string(),
    });
    let _ = handle.send(Action::Intent(intent)).await;
    tokio::time::sleep(Duration::from_millis(20)).await;

    // Request snapshot explicitly
    let _ = handle.send(Action::RequestSnapshot).await;
    tokio::time::sleep(Duration::from_millis(10)).await;

    let snapshot = handle.snapshot();
    assert!(!snapshot.views.is_empty(), "Should have at least one view");

    let _ = handle.send(Action::Quit).await;
    let _ = task_handle.await;
}

#[tokio::test]
async fn test_cursor_motion() {
    let (task, handle) = CoreTask::new();
    let task_handle = tokio::spawn(task.run());

    // Insert some text first
    let text = Intent::new(IntentKind::InsertText {
        text: "Hello World".to_string(),
    });
    let _ = handle.send(Action::Intent(text)).await;
    tokio::time::sleep(Duration::from_millis(10)).await;

    // Move to start
    let motion = Intent::motion(Motion::new(MotionKind::BufferStart));
    let _ = handle.send(Action::Intent(motion)).await;
    tokio::time::sleep(Duration::from_millis(10)).await;

    let snapshot = handle.snapshot();
    assert!(!snapshot.views.is_empty());
    let view = &snapshot.views[0];
    assert_eq!(view.cursor.line(), 0);
    assert_eq!(view.cursor.col(), 0);

    let _ = handle.send(Action::Quit).await;
    let _ = task_handle.await;
}

#[tokio::test]
async fn test_quit_action() {
    let (task, handle) = CoreTask::new();
    let task_handle = tokio::spawn(task.run());

    let _ = handle.send(Action::Quit).await;
    let result = task_handle.await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_resize() {
    let (task, handle) = CoreTask::new();
    let task_handle = tokio::spawn(task.run());

    // Resize terminal
    let _ = handle.send(Action::resize(120, 40)).await;
    tokio::time::sleep(Duration::from_millis(10)).await;

    let _ = handle.send(Action::Quit).await;
    let _ = task_handle.await;
}
