use eframe::egui;
use serde::{Deserialize, Serialize};
use std::fs;

const FILE_PATH: &str = "todo_list.json";

#[derive(Serialize, Deserialize, Clone)]
struct Task {
    description: String,
    done: bool,
}

#[derive(Serialize, Deserialize)]
struct TodoList {
    tasks: Vec<Task>,
}

impl TodoList {
    fn load() -> Self {
        match fs::read_to_string(FILE_PATH) {
            Ok(content) => serde_json::from_str(&content).unwrap_or_else(|_| Self { tasks: vec![] }),
            Err(_) => Self { tasks: vec![] },
        }
    }

    fn save(&self) {
        let content = serde_json::to_string_pretty(self).expect("Failed to serialize tasks");
        fs::write(FILE_PATH, content).expect("Failed to save tasks");
    }

    fn add_task(&mut self, description: String) {
        self.tasks.push(Task {
            description,
            done: false,
        });
        self.save();
    }

    fn toggle_done(&mut self, index: usize) {
        if let Some(task) = self.tasks.get_mut(index) {
            task.done = !task.done;
            self.save();
        }
    }

    fn remove_task(&mut self, index: usize) {
        if index < self.tasks.len() {
            self.tasks.remove(index);
            self.save();
        }
    }
}

pub struct TodoApp {
    todo_list: TodoList,
    new_task: String,
}

impl TodoApp {
    fn new() -> Self {
        Self {
            todo_list: TodoList::load(),
            new_task: String::new(),
        }
    }
}

impl eframe::App for TodoApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("To-Do List");

            // Input for new task
            ui.horizontal(|ui| {
                ui.text_edit_singleline(&mut self.new_task);
                if ui.button("Add").clicked() && !self.new_task.is_empty() {
                    self.todo_list.add_task(self.new_task.clone());
                    self.new_task.clear();
                }
            });

            ui.separator();

            // Render tasks
            let tasks: Vec<(usize, Task)> = self
                .todo_list
                .tasks
                .iter()
                .enumerate()
                .map(|(i, task)| (i, task.clone()))
                .collect();

            let mut tasks_to_remove = vec![];
            let mut tasks_to_toggle = vec![];

            for (i, task) in tasks {
                ui.horizontal(|ui| {
                    let mut done = task.done;
                    if ui.checkbox(&mut done, "").clicked() {
                        tasks_to_toggle.push(i);
                    }
                    ui.label(if task.done {
                        format!("✔ {}", task.description)
                    } else {
                        task.description.clone()
                    });
                    if ui.button("❌").clicked() {
                        tasks_to_remove.push(i);
                    }
                });
            }

            // Apply changes
            for &i in tasks_to_toggle.iter() {
                self.todo_list.toggle_done(i);
            }
            for &i in tasks_to_remove.iter().rev() {
                self.todo_list.remove_task(i);
            }
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "To-Do App",
        options,
        Box::new(|_cc| Box::new(TodoApp::new())),
    )
}
