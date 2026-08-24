# LarpOps 

**LarpOps** is a modular, high-performance SecOps orchestrator built in Rust. Designed for Incident Response (DFIR) and Network Reconnaissance, it establishes a unified UNIX-style **NDJSON** streaming pipeline between CLI modules, a WASM runtime sandbox, remote agents, and a real-time terminal UI.

---

## Architecture Overview

LarpOps uses a Cargo workspace to ensure isolated builds, clear boundary enforcement, and fast execution:

* **`larp` (Core Engine):** CLI dispatcher, playbook execution runner, and stream router.
* **`shared-types`:** Shared NDJSON data schema (`OutputEvent`) and stream IO handlers.
* **`dfir-evtx`:** DFIR artifact ingestion wrapper (Windows Event Logs).
* **`recon-net`:** Network discovery & port scanning execution wrapper.


```

```
              +-----------------------------------+
              |   Playbook Engine / Remote Client |
              +-----------------+-----------------+
                                | (NDJSON Stream)
                                v

```

+------------------+      +------------------+      +------------------+
|   dfir-evtx      | ---> |    larp pipe     | ---> |   ratatui TUI    |
|   recon-net      |      |   (Filter/WASM)  |      |   Dashboard      |
+------------------+      +------------------+      +------------------+
|
v
+------------------+
|  Remote Agent    |
|  (HTTP Stream)   |
+------------------+

```

---

## Features

- **NDJSON Pipeline:** Inter-process communication via structured, newline-delimited JSON.
- **YAML Playbooks:** Declarative task sequence definitions with argument propagation.
- **Real-time TUI:** Interactive event stream monitoring built with `ratatui` and `crossterm`.
- **WASM Engine:** Isolated security plugin sandbox powered by `wasmtime`.
- **Remote Agent (RAEM):** High-throughput execution server with streaming responses via `axum`.
- **Package Installer:** Built-in downloader for external security tool binaries (`larp module install`).

---

## Installation & Building

### Prerequisites

- **Rust:** `1.75.0` or higher
- **Cargo Target Dir (Optional):** Ensure permissions if `CARGO_TARGET_DIR` is set.

### Build from Source

```powershell
# Clone repository
git clone [https://github.com/itsVentie/LarpOps.git](https://github.com/itsVentie/LarpOps.git)
cd LarpOps

# Build workspace in debug mode
cargo build

# Build optimized release binary
cargo build --release

```

---

## Usage Guide

### 1. Execute Playbooks

Define your sequence in YAML:

```yaml
name: "Incident Triage & Recon"
description: "Run EVTX analysis followed by network discovery"
steps:
  - name: "Parse System EVTX Logs"
    module: "dfir-evtx"
    command: "evtx"
    args:
      - "--source"
      - "C:\\Windows\\System32\\winevt\\Logs\\System.evtx"
      - "--critical-only"

  - name: "Gateway Network Scan"
    module: "recon-net"
    command: "scan"
    args:
      - "--target"
      - "192.168.1.1"
      - "--ports"
      - "80,443,22"

```

Run the playbook:

```powershell
cargo run -- playbook --file sample.yaml

```

---

### 2. Stream into Real-Time TUI Dashboard

Pipe any NDJSON event generator straight into the Ratatui monitor:

```powershell
cargo run --quiet -- playbook --file sample.yaml | cargo run --quiet -- tui

```

---

### 3. Remote Agent Execution Mode (RAEM)

**Start Agent (Target Server):**

```powershell
larp agent --bind 0.0.0.0:8080

```

**Dispatch Playbook (Controller Client):**

```powershell
larp remote --target [http://192.168.1.50:8080](http://192.168.1.50:8080) --file sample.yaml | larp tui

```

---

### 4. Sandbox Event Inspection with WASM

Run an isolated `.wasm` security inspection filter over the pipeline:

```powershell
larp playbook --file sample.yaml | larp wasm --plugin ./filters/alert_rules.wasm

```

---

### 5. Install Binary Dependencies

Fetch binary tools directly to your local workspace:

```powershell
larp module install evtx_dump
larp module install nmap

```

---

### 6. Shell Completions

Generate auto-completion scripts for your favorite shell:

```powershell
# PowerShell (current session)
larp completion powershell | Out-String | Invoke-Expression

# Bash / Zsh
larp completion bash > /etc/bash_completion.d/larp

```

---

## Data Schema (`OutputEvent`)

Every event printed by LarpOps strictly adheres to the standard schema:

```json
{
  "module": "playbook-runner",
  "timestamp": "2026-08-21T16:41:41.285405100+00:00",
  "level": "STEP",
  "payload": {
    "name": "Parse System EVTX Logs",
    "module": "dfir-evtx",
    "command": "evtx",
    "step_index": 1
  }
}

```

---

## Roadmap (Phase 5 – Phase 7)

### Phase 5: WASM Plugin SDK & Ecosystem Expansion

* [ ] **LarpOps Plugin SDK (`larp-plugin-sdk`):** Develop a dedicated Rust/C crate providing procedural macros (e.g., `#[larp_plugin]`) for automated NDJSON memory management and serialization.
* [ ] **Threat Intel Engine (Sigma & YARA):** Implement WASM sandboxes pre-compiled with YARA rule matching for binary analysis and Sigma rule evaluation for structured event logs.
* [ ] **Expanded Native Modules:**
* [ ] `dfir-mft`: High-speed Master File Table (MFT) parsing piped to NDJSON.
* [ ] `recon-dns`: Asynchronous DNS brute-forcing and subdomain enumeration.



### Phase 6: Enterprise Remote Agent (RAEM) Hardening

* [ ] **mTLS Authentication & Encryption:** Enforce mutual TLS authentication with client/server certificate validation for agent-controller communication.
* [ ] **Stream Compression:** Integrate real-time `zstd`/`gzip` compression over `axum` HTTP streams to optimize bandwidth across WAN networks.
* [ ] **Persistence & Daemon Mode:** Implement background service capabilities (Windows Service and `systemd` unit generation) with heartbeat monitoring.

### Phase 7: Analytics, Storage & Web UI

* [ ] **SIEM & DB Storage Drivers:** Enable direct sink connectors to route NDJSON event streams into **ClickHouse**, **Elasticsearch**, or local **SQLite** databases.
* [ ] **Real-time Web Dashboard:** Build a Web UI alternative powered by `Axum` and WebSockets for incident timelines and graph visualization.

---

## License

Distributed under the **MIT License**. See `LICENSE` for more details.
