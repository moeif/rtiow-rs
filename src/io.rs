use std::fs::File;
use std::io::prelude::*;

pub fn write_to_file(file_path: &str, content: &str) -> bool {
    if let Ok(mut file) = File::create(file_path) {
        if let Ok(_) = file.write_all(content.as_bytes()) {
            return true;
        }
    }
    return false;
}
