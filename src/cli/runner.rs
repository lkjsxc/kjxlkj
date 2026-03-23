use crate::error::AppError;

use super::compose::run_compose_verify;
use super::line_limits::{run_docs_validate_terms, run_quality_check_lines};
use super::topology::run_docs_validate_topology;
use super::CommandResult;

pub async fn run_cli(args: &[String]) -> Result<CommandResult, AppError> {
    let parts = args.iter().map(String::as_str).collect::<Vec<_>>();
    let result = match parts.as_slice() {
        ["docs", "validate-topology"] => run_docs_validate_topology(),
        ["docs", "validate-terms"] => run_docs_validate_terms(),
        ["quality", "check-lines"] => run_quality_check_lines(),
        ["compose", "verify"] => run_compose_verify(),
        _ => {
            return Err(AppError::UnsupportedCommand(if args.is_empty() {
                String::new()
            } else {
                args.join(" ")
            }));
        }
    };
    Ok(result)
}
