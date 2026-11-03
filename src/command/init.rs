use std::fs::File;
use std::io::ErrorKind;
use std::io::Write;
use std::path::Path;

use crate::config::CONFIG_FILE;
use crate::config::DEFAULT_CONFIG_FILE;

pub fn init() -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new(CONFIG_FILE);

    let mut file = match File::create_new(path) {
        Ok(file) => file,
        Err(e) if e.kind() == ErrorKind::AlreadyExists => {
            return Err(format!("{} already exists", path.display()).into());
        }
        Err(e) => {
            return Err(format!("{} could not be created: {e}", path.display()).into());
        }
    };

    file.write_all(DEFAULT_CONFIG_FILE.as_bytes())
        .map_err(|e| format!("{} could not be written: {e}", path.display()))?;

    println!("successfully created {}", path.display());
    Ok(())
}
