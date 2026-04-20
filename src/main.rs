use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    commadn: Commands,
}


#[derive(Subcommand, Debug)]
enum Commands {
    Start {},
    Stop {},
    Status {},
}

fn main() {
    let cli = Cli::parse();

    match cli.commadn {
        Commands::Start {} => println!("Starting..."),
        Commands::Stop {} => println!("Stopping..."),
        Commands::Status {} => println!("Status: Running"),
    }
}
