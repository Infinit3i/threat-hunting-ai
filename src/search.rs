use std::fs;
use std::path::{Path};

const KNOWLEDGE_DIR: &str = "knowledge";

/// Search for files containing a specific keyword in their content.
pub fn search_files(keyword: &str) -> Vec<String> {
    let mut results = Vec::new();

    fn collect_files(dir: &Path, keyword: &str, results: &mut Vec<String>) {
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    collect_files(&path, keyword, results); // Recursive call for subdirectories
                } else if path.is_file() && path.extension().unwrap_or_default() == "md" {
                    if let Ok(content) = fs::read_to_string(&path) {
                        if content.contains(keyword) {
                            if let Some(file_name) = path.strip_prefix(KNOWLEDGE_DIR).ok() {
                                results.push(file_name.display().to_string());
                            }
                        }
                    }
                }
            }
        }
    }

    collect_files(Path::new(KNOWLEDGE_DIR), keyword, &mut results);
    results
}

/// Search for files with names containing a specific keyword.
pub fn search_filenames(keyword: &str) -> Vec<String> {
    let mut results = Vec::new();

    if let Ok(entries) = fs::read_dir(KNOWLEDGE_DIR) {
        for entry in entries.filter_map(|e| e.ok()) {
            if let Some(file_name) = entry.file_name().to_str() {
                if file_name.contains(keyword) && file_name.ends_with(".md") {
                    results.push(file_name.to_string());
                }
            }
        }
    }

    results
}
