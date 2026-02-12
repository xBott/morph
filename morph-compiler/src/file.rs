use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

pub struct FileWrapper {
    pub path: PathBuf,
    pub file: File,
}

impl FileWrapper {

    pub fn read_to_string(&mut self) -> std::io::Result<String> {
        let mut content = String::new();
        self.file.read_to_string(&mut content).map_err(|e| {
            std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Could not read file {}: {}", self.path.to_string_lossy(), e),
            )
        })?;
        Ok(content)
    }
}