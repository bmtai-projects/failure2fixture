//! Runs a command and records how it ended.

use f2f_core::CommandSpec;

#[derive(Debug, thiserror::Error)]
pub enum CaptureError {
    #[error("{0} is not implemented yet")]
    NotImplemented(&'static str),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RunOutcome {
    pub exit_code: Option<i32>,
    pub signal: Option<i32>,
    pub timed_out: bool,
    pub stdout: Vec<u8>,
    pub stderr: Vec<u8>,
    /// stdout and stderr in the order they arrived.
    pub combined: Vec<u8>,
    pub duration_ms: u64,
}

pub fn run_command(_spec: &CommandSpec) -> Result<RunOutcome, CaptureError> {
    Err(CaptureError::NotImplemented("run_command"))
}
