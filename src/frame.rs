use eframe::egui::{self, Color32, Ui};
use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};

use crate::{
    store::{self, Store},
    types::Task,
};

pub struct AppFrame {
    show_deferred_history: Arc<AtomicBool>,
    store: store::Store,
}

impl Default for AppFrame {
    fn default() -> Self {
        let store = Store::default().unwrap();
        Self {
            show_deferred_history: Arc::new(AtomicBool::new(false)),
            store,
        }
    }
}

impl AppFrame {
    pub fn window(
        &self,
        ctx: &egui::Context,
        title: &str,
        add_contents: impl FnOnce(&mut egui::Ui),
    ) {
        use egui::*;

        let panel_frame = egui::Frame {
            fill: ctx.style().visuals.window_fill(),
            rounding: 10.0.into(),
            stroke: ctx.style().visuals.widgets.noninteractive.fg_stroke,
            outer_margin: 0.5.into(), // so the stroke is within the bounds
            ..Default::default()
        };

        CentralPanel::default().frame(panel_frame).show(ctx, |ui| {
            let app_rect = ui.max_rect();

            let title_bar_height = 32.0;
            let title_bar_rect = {
                let mut rect = app_rect;
                rect.max.y = rect.min.y + title_bar_height;
                rect
            };
            AppFrame::title_bar_ui(self, ui, title_bar_rect, title);

            // Add the contents:
            let content_rect = {
                let mut rect = app_rect;
                rect.min.y = title_bar_rect.max.y;
                rect
            }
            .shrink(4.0);
            let mut content_ui = ui.child_ui(content_rect, *ui.layout());
            add_contents(&mut content_ui);
        });

        // open the history viewport
        if self.show_deferred_history.load(Ordering::Relaxed) {
            let show_deferred_viewport = self.show_deferred_history.clone();
            let todo_list: Vec<Task> = self.store.get_all_tasks().unwrap_or_default();
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

    fn title_bar_ui(&self, ui: &mut egui::Ui, title_bar_rect: eframe::epaint::Rect, title: &str) {
        use egui::*;

        let painter = ui.painter();

        let title_bar_response = ui.interact(title_bar_rect, Id::new("title_bar"), Sense::click());

        // Paint the title:
        painter.text(
            title_bar_rect.left_center(),
            Align2::LEFT_CENTER,
            title,
            FontId::proportional(14.0),
            ui.style().visuals.text_color(),
        );

        // Interact with the title bar (drag to move window):
        if title_bar_response.double_clicked() {
            let is_maximized = ui.input(|i| i.viewport().maximized.unwrap_or(false));
            ui.ctx()
                .send_viewport_cmd(ViewportCommand::Maximized(!is_maximized));
        }

        if title_bar_response.is_pointer_button_down_on() {
            ui.ctx().send_viewport_cmd(ViewportCommand::StartDrag);
        }

        ui.allocate_ui_at_rect(title_bar_rect, |ui| {
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.spacing_mut().item_spacing.x = 0.0;
                ui.visuals_mut().button_frame = false;
                ui.add_space(8.0);
                AppFrame::close_menu(self, ui);
            });
        });
    }

    /// Show some close/maximize/minimize buttons for the native window.
    fn close_menu(&self, ui: &mut egui::Ui) {
        use egui::{Button, RichText};

        let button_height = 12.0;

        let close_response = ui
            .add(Button::new(RichText::new("❌").size(button_height)))
            .on_hover_text("Close the window");
        if close_response.clicked() {
            ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
        }

        let mut show_deferred_viewport = self.show_deferred_history.load(Ordering::Relaxed);

        ui.checkbox(&mut show_deferred_viewport, "")
            .on_hover_text("Show history worklog");
        self.show_deferred_history
            .store(show_deferred_viewport, Ordering::Relaxed);
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
    sorted_dates.sort_by(|a, b| b.cmp(a));

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
                            ui.label(format!("{} {}", is_completed, task.description).trim_end());
                        });
                    }
                    ui.separator();
                });
                ui.add_space(12.0);
            }
        }
    });
}
