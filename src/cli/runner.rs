use crate::error::AppError;

use super::runner_commands::{
    run_compose_verify, run_docs_validate_terms, run_docs_validate_topology,
    run_quality_check_lines,
};
use super::CommandResult;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum CliCommand {
    DocsValidateTopology,
    DocsValidateTerms,
    QualityCheckLines,
    ComposeVerify,
}

pub async fn run_cli(args: &[String]) -> Result<CommandResult, AppError> {
    match parse_command(args)? {
        CliCommand::DocsValidateTopology => Ok(run_docs_validate_topology()),
        CliCommand::DocsValidateTerms => Ok(run_docs_validate_terms()),
        CliCommand::QualityCheckLines => Ok(run_quality_check_lines()),
        CliCommand::ComposeVerify => Ok(run_compose_verify()),
    }
}

pub(super) fn parse_command(args: &[String]) -> Result<CliCommand, AppError> {
    let as_str = args.iter().map(String::as_str).collect::<Vec<_>>();
    match as_str.as_slice() {
        ["docs", "validate-topology"] => Ok(CliCommand::DocsValidateTopology),
        ["docs", "validate-terms"] => Ok(CliCommand::DocsValidateTerms),
        ["quality", "check-lines"] => Ok(CliCommand::QualityCheckLines),
        ["compose", "verify"] => Ok(CliCommand::ComposeVerify),
        _ => Err(AppError::unsupported_command(if args.is_empty() {
            String::new()
        } else {
            args.join(" ")
        })),
    }
}
