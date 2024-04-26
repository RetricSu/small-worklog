#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::time::{SystemTime, UNIX_EPOCH};

use chrono::{DateTime, Local};
use eframe::egui::{self, Align, Color32, Layout};
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
        egui::TopBottomPanel::top("top-panel").show(ctx, |ui| {
            let faded_color = ui.visuals().window_fill();
            let faded_color = |color: Color32| -> Color32 {
                use egui::Rgba;
                let t = { 0.8 };
                egui::lerp(Rgba::from(color)..=Rgba::from(faded_color), t).into()
            };

            let now: DateTime<Local> = Local::now();
            let date_string = now.format("%Y-%m-%d").to_string();

            ui.heading(&format!("{} {}", "🔆", date_string));

            ui.add_space(12.0);

            // input area
            ui.horizontal(|ui| {
                ui.with_layout(Layout::left_to_right(Align::Min), |ui| {
                    ui.style_mut().visuals.extreme_bg_color =
                        faded_color(Color32::from_white_alpha(9));
                    ui.add(
                        egui::TextEdit::multiline(&mut self.new_task)
                            .frame(true)
                            .desired_width(f32::INFINITY),
                    );
                });

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
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            let faded_color = ui.visuals().window_fill();
            let faded_color = |color: Color32| -> Color32 {
                use egui::Rgba;
                let t = { 0.8 };
                egui::lerp(Rgba::from(color)..=Rgba::from(faded_color), t).into()
            };
            // Display todo list
            let mut tasks_to_remove = vec![];

            egui::ScrollArea::vertical().show(ui, |ui| {
                // Add a lot of widgets here.

                for (index, task) in self
                    .todo_list
                    .iter_mut()
                    .filter(|todo| todo.is_today() || !todo.completed)
                    .enumerate()
                {
                    ui.horizontal(|ui| {
                        let description = task.description.clone();
                        let mut completed = task.completed;

                        ui.columns(1, |cols| {
                            cols[0].horizontal_centered(|ui| {
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

                                if ui
                                    .add(
                                        egui::Button::new("Delete")
                                            .small()
                                            .fill(faded_color(Color32::RED)),
                                    )
                                    .clicked()
                                {
                                    tasks_to_remove.push(index);
                                }
                            });
                        });
                    });
                }
            });

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
