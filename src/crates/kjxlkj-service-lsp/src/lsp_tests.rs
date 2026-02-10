//! Tests for LSP service.

#[cfg(test)]
mod tests {
    use crate::client::{LspService, ServerConfig, ServerState};
    use crate::types::{LspRequest, Position};
    use std::path::PathBuf;

    #[test]
    fn lsp_register_server() {
        let mut svc = LspService::new();
        let config = ServerConfig {
            cmd: vec!["rust-analyzer".into()],
            filetypes: vec!["rust".into()],
            root_markers: vec!["Cargo.toml".into()],
        };
        svc.register_server("rust", config);
        assert_eq!(svc.server_state("rust"), ServerState::Stopped);
        assert!(svc.server_config("rust").is_some());
    }

    #[test]
    fn lsp_initialize_lifecycle() {
        let mut svc = LspService::new();
        svc.register_server(
            "rust",
            ServerConfig {
                cmd: vec!["rust-analyzer".into()],
                filetypes: vec!["rust".into()],
                root_markers: vec!["Cargo.toml".into()],
            },
        );
        let req = LspRequest::Initialize {
            root: PathBuf::from("/project"),
        };
        let resp = svc.handle_request(&req);
        assert!(resp.is_some());
        assert_eq!(svc.server_state("rust"), ServerState::Initializing);

        svc.mark_ready("rust");
        assert_eq!(svc.server_state("rust"), ServerState::Running);
        let caps = svc.capabilities("rust").unwrap();
        assert!(caps.completion);
        assert!(caps.hover);
        assert!(caps.definition);
    }

    #[test]
    fn lsp_shutdown() {
        let mut svc = LspService::new();
        svc.register_server(
            "python",
            ServerConfig {
                cmd: vec!["pylsp".into()],
                filetypes: vec!["python".into()],
                root_markers: vec!["setup.py".into()],
            },
        );
        let _ = svc.handle_request(&LspRequest::Initialize {
            root: PathBuf::from("/py"),
        });
        svc.mark_ready("python");
        let resp = svc.handle_request(&LspRequest::Shutdown);
        assert!(resp.is_some());
        assert_eq!(svc.server_state("python"), ServerState::ShuttingDown);
    }

    #[test]
    fn lsp_completion_returns_empty() {
        let mut svc = LspService::new();
        svc.register_server(
            "rust",
            ServerConfig {
                cmd: vec!["rust-analyzer".into()],
                filetypes: vec!["rust".into()],
                root_markers: vec![],
            },
        );
        let resp = svc.handle_request(&LspRequest::Completion {
            file: PathBuf::from("main.rs"),
            position: Position::new(0, 5),
        });
        assert!(resp.is_some());
    }

    #[test]
    fn lsp_crash_restart_limit() {
        let mut svc = LspService::new();
        svc.register_server(
            "lua",
            ServerConfig {
                cmd: vec!["lua-language-server".into()],
                filetypes: vec!["lua".into()],
                root_markers: vec![],
            },
        );
        assert!(svc.record_crash("lua")); // 1
        assert!(svc.record_crash("lua")); // 2
        assert!(svc.record_crash("lua")); // 3
        assert!(!svc.record_crash("lua")); // 4 => disabled
    }

    #[test]
    fn lsp_request_ids_increment() {
        let mut svc = LspService::new();
        assert_eq!(svc.next_id(), 1);
        assert_eq!(svc.next_id(), 2);
        assert_eq!(svc.next_id(), 3);
    }

    #[test]
    fn lsp_hover_and_definition() {
        let mut svc = LspService::new();
        svc.register_server(
            "ts",
            ServerConfig {
                cmd: vec!["tsserver".into()],
                filetypes: vec!["typescript".into()],
                root_markers: vec!["tsconfig.json".into()],
            },
        );
        let hover = svc.handle_request(&LspRequest::Hover {
            file: PathBuf::from("index.ts"),
            position: Position::new(10, 3),
        });
        assert!(hover.is_some());

        let defn = svc.handle_request(&LspRequest::Definition {
            file: PathBuf::from("index.ts"),
            position: Position::new(10, 3),
        });
        assert!(defn.is_some());
    }

    #[test]
    fn lsp_rename_and_format() {
        let mut svc = LspService::new();
        let rename = svc.handle_request(&LspRequest::Rename {
            file: PathBuf::from("lib.rs"),
            position: Position::new(5, 10),
            new_name: "new_name".into(),
        });
        assert!(rename.is_some());

        let fmt = svc.handle_request(&LspRequest::Format {
            file: PathBuf::from("lib.rs"),
        });
        assert!(fmt.is_some());
    }
}
