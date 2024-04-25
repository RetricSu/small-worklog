#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::time::{SystemTime, UNIX_EPOCH};

use chrono::{DateTime, Local};
use eframe::egui;
use types::Task;

mod store;
mod types;

fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([520.0, 340.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Worklog",
        options,
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);

            Box::<MyApp>::default()
        }),
    )
}

struct MyApp {
    new_task: String,
    todo_list: Vec<Task>,
}

impl Default for MyApp {
    fn default() -> Self {
        let tasks = store::load_tasks().unwrap_or_default();
        Self {
            new_task: "".to_owned(),
            todo_list: tasks,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let now: DateTime<Local> = Local::now();
            let date_string = now.format("%Y-%m-%d").to_string();

            ui.heading(&format!("{} {}", "🔆", date_string));

            ui.add_space(12.0);

            ui.horizontal(|ui| {
                ui.add(egui::TextEdit::singleline(&mut self.new_task).desired_width(f32::INFINITY));

                if ui.input(|input| input.key_pressed(egui::Key::Enter)) {
                    self.todo_list.push(Task::new(self.new_task.clone()));
                    self.new_task.clear(); // Reset input field

                    // save in the store
                    store::store_tasks(&self.todo_list).unwrap_or_else(|err| {
                        eprintln!("Failed to store tasks: {}", err);
                    });
                }
            });

            ui.add_space(12.0);

            // Display todo list
            let mut tasks_to_remove = vec![];

            for (index, task) in self
                .todo_list
                .iter_mut()
                .filter(|todo| todo.is_today() || !todo.completed)
                .enumerate()
            {
                ui.horizontal(|ui| {
                    let description = task.description.clone();
                    let mut completed = task.completed;

                    if ui.checkbox(&mut completed, "").clicked() {
                        task.completed = completed;

                        // update the completed_at
                        if completed {
                            let now = SystemTime::now();
                            let completed_at = now
                                .duration_since(UNIX_EPOCH)
                                .expect("Time went backwards")
                                .as_secs();
                            task.completed_at = completed_at;
                        } else {
                            task.completed_at = 0;
                        }
                    }
                    ui.label(description);
                    if ui.button("Delete").clicked() {
                        tasks_to_remove.push(index);
                    }
                });
            }

            // Remove tasks outside the loop
            for &index in tasks_to_remove.iter().rev() {
                self.todo_list.remove(index);
            }

            tasks_to_remove.clear();

            store::store_tasks(&self.todo_list).unwrap_or_else(|err| {
                eprintln!("Failed to store tasks: {}", err);
            });
        });
    }

    fn save(&mut self, _storage: &mut dyn eframe::Storage) {
        store::store_tasks(&self.todo_list).unwrap_or_else(|err| {
            eprintln!("Failed to store tasks: {}", err);
        });
    }
}
