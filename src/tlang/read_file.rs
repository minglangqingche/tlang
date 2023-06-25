use std::fs;

pub fn read_file(path: &str) -> String {
    match fs::read_to_string(&path) {
        Ok(val) => {
            val
        },
        Err(_) => {
            crate::interpreter_error::error_exit(&format!("can't read file ' {} '!", path), 64);
            "".to_string()
        }
    }
}
