pub mod movefns {
    use std::{
        fs,
        path::{Path, PathBuf},
    };

    pub fn move_folder(
        source_path: &str,
        target_path: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // get affected files list
        let source_path = Path::new(&source_path);
        let is_dir = source_path.is_dir();
        let mut files = vec![];
        if is_dir {
            let read_dir = fs::read_dir(&source_path)?;
            for file in read_dir {
                if !file.is_err() {
                    files.push(file.unwrap().path())
                }
            }
        } else if source_path.is_file() {
            files.push(PathBuf::from(&source_path))
        }

        // move files
        for file in files {
            let path = Path::new(file.as_path());

            let mut target = String::from(target_path.clone());
            if !target_path.ends_with("/") {
                target += "/";
            }
            target += path.file_name().unwrap().to_str().unwrap();

            fs::rename(path, Path::new(target.as_str()))?;

            ()
        }

        // find affected tracks in db

        // update mixxx database
        Ok(())
    }
}
