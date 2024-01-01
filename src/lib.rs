use std::fs::read_to_string;
use std::path::{Path, PathBuf};

fn get_resource_path(resource: &str) -> Option<PathBuf> {
    let filename = std::env::current_exe()
        .expect("Can't find path to executable");
    // Directory containing binary
    let Some(item) = filename.ancestors().find(|&x| x.ends_with(Path::new("advent-of-code-2023"))) else {
        println!("Ancestor not found");
        return None;
    };
    // Subdir relative to binary
    let mut path_buf = item.to_path_buf();
    path_buf.push(format!("resource/{resource}"));
    Some(path_buf)
}

pub fn read_resource_lines(filename: &str) -> Vec<String> {
    if let Some(path_buf) = get_resource_path(filename) {
        let mut result = Vec::new();

        for line in read_to_string(path_buf).unwrap().lines() {
            result.push(line.to_string())
        }

        result
    } else {
        panic!("Error reading resource file")
    }
}