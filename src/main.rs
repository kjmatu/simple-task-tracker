use clap::{Parser, Subcommand};

mod subcommands;

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
    List {},
}

fn main() {
    let cli = Cli::parse();

    match cli.commands {
        Commands::Start { task_id} => subcommands::start(task_id),
        Commands::Stop {} => subcommands::stop(),
        Commands::Status {} => subcommands::status(),
        Commands::List {} => subcommands::list()
    }
}
