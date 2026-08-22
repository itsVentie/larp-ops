mod config;
mod playbook;
mod tui;

use clap::{Args, Parser, Subcommand};
use config::AppConfig;
use playbook::Playbook;
use shared_types::OutputEvent;

#[derive(Parser)]
#[command(name = "larp", version, about = "Orchestrator")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Dfir(DfirArgs),
    Recon(ReconArgs),
    Pipe(PipeArgs),
    Playbook(PlaybookArgs),
    Tui,
}

#[derive(Args)]
pub struct PlaybookArgs {
    #[arg(short, long)]
    file: String,
}

#[derive(Args)]
pub struct PipeArgs {
    #[arg(short, long)]
    level: Option<String>,
    #[arg(short, long)]
    module: Option<String>,
}

#[derive(Args)]
pub struct DfirArgs {
    #[command(subcommand)]
    pub tool: DfirTools,
}

#[derive(Subcommand)]
pub enum DfirTools {
    Evtx {
        #[arg(short, long)]
        source: String,

        #[arg(short, long, default_value_t = false)]
        critical_only: bool,
    },
}

#[derive(Args)]
pub struct ReconArgs {
    #[command(subcommand)]
    pub tool: ReconTools,
}

#[derive(Subcommand)]
pub enum ReconTools {
    Scan {
        #[arg(short, long)]
        target: String,

        #[arg(short, long, default_value = "top100")]
        ports: String,
    },
}

#[tokio::main]
async fn main() {
    let config = match AppConfig::load() {
        Ok(cfg) => cfg,
        Err(err) => {
            eprintln!("[!] Config load warning: {}", err);
            AppConfig::default()
        }
    };

    let cli = Cli::parse();

    match cli.command {
        Commands::Pipe(args) => {
            OutputEvent::process_stdin(|event| {
                let matches_level = args
                    .level
                    .as_ref()
                    .map_or(true, |l| event.level.eq_ignore_ascii_case(l));

                let matches_module = args
                    .module
                    .as_ref()
                    .map_or(true, |m| event.module.eq_ignore_ascii_case(m));

                if matches_level && matches_module {
                    event.print_ndjson();
                }
            });
        }
        Commands::Playbook(args) => match Playbook::load_from_file(&args.file) {
            Ok(pb) => {
                let start_event = OutputEvent::new(
                    "playbook-runner",
                    "INFO",
                    serde_json::json!({
                        "playbook": pb.name,
                        "description": pb.description,
                        "total_steps": pb.steps.len()
                    }),
                );
                start_event.print_ndjson();

                for (idx, step) in pb.steps.iter().enumerate() {
                    let step_event = OutputEvent::new(
                        "playbook-runner",
                        "STEP",
                        serde_json::json!({
                            "step_index": idx + 1,
                            "name": step.name,
                            "module": step.module,
                            "command": step.command,
                            "args": step.args
                        }),
                    );
                    step_event.print_ndjson();
                }
            }
            Err(err) => {
                let err_event = OutputEvent::new(
                    "playbook-runner",
                    "ERROR",
                    serde_json::json!({ "error": err.to_string() }),
                );
                err_event.print_ndjson();
            }
        },
        Commands::Dfir(dfir) => match dfir.tool {
            DfirTools::Evtx {
                source,
                critical_only,
            } => {
                let evtx_binary = config
                    .tools
                    .get("evtx_dump")
                    .map(|t| t.path.as_str())
                    .unwrap_or("evtx_dump.exe");

                let start_event = OutputEvent::new(
                    "dfir-evtx",
                    "INFO",
                    serde_json::json!({
                        "status": "starting",
                        "source": source,
                        "critical_only": critical_only
                    }),
                );
                start_event.print_ndjson();

                if let Err(err) =
                    dfir_evtx::run_evtx_dump(evtx_binary, &source, critical_only).await
                {
                    let err_event = OutputEvent::new(
                        "dfir-evtx",
                        "ERROR",
                        serde_json::json!({ "error": err.to_string() }),
                    );
                    err_event.print_ndjson();
                }
            }
        },
        Commands::Tui => {
            if let Err(err) = tui::run_dashboard().await {
                eprintln!("[!] TUI Error: {}", err);
            }
        }
        Commands::Recon(recon) => match recon.tool {
            ReconTools::Scan { target, ports } => {
                let scanner_binary = config
                    .tools
                    .get("scanner")
                    .map(|t| t.path.as_str())
                    .unwrap_or("nmap.exe");

                let start_event = OutputEvent::new(
                    "recon-net",
                    "INFO",
                    serde_json::json!({
                        "status": "starting",
                        "target": target,
                        "ports": ports
                    }),
                );
                start_event.print_ndjson();

                if let Err(err) = recon_net::run_port_scan(scanner_binary, &target, &ports).await {
                    let err_event = OutputEvent::new(
                        "recon-net",
                        "ERROR",
                        serde_json::json!({ "error": err.to_string() }),
                    );
                    err_event.print_ndjson();
                }
            }
        },
    }
}
