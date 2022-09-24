use std::path::PathBuf;

use clap::{Parser, Subcommand, Args};
use crate::obis::Obis;

#[derive(Parser)]
pub struct Cli  {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Convert a obis to hex
    Hex(ObisArgs),
    /// Convert a obis to decimal
    Dec(ObisArgs),
    /// Search a xml file for an obis and give you its information
    Search(SearchArgs),
}

#[derive(Args)]
pub struct ObisArgs {
    /// Obis code you want to convert
    pub code: Obis,
}

#[derive(Args)]
pub struct SearchArgs {
    /// Obis code you want to search for
    pub code: Obis,
    /// Path to the xml you want to search
    pub path: PathBuf,
}