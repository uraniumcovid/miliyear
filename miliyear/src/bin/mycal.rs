use clap::{Parser, Subcommand};
use chrono::Local;
use milliyear::{conversion, MilliyearDate};

#[derive(Parser)]
#[command(name = "mycal")]
#[command(about = "Milliyear Calendar utility")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Now {
        #[arg(short, long, default_value_t = 3)]
        precision: usize,
    },
    Convert {
        #[arg(short, long)]
        date: String,
        
        #[arg(short, long)]
        time: Option<String>,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Now { precision } => {
            let now = MilliyearDate::now();
            println!("{}", milliyear::display::format_milliyear(&now, precision));
        },
        Commands::Convert { date, time } => {
            // Add conversion logic here
            println!("Converting traditional date {} to milliyear format", date);
        }
    }
}