use std::env;
use std::process;

mod file_comparator;
mod file_copier;
mod file_comparer;
mod user_interface;
mod directory_scanner;

use file_comparator::FileComparator;

fn main() {
    // Get the command line arguments
    let args: Vec<String> = env::args().collect();

    // Check if the correct number of arguments is provided
    if args.len() != 5 {
        eprintln!("Usage: obsidian-cprs -s <source_directory> -d <destination_directory>");
        process::exit(1);
    }

    // Get the source and destination directories from the command line arguments
    let source_directory = &args[2];
    let destination_directory = &args[4];

    // Create a new instance of FileComparator and compare the directories
    let file_comparator = FileComparator::new();
    if let Err(err) = file_comparator.compare_directories(source_directory, destination_directory) {
        eprintln!("Error: {}", err);
        process::exit(1);
    }
}
