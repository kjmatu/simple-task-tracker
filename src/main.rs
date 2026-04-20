use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    commands: Commands,
}


#[derive(Subcommand, Debug)]
enum Commands {
    Start { task_id: u8 },
    Stop {},
    Status {},
}

fn main() {
    let cli = Cli::parse();

    match cli.commands {
        Commands::Start { task_id} => println!("Starting... {}", task_id),
        Commands::Stop {} => println!("Stopping..."),
        Commands::Status {} => println!("Status: Running"),
    }
}
