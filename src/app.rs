use std::time::{SystemTime, UNIX_EPOCH};

use crate::frame::AppFrame;
use crate::store::Store;

use super::store;
use super::Task;

use chrono::{DateTime, Local};
use eframe::egui::{self, Align, Color32, Layout};

pub struct MyApp {
    new_task: String,
    store: store::Store,
    app_frame: AppFrame,
}

impl Default for MyApp {
    fn default() -> Self {
        let store = Store::default().unwrap();
        Self {
            new_task: "".to_owned(),
            store,
            app_frame: AppFrame::default(),
        }
    }
}

impl eframe::App for MyApp {
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        egui::Rgba::TRANSPARENT.to_array() // Make sure we don't paint anything behind the rounded corners
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let now: DateTime<Local> = Local::now();
        let date_string = now.format("%Y-%m-%d").to_string();
        let title = format!("{} {}", "🔆", date_string);

        self.app_frame.window(ctx, title.as_str(), |ui| {
            egui::CentralPanel::default().show_inside(ui, |ui| {
                let faded_color = ui.visuals().window_fill();
                let faded_color = |color: Color32| -> Color32 {
                    use egui::Rgba;
                    let t = { 0.8 };
                    egui::lerp(Rgba::from(color)..=Rgba::from(faded_color), t).into()
                };

                ui.add_space(12.0);

                // input area
                ui.horizontal(|ui| {
                    ui.with_layout(Layout::left_to_right(Align::Min), |ui| {
                        ui.style_mut().visuals.extreme_bg_color =
                            faded_color(Color32::from_white_alpha(9));
                        ui.add(
                            egui::TextEdit::multiline(&mut self.new_task)
                                .frame(true)
                                .hint_text("add new task by press Enter")
                                .desired_width(f32::INFINITY),
                        );
                    });

                    if ui.input(|input| input.key_pressed(egui::Key::Enter)) {
                        let task = Task::new(self.new_task.clone());
                        self.new_task.clear(); // Reset input field

                        // save in the store
                        self.store.add_task(&task).unwrap();
                    }
                });

                ui.add_space(12.0);

                // Display todo list
                egui::ScrollArea::vertical().show(ui, |ui| {
                    // Add a lot of widgets here.

                    let mut tasks = self.store.get_all_tasks().unwrap_or_default();
                    for task in tasks
                        .iter_mut()
                        .filter(|todo| todo.is_today() || !todo.completed)
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

                                        self.store.update_task(task).unwrap();
                                    }
                                    ui.label(description.trim_end());

                                    if ui.add(egui::Button::new("❌").small()).clicked() {
                                        self.store.delete_task_by_id(&task.id).unwrap();
                                    }
                                });
                            });
                        });
                    }
                });
            });
        });
    }
}
