use crate::cli::run_cli;

#[tokio::test]
async fn cli_rejects_unknown_command() {
    let args = vec!["unknown".to_owned(), "command".to_owned()];
    let result = run_cli(&args).await;
    assert!(result.is_err());
}
