# LarpOps 

A modular CLI orchestrator written in Rust for Red Team and DFIR workflows. It wraps external binaries, scripts, and internal crates into a single, structured interface to standardize execution flags and data output.

---

## Features

* **Modular Workspace**: Structured as a Cargo workspace where each tool or integration exists as an isolated crate.
* **Process Wrapping**: Executes Go, Python, Bash, or Rust binaries via sub-processes, handling argument mapping transparently.
* **Structured Output**: Standardizes stdout/stderr across tools using NDJSON (JSON Lines) for unix piping (`|`).
* **Declarative Configuration**: Centralized `config.yaml` for managing tool paths, default parameters, and environment settings.
* **Zero External Dependencies**: Compiles to a single static binary for Linux (`musl`) and Windows.

---

## Installation

### Pre-compiled Binary
Download the latest release binary for your platform from the [Releases](https://github.com/username/larp-ops/releases) page and place it in your `$PATH`.

### Build from Source
Requires Rust (1.75+ recommended):

```bash
git clone [https://github.com/itsventie/larp-ops.git](https://github.com/itsventie/larp-ops.git)
cd larp-ops
cargo build --release

```

The resulting binary will be at `target/release/larp`.

---

## Usage

```bash
# General syntax
larp <subcommand> [options]

# Example: Run EVTX triage wrapper
larp dfir triage --source /path/to/system.evtx --severity critical

# Example: Run network reconnaissance wrapper
larp recon scan --target 192.168.1.0/24 --ports 80,443,8080

# Pipe structured NDJSON output into jq or other tools
larp dfir triage --source /path/to/system.evtx | jq '.records[]'

```

---

## Configuration

`larp` checks `~/.config/larp/config.yaml` (or `%APPDATA%\larp\config.yaml` on Windows) for binary locations and global options:

```yaml
tools:
  evtx_dump:
    path: "/usr/local/bin/evtx_dump"
    timeout: 300
  scanner:
    path: "./tools/go_scanner"
    timeout: 60

```

---

## Project Structure

```text
larp-ops/
├── Cargo.toml          # Workspace manifest
├── src/                # Core orchestrator and CLI parser (`clap`)
└── crates/
    ├── dfir-evtx/      # EVTX parsing module wrapper
    ├── recon-net/      # Network scanning module wrapper
    └── shared-types/   # Common JSON schema structs and errors

```

---

## Roadmap

* [ ] Cargo workspace initialization and core CLI parser (`clap`).
* [ ] Process execution manager (`std::process::Command` / `tokio::process`).
* [ ] Global configuration loader (`config.yaml`).
* [ ] NDJSON logging and stdout formatting layer.

* [ ] DFIR artifact parsing wrappers (EVTX, RAM triage).
* [ ] Reconnaissance tool wrappers (Go/Rust port scanners).
* [ ] Inter-module piping (streaming stdout to stdin).
* [ ] YAML playbook parser for multi-step tasks.

* [ ] TUI dashboard implementation via `ratatui`.
* [ ] Shell completion generation (`bash`, `zsh`, `fish`).
* [ ] Modular package downloader (`larp module install <name>`).

* [ ] WASM plugin integration for isolated script execution.
* [ ] Remote agent execution mode.

---

## License

MIT
