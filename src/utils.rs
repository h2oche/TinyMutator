use std::env::current_dir;
use std::fs::canonicalize;
use std::path::PathBuf;
use std::str;

/**
 * Get absolute path
 */
pub fn get_abs_path(path: &str) -> String {
    // Get Absolute path
    let path = PathBuf::from(path);
    let absolute_path = match canonicalize(path) {
        Ok(v) => v,
        Err(_) => panic!("Failed to Get Absolute Path"),
    };
    match absolute_path.to_str() {
        Some(x) => x.to_owned(),
        None => panic!("Failed to Get Absolute Path String"),
    }
}

/**
 * Get current working directory
 */
pub fn get_cwd() -> String {
    let current_working_dir = match current_dir() {
        Ok(v) => v,
        Err(_) => panic!("Failed to Get Current Working Directory"),
    };
    match current_working_dir.to_str() {
        Some(v) => v.to_owned(),
        None => panic!("Failed to Get Current Working Directory Path String"),
    }
}
