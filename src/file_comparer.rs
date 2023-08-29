use std::fs;
use std::io::Read;
use similar::{ChangeTag, TextDiff};

pub struct FileComparer;

impl FileComparer {
    pub fn new() -> FileComparer {
        FileComparer
    }

    pub fn compare_files(&self, source_file: &str, destination_file: &str) -> bool {
        let mut source_content = Vec::new();
        let mut destination_content = Vec::new();

        // Read the content of the source file
        if let Ok(mut source_file) = fs::File::open(source_file) {
            if let Err(_) = source_file.read_to_end(&mut source_content) {
                return false;
            }
        } else {
            return false;
        }

        // Read the content of the destination file
        if let Ok(mut destination_file) = fs::File::open(destination_file) {
            if let Err(_) = destination_file.read_to_end(&mut destination_content) {
                return false;
            }
        } else {
            return false;
        }

        let diff = TextDiff::from_lines(
            "Hello World\nThis is the second line.\nThis is the third.",
            "Hallo Welt\nThis is the second line.\nThis is life.\nMoar and more",
        );

        for change in diff.iter_all_changes() {
            let sign = match change.tag() {
                ChangeTag::Delete => "-",
                ChangeTag::Insert => "+",
                ChangeTag::Equal => " ",
            };
            print!("{}{}", sign, change);
        }

        // Compare the content of the files
        source_content == destination_content

    }
}
