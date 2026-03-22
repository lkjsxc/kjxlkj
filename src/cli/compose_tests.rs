use std::cell::RefCell;
use std::collections::VecDeque;
use std::io;

use super::compose::{verify_compose, CommandExecution, CommandRunner};
use super::CommandResult;

struct StubRunner {
    outcomes: RefCell<VecDeque<io::Result<CommandExecution>>>,
    calls: RefCell<Vec<Vec<String>>>,
}

impl StubRunner {
    fn new(outcomes: Vec<io::Result<CommandExecution>>) -> Self {
        Self {
            outcomes: RefCell::new(VecDeque::from(outcomes)),
            calls: RefCell::new(Vec::new()),
        }
    }

    fn calls(&self) -> Vec<Vec<String>> {
        self.calls.borrow().clone()
    }
}

impl CommandRunner for StubRunner {
    fn run(&self, program: &str, args: &[&str]) -> io::Result<CommandExecution> {
        let mut call = vec![program.to_owned()];
        call.extend(args.iter().map(|arg| (*arg).to_owned()));
        self.calls.borrow_mut().push(call);

        self.outcomes
            .borrow_mut()
            .pop_front()
            .expect("missing stubbed compose command outcome")
    }
}

#[test]
fn verify_compose_reports_pass_when_all_steps_succeed() {
    let runner = StubRunner::new(vec![success(), success(), success(), success()]);

    let report = verify_compose(&runner);

    assert_eq!(report.result(), CommandResult::Pass);
    assert_eq!(report.steps_passed(), 4);
    assert_eq!(report.steps.len(), 4);
    assert_eq!(report.failed_step, None);
    assert_eq!(runner.calls().len(), 4);
}

#[test]
fn verify_compose_stops_on_first_failure() {
    let runner = StubRunner::new(vec![success(), failed(Some(17), "build failed"), success()]);

    let report = verify_compose(&runner);

    assert_eq!(report.result(), CommandResult::Fail);
    assert_eq!(report.steps_passed(), 1);
    assert_eq!(report.steps.len(), 2);
    assert_eq!(report.failed_step, Some("build-app"));
    assert_eq!(report.steps[1].result, CommandResult::Fail);
    assert_eq!(runner.calls().len(), 2);
}

fn success() -> io::Result<CommandExecution> {
    Ok(CommandExecution {
        success: true,
        exit_code: Some(0),
        stdout: String::new(),
        stderr: String::new(),
    })
}

fn failed(exit_code: Option<i32>, stderr: &str) -> io::Result<CommandExecution> {
    Ok(CommandExecution {
        success: false,
        exit_code,
        stdout: String::new(),
        stderr: stderr.to_owned(),
    })
}
