#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;

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

struct Task {
    description: String,
    completed: bool,
}

struct MyApp {
    new_task: String,
    todo_list: Vec<Task>,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            new_task: "".to_owned(),
            todo_list: vec![],
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My WorkLog");

            ui.add_space(12.0);

            ui.horizontal(|ui| {
                ui.add(egui::TextEdit::singleline(&mut self.new_task).desired_width(f32::INFINITY));

                if ui.input(|input| input.key_pressed(egui::Key::Enter)) {
                    self.todo_list.push(Task {
                        description: self.new_task.clone(),
                        completed: false,
                    });
                    self.new_task.clear(); // Reset input field
                }
            });

            ui.add_space(12.0);

            // Display todo list
            let mut tasks_to_remove = vec![];

            for (index, task) in self.todo_list.iter_mut().enumerate() {
                ui.horizontal(|ui| {
                    let description = task.description.clone();
                    let completed = &mut task.completed;

                    ui.checkbox(completed, "");
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
        });
    }
}
