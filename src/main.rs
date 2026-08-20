use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "larp", version, about = "Orchestrator")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Dfir,
    Recon,
}

#[tokio::main]
async fn main() {
    let _cli = Cli::parse();
    println!("larp initialized");
}
