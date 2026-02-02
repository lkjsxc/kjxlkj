//! Tests for terminal service.

#[cfg(test)]
mod pty_tests {
    use crate::Pty;
    use std::path::PathBuf;

    #[test]
    fn test_pty_new() {
        let pty = Pty::new(1, PathBuf::from("/bin/sh"));
        assert_eq!(pty.id(), 1);
        assert!(!pty.is_running());
    }

    #[tokio::test]
    async fn test_pty_start() {
        let mut pty = Pty::new(2, PathBuf::from("/bin/sh"));
        pty.start().await.unwrap();
        assert!(pty.is_running());
    }

    #[tokio::test]
    async fn test_pty_stop() {
        let mut pty = Pty::new(3, PathBuf::from("/bin/sh"));
        pty.start().await.unwrap();
        pty.stop().await;
        assert!(!pty.is_running());
    }

    #[tokio::test]
    async fn test_pty_write() {
        let mut pty = Pty::new(4, PathBuf::from("/bin/sh"));
        pty.start().await.unwrap();
        let result = pty.write(b"echo hello").await;
        assert!(result.is_ok());
    }

    #[test]
    fn test_pty_resize() {
        let mut pty = Pty::new(5, PathBuf::from("/bin/sh"));
        pty.resize(80, 24);
        assert!(!pty.is_running());
    }
}

#[cfg(test)]
mod service_tests {
    use crate::TerminalService;

    #[test]
    fn test_service_new() {
        let svc = TerminalService::new();
        assert!(svc.get(0).is_none());
    }

    #[test]
    fn test_service_create() {
        let mut svc = TerminalService::new();
        let id = svc.create();
        assert!(svc.get(id).is_some());
    }

    #[test]
    fn test_service_remove() {
        let mut svc = TerminalService::new();
        let id = svc.create();
        svc.remove(id);
        assert!(svc.get(id).is_none());
    }

    #[test]
    fn test_service_get_mut() {
        let mut svc = TerminalService::new();
        let id = svc.create();
        assert!(svc.get_mut(id).is_some());
    }

    #[test]
    fn test_service_multiple_ptys() {
        let mut svc = TerminalService::new();
        let id1 = svc.create();
        let id2 = svc.create();
        let id3 = svc.create();
        assert_ne!(id1, id2);
        assert_ne!(id2, id3);
        assert!(svc.get(id1).is_some());
        assert!(svc.get(id2).is_some());
        assert!(svc.get(id3).is_some());
    }
}
