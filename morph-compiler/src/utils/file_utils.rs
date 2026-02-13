use std::fs;
use std::fs::File;
use std::path::PathBuf;

pub struct FileWrapper {
    pub path: PathBuf,
    pub file: File,
}

pub fn create_file_if_not_exists(path: &PathBuf) -> std::io::Result<bool> {

    match fs::exists(path) {
        Ok(exists) => {

            if exists {
                return Ok(false);
            }

            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent)?;
            }

            match File::create(path) {
                Ok(_) => Ok(true),
                Err(err) => Err(err)
            }

        },
        Err(err) => return Err(err)
    }

}