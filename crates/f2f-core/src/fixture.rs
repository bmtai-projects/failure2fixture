use std::collections::BTreeMap;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

pub const SCHEMA_VERSION: u32 = 1;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Fixture {
    pub schema_version: u32,
    pub id: String,
    /// RFC 3339, UTC.
    pub created_at: String,
    pub f2f_version: String,
    pub command: CommandSpec,
    pub repo: RepoSnapshot,
    pub environment: EnvFingerprint,
    pub failure: FailureObservation,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommandSpec {
    /// Run directly, never through a shell.
    pub argv: Vec<String>,
    /// Relative to the repository root, so fixtures stay portable.
    pub cwd: PathBuf,
    /// Allowlisted variables only. Secrets must never land here.
    pub env: BTreeMap<String, String>,
    pub timeout_secs: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepoSnapshot {
    pub base_commit: Option<String>,
    pub branch: Option<String>,
    /// Archive of tracked and untracked files, minus ignored ones.
    pub snapshot_file: String,
    pub file_count: u32,
    /// SHA-256 over sorted paths and content hashes.
    pub tree_digest: String,
    /// True if the command changed files while running.
    pub tree_modified_by_command: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EnvFingerprint {
    pub os: String,
    pub arch: String,
    pub rustc: Option<String>,
    pub cargo: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FailureObservation {
    pub exit_code: Option<i32>,
    pub signal: Option<i32>,
    pub timed_out: bool,
    /// Sorted names of failed tests, parsed from the output.
    pub failed_tests: Vec<String>,
    /// First error or panic line. Used when no test names were found.
    pub first_error: Option<String>,
    pub duration_ms: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> Fixture {
        Fixture {
            schema_version: SCHEMA_VERSION,
            id: "fx_TEST".into(),
            created_at: "2026-10-26T00:00:00Z".into(),
            f2f_version: "0.1.0".into(),
            command: CommandSpec {
                argv: vec!["cargo".into(), "test".into()],
                cwd: PathBuf::from("examples/broken-rust-project"),
                env: BTreeMap::new(),
                timeout_secs: None,
            },
            repo: RepoSnapshot {
                base_commit: None,
                branch: None,
                snapshot_file: "snapshot.tar.gz".into(),
                file_count: 4,
                tree_digest: "sha256:00".into(),
                tree_modified_by_command: false,
            },
            environment: EnvFingerprint {
                os: "windows".into(),
                arch: "x86_64".into(),
                rustc: Some("rustc 1.98.1".into()),
                cargo: None,
            },
            failure: FailureObservation {
                exit_code: Some(101),
                signal: None,
                timed_out: false,
                failed_tests: vec!["tests::ten_percent_off_one_thousand".into()],
                first_error: None,
                duration_ms: 1200,
            },
        }
    }

    #[test]
    fn manifest_round_trips() {
        let fixture = sample();
        let json = serde_json::to_string_pretty(&fixture).unwrap();
        let back: Fixture = serde_json::from_str(&json).unwrap();
        assert_eq!(fixture, back);
    }

    #[test]
    fn manifest_carries_schema_version() {
        let json = serde_json::to_value(sample()).unwrap();
        assert_eq!(json["schema_version"], 1);
    }
}
