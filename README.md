# Failure2Fixture

Capture a failed command, save the exact state it failed in, and replay it later to prove it still fails.

Failure2Fixture is not a coding agent. It records failures, reproduces them, and (in later versions) checks candidate fixes. It never writes the fix.

> **Status: early development.** The CLI parses its commands but they are not implemented yet. The interface below is the planned v0.1.

## How it will work

```bash
cd examples/broken-rust-project

f2f capture -- cargo test    # command fails, a fixture is saved
f2f inspect latest           # read what was captured
f2f replay latest            # re-run in a clean directory: REPRODUCED or not
```

A fixture is a folder in `.f2f/fixtures/` holding the command, a copy of every file at that moment (including untracked ones), the output, and the toolchain details.

## Build from source

Requires Rust 1.85 or newer and `git`.

```bash
cargo build --workspace
cargo test --workspace
cargo install --path crates/f2f-cli
```

## Layout

| Path | Purpose |
| --- | --- |
| `crates/f2f-core` | Fixture data model |
| `crates/f2f-capture` | Run a command and capture its output |
| `crates/f2f-store` | Read and write fixtures on disk |
| `crates/f2f-cli` | The `f2f` binary |
| `examples/broken-rust-project` | Deliberately broken demo target |

## License

Licensed under the [Apache License, Version 2.0](LICENSE).
