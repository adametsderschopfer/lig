#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod features;
mod infrastructure;
mod shared;

fn main() -> std::process::ExitCode {
    infrastructure::graphics::run(app::run)
}
