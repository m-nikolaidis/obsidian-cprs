pub struct FileNameComp {
}

impl FileNameComp {
    pub fn new() -> FileNameComp {
        FileNameComp {
        }
    }

    pub fn find_unique_files(&self, files1: &[String], files2: &[String]) -> Vec<String> {
        let mut unique_files = Vec::new();
        for file in files1 {
            if !files2.contains(file) {
                unique_files.push(file.clone());
            }
        }
        unique_files
    }

    pub fn find_common_files(&self, files1: &[String], files2: &[String]) -> Vec<String> {
        let mut common_files = Vec::new();
        for file in files1 {
            if files2.contains(file) {
                common_files.push(file.clone());
            }
        }
        common_files
    }
}
