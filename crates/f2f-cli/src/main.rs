use std::collections::BTreeMap;
use std::error::Error;
use std::path::PathBuf;
use std::process::ExitCode;

use clap::{Parser, Subcommand};
use f2f_core::CommandSpec;
use f2f_store::FixtureStore;

#[derive(Parser)]
#[command(
    name = "f2f",
    version,
    about = "Turn failed commands into reproducible fixtures"
)]
struct Cli {
    #[command(subcommand)]
    command: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    /// Run a command and save a fixture if it fails
    Capture {
        /// Kill the command after this many seconds
        #[arg(long)]
        timeout: Option<u64>,
        /// Command to run, after `--`
        #[arg(last = true, required = true)]
        argv: Vec<String>,
    },
    /// Show what a fixture contains
    Inspect {
        /// Fixture ID, unique prefix, or `latest`
        id: String,
        #[arg(long)]
        json: bool,
    },
    /// Re-run a fixture in a clean directory and compare
    Replay {
        /// Fixture ID, unique prefix, or `latest`
        id: String,
    },
}

type CliResult = Result<ExitCode, Box<dyn Error>>;

fn main() -> ExitCode {
    let cli = Cli::parse();
    let result = match cli.command {
        Cmd::Capture { timeout, argv } => capture(argv, timeout),
        Cmd::Inspect { id, json } => inspect(&id, json),
        Cmd::Replay { id } => replay(&id),
    };
    result.unwrap_or_else(|err| {
        eprintln!("f2f: {err}");
        ExitCode::from(2)
    })
}

fn capture(argv: Vec<String>, timeout: Option<u64>) -> CliResult {
    let spec = CommandSpec {
        argv,
        cwd: PathBuf::from("."),
        env: BTreeMap::new(),
        timeout_secs: timeout,
    };
    f2f_capture::run_command(&spec)?;
    Ok(ExitCode::SUCCESS)
}

fn inspect(id: &str, json: bool) -> CliResult {
    let fixture = FixtureStore::open(".").load(id)?;
    if json {
        println!("{}", serde_json::to_string_pretty(&fixture)?);
    } else {
        println!("Fixture: {}", fixture.id);
        println!("Command: {}", fixture.command.argv.join(" "));
    }
    Ok(ExitCode::SUCCESS)
}

fn replay(id: &str) -> CliResult {
    FixtureStore::open(".").load(id)?;
    Err("replay is not implemented yet".into())
}
