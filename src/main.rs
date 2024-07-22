use std::string::String;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use clap::{Parser};
use walkdir::WalkDir;
use copypasta::{ClipboardContext, ClipboardProvider};
use crate::cli::{AppCommand, Cli, Options};

mod cli;

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(AppCommand::Aggregate { options }) => {
            aggregate(options);
        }
        Some(AppCommand::Distribute { options }) => {
            distribute(options);
        }
        None => println!("No command provided"),
    }
}

fn aggregate(options: Options) {
    println!("{}", "Aggregating files...");

    let root_path = options.path.unwrap_or_else(|| std::env::current_dir().unwrap());

    println!("Using path: {:?}", root_path);


    if let Some(extensions) = options.extensions {
        println!("Using extensions: {:?}", extensions);
        // Extension logic here
    }

    let file_paths = get_file_paths(&root_path);

    let contents = combine_file_contents(&root_path, &file_paths);
    match contents {
        None => { eprintln!("No content found.") }
        Some(contents) => {
            let mut ctx = ClipboardContext::new().unwrap();
            ctx.set_contents(contents).unwrap();
            println!("Copied contents to clipboard!")
        }
    }
}

fn combine_file_contents(root_path: &PathBuf, file_paths: &Vec<PathBuf>) -> Option<String> {
    let mut combined_result = String::new();
    for file_path in file_paths.iter() {
        match make_relative(file_path, &root_path) {
            Some(relative_path) => {
                let mut contents = String::new();
                match File::open(file_path) {
                    Ok(mut file) => {
                        if let Err(err) = file.read_to_string(&mut contents) {
                            eprintln!("Error reading file {}: {}", file_path.display(), err);
                        } else {
                            combined_result.push_str(&format!("//{}\n", relative_path.display()));
                            combined_result.push_str(&contents);
                            combined_result.push('\n');
                        }
                    }
                    Err(err) => {
                        eprintln!("Error opening file {}: {}", file_path.display(), err);
                    }
                }
            }
            None => println!("The base path is not an ancestor of the absolute path."),
        }
    }
    if combined_result.is_empty() {
        None
    } else {
        Some(combined_result)
    }
}

fn get_file_paths(root_path: &PathBuf) -> Vec<PathBuf> {
    let mut files = Vec::new();
    for entry in WalkDir::new(&root_path).into_iter().filter_map(|e| e.ok()) {
        if entry.metadata().map(|m| m.is_file()).unwrap_or(false) {
            files.push(entry.path().to_path_buf());
        }
    }
    files
}

fn distribute(options: Options) {
    println!("{}", "Distributing files...");

    let root_path = options.path.unwrap_or_else(|| std::env::current_dir().unwrap());

    println!("Using path: {:?}", root_path);


    if let Some(extensions) = options.extensions {
        println!("Using extensions: {:?}", extensions);
        // Extension logic here
    }

    let mut ctx = ClipboardContext::new().unwrap();
    let clipboard_text = ctx.get_contents().unwrap();
    println!("{}", clipboard_text);
}

fn make_relative(absolute_path: &Path, base_path: &Path) -> Option<PathBuf> {
    // Check if base_path is an ancestor of absolute_path
    if absolute_path.starts_with(base_path) {
        let relative_path = absolute_path.strip_prefix(base_path).ok()?;
        Some(relative_path.to_path_buf())
    } else {
        None
    }
}