use std::env;
use std::process;

mod directory_scanner;
mod filename_comp;
mod file_diffcheck;
mod user_interface;
mod file_copier;

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

    // Create a new instance of DirectoryScanner and compare the directories
    let dir_scanner = directory_scanner::DirectoryScanner::new();
    let source_files = match dir_scanner.scan_directory(source_directory) {
        Ok(source_files) => source_files,
        Err(err) => {
            eprintln!("Error: {}", err);
            process::exit(1);
        }
    };
    let destination_files = match dir_scanner.scan_directory(destination_directory) {
        Ok(destination_files) => destination_files,
        Err(err) => {
            eprintln!("Error: {}", err);
            process::exit(1);
        }
    };

    // Use the file name comparer to compare the files
    let file_comparer = filename_comp::FileNameComp::new();
    let unique_source_files = file_comparer.find_unique_files(&source_files, &destination_files);
    let unique_destination_files = file_comparer.find_unique_files(&destination_files, &source_files);
    let common_files = file_comparer.find_common_files(&source_files, &destination_files);

    // Set up the user interface in order to accept a prompt from the user
    let prompt_interface = user_interface::UserInterface::new();

    // Set up the file copier to copy files on the fly
    let file_copier = file_copier::FileCopier::new();

    // Diff check the common files and prompt the user to choose which file to keep
    let diff_checker = file_diffcheck::DiffChecker::new();
    for file in common_files {
        let source_file = format!("{}/{}", source_directory, file);
        let destination_file = format!("{}/{}", destination_directory, file);
        diff_checker.compare_files(&source_file, &destination_file);
        let message = format!("Files {} (s) and {} (d) are different. Which file do you want to keep? (s/d)", source_file, destination_file);
        let user_input = prompt_interface.prompt_user(&message);
        match user_input.as_str() {
            "s" => {

                if let Err(err) = file_copier.copy_file(&source_file, &destination_file, true) {
                    panic!("Error: {}", err);
                }
            }
            "d" => {
                if let Err(err) = file_copier.copy_file(&destination_file, &source_file, true) {
                    panic!("Error: {}", err);
                }
            }
            _ => {
                prompt_interface.display_message("Invalid input. Skipping file comparison.");
            }
        }
    }

    // Finally copy the unique files
    for file in unique_source_files {
        let source_file = format!("{}/{}", source_directory, file);
        let destination_file = format!("{}/{}", destination_directory, file);
        if let Err(err) = file_copier.copy_file(&source_file, &destination_file, false) {
            panic!("Error: {}", err);
        }
    }

    // Copy the unique destination files to the source directory
    for file in unique_destination_files {
        let source_file = format!("{}/{}", destination_directory, file);
        let destination_file = format!("{}/{}", source_directory, file);
        if let Err(err) = file_copier.copy_file(&source_file, &destination_file, false) {
            panic!("Error: {}", err);
        }
    }
}
