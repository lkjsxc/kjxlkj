//! Tests for services crate.

mod bus_tests {
    use super::super::bus::{
        FsEvent, GitEvent, IndexEvent, LspEvent, Message, MessageBus, TerminalEvent,
    };
    use kjxlkj_core_types::BufferId;
    use std::path::PathBuf;

    #[test]
    fn test_message_bus_new() {
        let (bus, _rx) = MessageBus::new();
        let _sender = bus.sender();
    }

    #[tokio::test]
    async fn test_message_bus_send() {
        let (bus, mut rx) = MessageBus::new();
        let msg = Message::GitEvent(GitEvent::StatusChanged);
        bus.send(msg).await.unwrap();
        
        let received = rx.recv().await.unwrap();
        assert!(matches!(received, Message::GitEvent(GitEvent::StatusChanged)));
    }

    #[test]
    fn test_fs_event_file_changed() {
        let event = FsEvent::FileChanged(PathBuf::from("/tmp/test.txt"));
        let msg = Message::FsEvent(event);
        assert!(matches!(msg, Message::FsEvent(FsEvent::FileChanged(_))));
    }

    #[test]
    fn test_fs_event_file_created() {
        let event = FsEvent::FileCreated(PathBuf::from("/new/file.rs"));
        assert!(matches!(event, FsEvent::FileCreated(_)));
    }

    #[test]
    fn test_fs_event_file_deleted() {
        let event = FsEvent::FileDeleted(PathBuf::from("/old/file.rs"));
        assert!(matches!(event, FsEvent::FileDeleted(_)));
    }

    #[test]
    fn test_lsp_event_started() {
        let event = LspEvent::Started {
            language: "rust".to_string(),
        };
        if let LspEvent::Started { language } = event {
            assert_eq!(language, "rust");
        }
    }

    #[test]
    fn test_lsp_event_diagnostics() {
        let event = LspEvent::Diagnostics {
            buffer_id: BufferId::new(1),
        };
        assert!(matches!(event, LspEvent::Diagnostics { .. }));
    }

    #[test]
    fn test_git_event_hunks_updated() {
        let event = GitEvent::HunksUpdated {
            buffer_id: BufferId::new(5),
        };
        if let GitEvent::HunksUpdated { buffer_id } = event {
            assert_eq!(buffer_id.raw(), 5);
        }
    }

    #[test]
    fn test_index_event_complete() {
        let event = IndexEvent::IndexComplete;
        assert!(matches!(event, IndexEvent::IndexComplete));
    }

    #[test]
    fn test_terminal_event_output() {
        let event = TerminalEvent::Output {
            id: 42,
            data: "hello".to_string(),
        };
        if let TerminalEvent::Output { id, data } = event {
            assert_eq!(id, 42);
            assert_eq!(data, "hello");
        }
    }

    #[test]
    fn test_terminal_event_exited() {
        let event = TerminalEvent::Exited { id: 1, code: 0 };
        if let TerminalEvent::Exited { id, code } = event {
            assert_eq!(id, 1);
            assert_eq!(code, 0);
        }
    }
}
