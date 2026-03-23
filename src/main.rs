use std::process::ExitCode;

fn main() -> ExitCode {
    println!("{}", r#"{"status":"phase","name":"prune-bootstrap"}"#);
    ExitCode::SUCCESS
}
