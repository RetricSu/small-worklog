#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod app;
mod store;
mod types;
mod version;

use app::MyApp;
use eframe::egui::{self};
use types::Task;
use version::read_version_from_toml;

fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([520.0, 340.0]),
        ..Default::default()
    };
    let version = read_version_from_toml().unwrap_or_else(|| "v-unknown".to_string());
    eframe::run_native(
        format!("Small Worklog v{}", version).as_str(),
        options,
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);

            Box::<MyApp>::default()
        }),
    )
}
