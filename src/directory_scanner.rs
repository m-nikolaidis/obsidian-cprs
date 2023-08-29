use std::fs;

pub struct DirectoryScanner;

impl DirectoryScanner {
    pub fn new() -> DirectoryScanner {
        DirectoryScanner
    }

    pub fn scan_directory(&self, directory: &str) -> Result<Vec<String>, String> {
        let mut file_names = Vec::new();

        // Read the directory entries
        let entries = match fs::read_dir(directory) {
            Ok(entries) => entries,
            Err(err) => return Err(format!("Failed to read directory: {}", err)),
        };

        // Iterate over the directory entries
        for entry in entries {
            if let Ok(entry) = entry {
                if let Ok(file_name) = entry.file_name().into_string() {
                    file_names.push(file_name);
                }
            }
        }

        Ok(file_names)
    }
}
