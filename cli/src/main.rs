use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "driftlens",
    about = "Detects JPA schema drift before it hits production",
    version
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run a full drift check
    Check {
        #[arg(long, default_value = "driftlens.toml")]
        config: String,

        #[arg(long)]
        database_url: Option<String>,

        #[arg(long)]
        source_dir: Option<String>,

        #[arg(long, default_value = "html")]
        output: String,
    },
    /// Validate configuration without connecting to the database
    Validate {
        #[arg(long, default_value = "driftlens.toml")]
        config: String,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let cli = Cli::parse();

    match cli.command {
        Commands::Check { .. } => {
            println!("DriftLens check — coming soon");
        }
        Commands::Validate { .. } => {
            println!("DriftLens validate — coming soon");
        }
    }

    Ok(())
}