#[cfg(test)]
mod tests {
    use crate::error::AppError;

    use super::super::runner::{parse_command, CliCommand};

    #[test]
    fn parse_command_supports_documented_surfaces() {
        assert_eq!(
            parse_command(&args(&["docs", "validate-topology"])).unwrap(),
            CliCommand::DocsValidateTopology
        );
        assert_eq!(
            parse_command(&args(&["docs", "validate-terms"])).unwrap(),
            CliCommand::DocsValidateTerms
        );
        assert_eq!(
            parse_command(&args(&["quality", "check-lines"])).unwrap(),
            CliCommand::QualityCheckLines
        );
        assert_eq!(
            parse_command(&args(&["compose", "verify"])).unwrap(),
            CliCommand::ComposeVerify
        );
    }

    #[test]
    fn parse_command_rejects_unknown_command() {
        let error = parse_command(&args(&["docs", "missing"])).unwrap_err();
        match error {
            AppError::UnsupportedCommand { command } => assert_eq!(command, "docs missing"),
            other => panic!("expected unsupported command error, got {other:?}"),
        }
    }

    #[test]
    fn parse_command_rejects_extra_arguments() {
        let error = parse_command(&args(&["compose", "verify", "--json"])).unwrap_err();
        match error {
            AppError::UnsupportedCommand { command } => {
                assert_eq!(command, "compose verify --json")
            }
            other => panic!("expected unsupported command error, got {other:?}"),
        }
    }

    fn args(values: &[&str]) -> Vec<String> {
        values.iter().map(|value| (*value).to_owned()).collect()
    }
}
