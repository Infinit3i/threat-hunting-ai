mod search;

use eframe::egui;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use pulldown_cmark::Parser;

const KNOWLEDGE_DIR: &str = "knowledge";

pub struct KnowledgeApp {
    subdirectories: HashMap<String, Vec<String>>,
    active_subdir: Option<String>,
    selected_file: Option<String>,
    file_content: String,
    search_query: String,
    search_results: Vec<String>,
}

impl KnowledgeApp {
    fn new() -> Self {
        if !Path::new(KNOWLEDGE_DIR).exists() {
            fs::create_dir(KNOWLEDGE_DIR).expect("Failed to create knowledge directory");
        }

        Self {
            subdirectories: KnowledgeApp::scan_subdirectories(),
            active_subdir: None,
            selected_file: None,
            file_content: String::new(),
            search_query: String::new(),
            search_results: Vec::new(),
        }
    }

    /// Scan the `knowledge` directory and group files by subdirectory.
    fn scan_subdirectories() -> HashMap<String, Vec<String>> {
        let mut subdirs = HashMap::new();

        fn collect_files(dir: &Path, subdirs: &mut HashMap<String, Vec<String>>) {
            if let Ok(entries) = fs::read_dir(dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_dir() {
                        collect_files(&path, subdirs);
                    } else if path.is_file() {
                        let subdir = path.parent().unwrap_or(dir).strip_prefix(KNOWLEDGE_DIR).unwrap();
                        let subdir_name = subdir.to_str().unwrap_or("Unknown").to_string();
                        subdirs
                            .entry(subdir_name)
                            .or_insert_with(Vec::new)
                            .push(path.file_name().unwrap().to_str().unwrap().to_string());
                    }
                }
            }
        }

        collect_files(Path::new(KNOWLEDGE_DIR), &mut subdirs);
        subdirs
    }

    /// Load the content of a file.
    fn load_file(&mut self, file_name: &str) {
        let file_path = if let Some(active_subdir) = &self.active_subdir {
            format!("{}/{}/{}", KNOWLEDGE_DIR, active_subdir, file_name)
        } else {
            format!("{}/{}", KNOWLEDGE_DIR, file_name)
        };

        match fs::read_to_string(file_path) {
            Ok(content) => {
                self.file_content = content;
                self.selected_file = Some(file_name.to_string());
            }
            Err(err) => {
                eprintln!("Error reading file: {}", err);
                self.file_content.clear();
            }
        }
    }

    /// Perform the search and update the results.
    fn perform_search(&mut self) {
        if self.search_query.is_empty() {
            self.search_results.clear();
            return;
        }

        // Combine filename and content search results
        let mut results = search::search_filenames(&self.search_query);
        results.extend(search::search_files(&self.search_query));
        results.sort();
        results.dedup(); // Remove duplicates
        self.search_results = results;
    }
}

impl eframe::App for KnowledgeApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Top navigation bar
        egui::TopBottomPanel::top("Top Nav").show(ctx, |ui| {
            ui.horizontal(|ui| {
                for subdir in self.subdirectories.keys() {
                    if ui.button(subdir).clicked() {
                        self.active_subdir = Some(subdir.clone());
                        self.selected_file = None;
                        self.search_query.clear();
                        self.search_results.clear();
                    }
                }
                ui.separator();

                // Search Bar
                ui.horizontal(|ui| {
                    ui.label("Search:");
                    if ui.text_edit_singleline(&mut self.search_query).changed() {
                        self.perform_search();
                    }
                    if ui.button("Clear").clicked() {
                        self.search_query.clear();
                        self.search_results.clear();
                    }
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            if !self.search_results.is_empty() {
                // Display search results
                ui.heading("Search Results:");
                ui.separator();

                let results = self.search_results.clone(); // Clone the results to avoid borrow conflicts
                for result in results {
                    if ui.button(&result).clicked() {
                        self.load_file(&result);
                        self.search_results.clear(); // Clear after using cloned results
                        break; // Exit the loop after clearing to avoid using invalidated references
                    }
                }
            } else if let Some(selected) = self.selected_file.clone() {
                // Display the selected file content
                ui.horizontal(|ui| {
                    ui.heading(format!("Viewing: {}", selected));
                    if ui.button("Back").clicked() {
                        self.selected_file = None;
                    }
                });
                ui.separator();

                // Parse Markdown and render content
                let parser = Parser::new(&self.file_content);
                egui::ScrollArea::vertical().show(ui, |ui| {
                    for event in parser {
                        match event {
                            pulldown_cmark::Event::Start(tag) => match tag {
                                pulldown_cmark::Tag::Heading(level, _, _) => {
                                    let text = format!("{}\n", "#".repeat(level as usize));
                                    ui.label(egui::RichText::new(text).heading());
                                }
                                pulldown_cmark::Tag::Paragraph => {
                                    ui.separator();
                                }
                                _ => {}
                            },
                            pulldown_cmark::Event::Text(text) => {
                                ui.label(egui::RichText::new(text.to_string()).text_style(egui::TextStyle::Body));
                            }
                            pulldown_cmark::Event::SoftBreak | pulldown_cmark::Event::HardBreak => {
                                ui.add_space(5.0); // Add vertical spacing for line breaks
                            }
                            pulldown_cmark::Event::End(_) => {
                                ui.add_space(10.0); // Add spacing between blocks
                            }
                            _ => {}
                        }
                    }
                });
            } else if let Some(active_subdir) = &self.active_subdir {
                // Display the files in the active subdirectory
                ui.heading(format!("Files in {}", active_subdir));
                ui.separator();

                if let Some(files) = self.subdirectories.get(active_subdir).cloned() {
                    for file in files {
                        if ui.button(&file).clicked() {
                            self.load_file(&file);
                        }
                    }
                }
            } else {
                // Default message
                ui.heading("Select a subdirectory or perform a search.");
            }
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Knowledge App",
        options,
        Box::new(|_cc| Box::new(KnowledgeApp::new())),
    )
}
