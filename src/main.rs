#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod app;
mod frame;
mod migrate;
mod store;
mod types;
mod version;

use app::MyApp;
use eframe::egui::{self};
use types::Task;
use version::read_version_from_toml;

fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    migrate::try_migrate_v1();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_decorations(false) // Hide the OS-specific "chrome" around the window
            .with_inner_size([520.0, 340.0])
            .with_min_inner_size([520.0, 340.0])
            .with_transparent(true), // To have rounded corners we need transparency,
        ..Default::default()
    };
    eframe::run_native(
        format!("Small Worklog v{}", read_version_from_toml()).as_str(),
        options,
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);

            Box::<MyApp>::default()
        }),
    )
}
