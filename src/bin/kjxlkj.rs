use std::process::ExitCode;

fn main() -> ExitCode {
    println!("{}", r#"{"command":"placeholder","status":"pass"}"#);
    ExitCode::SUCCESS
}
