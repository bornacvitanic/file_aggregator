use std::path::PathBuf;
use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(name = "fileagg", about = "File aggregation and distribution utility.")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<AppCommand>,
}

#[derive(Subcommand)]
pub enum AppCommand {
    /// Aggregates file contents
    Aggregate {
        #[command(flatten)]
        options: Options,
    },
    /// Distributes file contents
    Distribute {
        #[command(flatten)]
        options: Options,
    },
}

#[derive(Args)]
pub struct Options {
    /// The path to use (optional, defaults to current directory)
    #[arg(short, long, help = "The path to use for the operation. If not specified, defaults to the current directory.")]
    pub(crate) path: Option<PathBuf>,

    /// Specifies a whitelist of extensions
    #[arg(short, long, value_delimiter = ',', help = "A comma-separated list of file extensions to include. If not specified, all files are included.")]
    pub(crate) extensions: Option<Vec<String>>,
}