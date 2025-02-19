mod cli;
mod file_operations;

use crate::cli::Options;
use clap::Parser;
use cli::{AppCommand, Cli};
use copypasta::{ClipboardContext, ClipboardProvider};
use file_operations::{combine_file_contents, distribute_contents, get_file_paths};

fn main() {
    let cli = Cli::parse();

    let command = cli.command.unwrap_or(AppCommand::Aggregate {
        options: Options {
            path: None,
            extensions: None,
        },
    });

    match command {
        AppCommand::Aggregate { options } => {
            aggregate(options);
        }
        AppCommand::Distribute { options } => {
            distribute(options);
        }
    }
}

fn aggregate(options: cli::Options) {
    println!("Aggregating files:");

    let root_path = options
        .path
        .unwrap_or_else(|| std::env::current_dir().unwrap());

    println!("Using path: {:?}", root_path);

    let extensions = options.extensions.unwrap_or_default();

    let file_paths = get_file_paths(&root_path, &extensions);

    match combine_file_contents(&root_path, &file_paths) {
        Ok(contents) => {
            let mut ctx = ClipboardContext::new().unwrap();
            ctx.set_contents(contents).unwrap();
            println!("Copied contents to clipboard!");
        }
        Err(err) => eprintln!("Error combining file contents: {}", err),
    }
}

fn distribute(options: cli::Options) {
    println!("Distributing files:");

    let root_path = options
        .path
        .unwrap_or_else(|| std::env::current_dir().unwrap());

    println!("Using path: {:?}", root_path);

    let mut ctx = ClipboardContext::new().unwrap();
    let clipboard_text = ctx.get_contents().unwrap();

    if let Err(err) = distribute_contents(&root_path, &clipboard_text) {
        eprintln!("Error distributing file contents: {}", err);
    } else {
        println!("Distributed contents from clipboard!");
    }
}
