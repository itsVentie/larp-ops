mod config;

use clap::{Args, Parser, Subcommand};
use config::AppConfig;
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

                let event = OutputEvent::new(
                    "dfir-evtx",
                    "INFO",
                    serde_json::json!({
                        "action": "triage_start",
                        "source": source,
                        "critical_only": critical_only,
                        "binary_used": evtx_binary
                    }),
                );
                event.print_ndjson();
            }
        },
        Commands::Recon(recon) => match recon.tool {
            ReconTools::Scan { target, ports } => {
                let scanner_binary = config
                    .tools
                    .get("scanner")
                    .map(|t| t.path.as_str())
                    .unwrap_or("nmap.exe");

                let event = OutputEvent::new(
                    "recon-net",
                    "INFO",
                    serde_json::json!({
                        "action": "scan_start",
                        "target": target,
                        "ports": ports,
                        "binary_used": scanner_binary
                    }),
                );
                event.print_ndjson();
            }
        },
    }
}
