use crate::file_copier::FileCopier;
use crate::file_comparer::FileComparer;
use crate::user_interface::UserInterface;
use crate::directory_scanner::DirectoryScanner;

pub struct FileComparator {
    file_copier: FileCopier,
    file_comparer: FileComparer,
    user_interface: UserInterface,
    directory_scanner: DirectoryScanner,
}

impl FileComparator {
    pub fn new() -> FileComparator {
        FileComparator {
            file_copier: FileCopier::new(),
            file_comparer: FileComparer::new(),
            user_interface: UserInterface::new(),
            directory_scanner: DirectoryScanner::new(),
        }
    }

    pub fn compare_directories(&self, source_directory: &str, destination_directory: &str) -> Result<(), String> {
        // Scan the source and destination directories
        let source_files = self.directory_scanner.scan_directory(source_directory)?;
        let destination_files = self.directory_scanner.scan_directory(destination_directory)?;

        // Find the unique and common files
        let unique_source_files = self.find_unique_files(&source_files, &destination_files);
        let unique_destination_files = self.find_unique_files(&destination_files, &source_files);
        let common_files = self.find_common_files(&source_files, &destination_files);

        // Copy the unique source files to the destination directory
        for file in unique_source_files {
            let source_file = format!("{}/{}", source_directory, file);
            let destination_file = format!("{}/{}", destination_directory, file);
            if let Err(err) = self.file_copier.copy_file(&source_file, &destination_file) {
                return Err(err);
            }
        }

        // Copy the unique destination files to the source directory
        for file in unique_destination_files {
            let source_file = format!("{}/{}", destination_directory, file);
            let destination_file = format!("{}/{}", source_directory, file);
            if let Err(err) = self.file_copier.copy_file(&source_file, &destination_file) {
                return Err(err);
            }
        }

        // Compare the common files and prompt the user to choose which file to keep
        for file in common_files {
            let source_file = format!("{}/{}", source_directory, file);
            let destination_file = format!("{}/{}", destination_directory, file);
            if !self.file_comparer.compare_files(&source_file, &destination_file) {
                let message = format!("Files {} and {} are different. Which file do you want to keep? (s/d)", source_file, destination_file);
                let user_input = self.user_interface.prompt_user(&message);
                match user_input.as_str() {
                    "s" => {
                        if let Err(err) = self.file_copier.copy_file(&source_file, &destination_file) {
                            return Err(err);
                        }
                    }
                    "d" => {
                        if let Err(err) = self.file_copier.copy_file(&destination_file, &source_file) {
                            return Err(err);
                        }
                    }
                    _ => {
                        self.user_interface.display_message("Invalid input. Skipping file comparison.");
                    }
                }
            }
        }

        Ok(())
    }

    fn find_unique_files(&self, files1: &[String], files2: &[String]) -> Vec<String> {
        let mut unique_files = Vec::new();
        for file in files1 {
            if !files2.contains(file) {
                unique_files.push(file.clone());
            }
        }
        unique_files
    }

    fn find_common_files(&self, files1: &[String], files2: &[String]) -> Vec<String> {
        let mut common_files = Vec::new();
        for file in files1 {
            if files2.contains(file) {
                common_files.push(file.clone());
            }
        }
        common_files
    }
}
