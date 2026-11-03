mod command;
mod fs_util;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Place the default configuration file
    Init,
    /// Output a list of files and the number of lines in each file
    List,
}

fn main() {
    let args = Args::parse();

    let result = match &args.command {
        Commands::Init => command::init::init(),
        Commands::List => command::list::list(),
    };

    if let Err(e) = result {
        eprintln!("{e}");
        std::process::exit(1);
    }
}
