use clap::{Args, Parser, Subcommand};
use std::process::exit;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Set(Set),
    Get(Get),
    Rm(Rm),
}

#[derive(Args)]
struct Set {
    #[clap(value_parser)]
    key: Option<String>,
    value: Option<String>,
}

#[derive(Args)]
struct Get {
    #[clap(value_parser)]
    key: Option<String>,
}

#[derive(Args)]
struct Rm {
    #[clap(value_parser)]
    key: Option<String>,
}

fn main() {
    let args = Cli::parse();
    match &args.command {
        Command::Set(set) => {
            eprintln!("unimplemented");
            exit(-1);
        }
        Command::Get(get) => {
            eprintln!("unimplemented");
            exit(-1);
        }
        Command::Rm(rm) => {
            eprintln!("unimplemented");
            exit(-1);
        }
    }
}
