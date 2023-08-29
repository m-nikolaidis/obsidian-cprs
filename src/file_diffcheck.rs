use std::fs::File;
use std::io::{BufReader, BufRead};
use console::Style;
use similar::{ChangeTag, TextDiff};

pub struct DiffChecker {
}

impl DiffChecker {
    pub fn new() -> DiffChecker {
        DiffChecker{
        }
    }

    fn show_diff (&self, source: &Vec<String>, destination: &Vec<String>){
        let source_str = source.join("\n").to_string();
        let destination_str = destination.join("\n").to_string();
        let diff = TextDiff::from_lines(
            &source_str,
            &destination_str
        );
        println!("Source vs Destination");
        for change in diff.iter_all_changes() {
            let (sign, style) = match change.tag() {
                ChangeTag::Delete => ("-", Style::new().red()),
                ChangeTag::Insert => ("+", Style::new().green()),
                ChangeTag::Equal => (" ", Style::new().dim()),
            };
            print!("{}{}", style.apply_to(sign).bold(), style.apply_to(change));
        }
    }

    fn load_file_contents(&self, filename: &str) -> Vec<String> {
        let mut contents = Vec::new();
        // Reade the lines of the file into a vector
        let file = File::open(filename).expect("file should open read only");
        let reader = BufReader::new(file);
        for line in reader.lines() {
            contents.push(line.unwrap());
        }
        return contents;
    }

    pub fn compare_files(&self, source_file: &str, destination_file: &str) {
        // Read the content of the source file
        let source_content = self.load_file_contents(source_file);

        // Read the content of the destination file
        let destination_content = self.load_file_contents(destination_file);

        // Compare the content of the files
        self.show_diff(&source_content, &destination_content);
    }
}
