use serde::Serialize;

#[derive(Serialize)]
struct GateResult {
    command: &'static str,
    status: &'static str,
    details: Option<String>,
}

pub fn pass_gate(cmd: &'static str) {
    let result = GateResult {
        command: cmd,
        status: "pass",
        details: None,
    };
    println!("{}", serde_json::to_string(&result).unwrap());
}

pub fn fail_gate(cmd: &'static str, msg: &str) -> Result<(), Box<dyn std::error::Error>> {
    let result = GateResult {
        command: cmd,
        status: "fail",
        details: Some(msg.to_string()),
    };
    println!("{}", serde_json::to_string(&result).unwrap());
    std::process::exit(1);
}
