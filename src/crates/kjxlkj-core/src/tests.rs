//! Tests for core crate.

#[cfg(test)]
mod action_tests {
    use crate::{Action, ActionResult};
    use kjxlkj_core_types::Mode;

    #[test]
    fn test_action_resize() {
        let action = Action::resize(80, 24);
        match action {
            Action::Resize(dims) => {
                assert_eq!(dims.width, 80);
                assert_eq!(dims.height, 24);
            }
            _ => panic!("Expected Resize action"),
        }
    }

    #[test]
    fn test_action_quit() {
        let action = Action::Quit;
        match action {
            Action::Quit => {}
            _ => panic!("Expected Quit action"),
        }
    }

    #[test]
    fn test_action_result_ok() {
        let result = ActionResult::Ok;
        match result {
            ActionResult::Ok => {}
            _ => panic!("Expected Ok result"),
        }
    }

    #[test]
    fn test_action_result_mode_changed() {
        let result = ActionResult::ModeChanged(Mode::Insert);
        match result {
            ActionResult::ModeChanged(m) => assert_eq!(m, Mode::Insert),
            _ => panic!("Expected ModeChanged result"),
        }
    }

    #[test]
    fn test_action_open_file() {
        let action = Action::OpenFile { path: "test.txt".to_string() };
        match action {
            Action::OpenFile { path } => assert_eq!(path, "test.txt"),
            _ => panic!("Expected OpenFile action"),
        }
    }

    #[test]
    fn test_action_save_as() {
        let action = Action::SaveAs { path: "new.txt".to_string() };
        match action {
            Action::SaveAs { path } => assert_eq!(path, "new.txt"),
            _ => panic!("Expected SaveAs action"),
        }
    }
}

#[cfg(test)]
mod core_handle_tests {
    use crate::CoreTask;

    #[test]
    fn test_core_task_new() {
        let (task, _handle) = CoreTask::new();
        assert!(task.running);
    }

    #[test]
    fn test_core_handle_snapshot() {
        let (_task, handle) = CoreTask::new();
        let snapshot = handle.snapshot();
        assert_eq!(snapshot.mode, kjxlkj_core_types::Mode::Normal);
    }
}
