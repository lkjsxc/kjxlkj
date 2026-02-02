//! Core task implementation.

use kjxlkj_core_state::EditorState;
use kjxlkj_core_ui::EditorSnapshot;
use tokio::sync::{mpsc, watch};

use crate::{Action, ActionResult};

/// The core task manages all editor state.
pub struct CoreTask {
    /// Editor state.
    state: EditorState,
    /// Action receiver.
    action_rx: mpsc::Receiver<Action>,
    /// Result sender.
    result_tx: mpsc::Sender<ActionResult>,
    /// Snapshot sender.
    snapshot_tx: watch::Sender<EditorSnapshot>,
    /// Running flag.
    running: bool,
}

/// Handle for communicating with the core task.
pub struct CoreHandle {
    /// Action sender.
    action_tx: mpsc::Sender<Action>,
    /// Snapshot receiver.
    snapshot_rx: watch::Receiver<EditorSnapshot>,
}

impl CoreTask {
    /// Creates a new core task.
    pub fn new() -> (Self, CoreHandle) {
        let (action_tx, action_rx) = mpsc::channel(256);
        let (result_tx, _result_rx) = mpsc::channel::<ActionResult>(256);
        let (snapshot_tx, snapshot_rx) = watch::channel(EditorSnapshot::default());

        let task = Self {
            state: EditorState::new(),
            action_rx,
            result_tx,
            snapshot_tx,
            running: true,
        };

        let handle = CoreHandle {
            action_tx,
            snapshot_rx,
        };

        (task, handle)
    }

    /// Runs the core task.
    pub async fn run(mut self) {
        while self.running {
            if let Some(action) = self.action_rx.recv().await {
                let result = self.process_action(action);
                let _ = self.result_tx.send(result).await;
            } else {
                break;
            }
        }
    }

    /// Processes an action.
    fn process_action(&mut self, action: Action) -> ActionResult {
        match action {
            Action::Intent(intent) => self.process_intent(intent),
            Action::Resize(dims) => {
                self.state.dimensions = dims;
                self.publish_snapshot();
                ActionResult::Ok
            }
            Action::OpenFile { path: _ } => ActionResult::Ok,
            Action::Save => ActionResult::Ok,
            Action::SaveAs { path: _ } => ActionResult::Ok,
            Action::Quit => {
                self.running = false;
                ActionResult::Quit
            }
            Action::ForceQuit => {
                self.running = false;
                ActionResult::Quit
            }
            Action::RequestSnapshot => {
                self.publish_snapshot();
                ActionResult::Snapshot
            }
        }
    }

    /// Processes an intent.
    fn process_intent(&mut self, intent: kjxlkj_core_mode::Intent) -> ActionResult {
        use kjxlkj_core_mode::IntentKind;

        match intent.kind {
            IntentKind::Noop => ActionResult::Ok,
            IntentKind::ChangeMode(mode) => {
                self.state.mode.transition(mode);
                self.publish_snapshot();
                ActionResult::ModeChanged(mode)
            }
            IntentKind::Quit => {
                self.running = false;
                ActionResult::Quit
            }
            _ => {
                self.publish_snapshot();
                ActionResult::Ok
            }
        }
    }

    /// Publishes a snapshot.
    fn publish_snapshot(&self) {
        let snapshot = self.state.snapshot();
        let _ = self.snapshot_tx.send(snapshot);
    }
}

impl CoreHandle {
    /// Sends an action to the core.
    pub async fn send(&self, action: Action) -> Result<(), mpsc::error::SendError<Action>> {
        self.action_tx.send(action).await
    }

    /// Gets the current snapshot.
    pub fn snapshot(&self) -> EditorSnapshot {
        self.snapshot_rx.borrow().clone()
    }

    /// Watches for snapshot changes.
    pub fn watch_snapshots(&self) -> watch::Receiver<EditorSnapshot> {
        self.snapshot_rx.clone()
    }
}

impl Default for CoreTask {
    fn default() -> Self {
        Self::new().0
    }
}
