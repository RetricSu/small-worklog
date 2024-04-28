use std::collections::HashMap;
use std::ops::Sub;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

use super::store;
use super::Task;

use chrono::{DateTime, Local};
use eframe::egui::{self, Align, Color32, Layout, Ui};

pub struct MyApp {
    new_task: String,
    todo_list: Vec<Task>,
    show_deferred_history: Arc<AtomicBool>,
}

impl Default for MyApp {
    fn default() -> Self {
        let tasks = store::load_tasks().unwrap_or_default();
        Self {
            new_task: "".to_owned(),
            todo_list: tasks,
            show_deferred_history: Arc::new(AtomicBool::new(false)),
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

            // header with title in the left and menu in the right
            ui.horizontal(|ui| {
                ui.columns(2, |columns| {
                    columns[0].heading(&format!("{} {}", "🔆", date_string));

                    columns[1].horizontal(|ui| {
                        // Add a flexible space to push the next item to the rightmost side
                        ui.add_space(ui.available_width().sub(100.));

                        let mut show_deferred_viewport =
                            self.show_deferred_history.load(Ordering::Relaxed);
                        ui.checkbox(&mut show_deferred_viewport, "Show history");
                        self.show_deferred_history
                            .store(show_deferred_viewport, Ordering::Relaxed);
                    });
                });
            });

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

        // open the history viewport
        if self.show_deferred_history.load(Ordering::Relaxed) {
            let show_deferred_viewport = self.show_deferred_history.clone();
            let todo_list: Vec<Task> = self.todo_list.clone();
            ctx.show_viewport_deferred(
                egui::ViewportId::from_hash_of("deferred_history_viewport"),
                egui::ViewportBuilder::default()
                    .with_title("Worklog History")
                    .with_inner_size([400.0, 500.0]),
                move |ctx, class| {
                    assert!(
                        class == egui::ViewportClass::Deferred,
                        "This egui backend doesn't support multiple viewports"
                    );
                    if ctx.input(|i| i.viewport().close_requested()) {
                        // Tell parent to close us.
                        show_deferred_viewport.store(false, Ordering::Relaxed);
                    }

                    // show history
                    egui::CentralPanel::default().show(ctx, |ui| {
                        egui::ScrollArea::vertical().show(ui, |ui| {
                            ui_history(ui, &todo_list);
                        });
                    });
                },
            );
        }
    }

    fn save(&mut self, _storage: &mut dyn eframe::Storage) {
        store::store_tasks(&self.todo_list).unwrap_or_else(|err| {
            eprintln!("Failed to store tasks: {}", err);
        });
    }
}

fn ui_history(ui: &mut Ui, tasks: &[Task]) {
    // Group tasks by created_at_date
    let mut tasks_by_date: HashMap<String, Vec<&Task>> = HashMap::new();
    for task in tasks {
        let date = task.created_at_date();
        tasks_by_date.entry(date).or_default().push(task);
    }

    // Sort dates
    let mut sorted_dates: Vec<_> = tasks_by_date.keys().collect();
    sorted_dates.sort();

    // Begin the UI layout
    ui.vertical_centered(|ui| {
        // Iterate through each date with tasks
        for dates in &sorted_dates {
            if let Some(tasks) = tasks_by_date.get(*dates) {
                // Add a header for the date
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new(*dates).color(Color32::DARK_GREEN));
                });

                // Begin a table for tasks
                ui.vertical(|ui| {
                    // Add a row for each task
                    for task in tasks.iter() {
                        ui.horizontal(|ui| {
                            let is_completed = if task.completed {
                                "\u{2714}"
                            } else {
                                "\u{2795}"
                            };
                            ui.label(format!("{} {}", is_completed, task.description));
                        });
                    }
                    ui.separator();
                });
                ui.add_space(12.0);
            }
        }
    });
}
