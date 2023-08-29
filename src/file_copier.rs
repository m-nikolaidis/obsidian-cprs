use std::fs;
use std::io;
use std::path::Path;

pub struct FileCopier;

impl FileCopier {
    pub fn new() -> FileCopier {
        FileCopier
    }

    pub fn copy_file(&self, source_file: &str, destination_file: &str) -> Result<(), String> {
        let source_path = Path::new(source_file);
        let destination_path = Path::new(destination_file);

        // Check if the source file exists
        if !source_path.exists() {
            return Err(format!("Source file {} does not exist.", source_file));
        }

        // Check if the destination file exists
        if destination_path.exists() {
            return Err(format!("Destination file {} already exists.", destination_file));
        }

        // Create the destination directory if it doesn't exist
        if let Some(parent) = destination_path.parent() {
            if !parent.exists() {
                if let Err(err) = fs::create_dir_all(parent) {
                    return Err(format!("Failed to create destination directory: {}", err));
                }
            }
        }

        // Copy the file from source to destination
        if let Err(err) = fs::copy(source_path, destination_path) {
            return Err(format!("Failed to copy file: {}", err));
        }

        Ok(())
    }
}
