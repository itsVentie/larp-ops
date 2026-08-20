use clap::{Args, Parser, Subcommand};

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
    let cli = Cli::parse();

    match cli.command {
        Commands::Dfir(dfir) => match dfir.tool {
            DfirTools::Evtx {
                source,
                critical_only,
            } => {
                println!(
                    "[*] Running EVTX triage on: {} (critical_only: {})",
                    source, critical_only
                );
            }
        },
        Commands::Recon(recon) => match recon.tool {
            ReconTools::Scan { target, ports } => {
                println!("[*] Running Port Scan on: {} (ports: {})", target, ports);
            }
        },
    }
}
