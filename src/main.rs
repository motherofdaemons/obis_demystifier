use clap::Parser;

mod obis;
mod args;
mod commands;
fn main() {
    let cli = args::Cli::parse();
    commands::run(cli.command);
}
