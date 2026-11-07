#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod features;
mod infrastructure;
mod shared;

fn main() -> std::process::ExitCode {
    let language = infrastructure::locale::detect();
    let i18n = std::rc::Rc::new(shared::i18n::I18n::new(language));
    infrastructure::graphics::run(&i18n, || app::run(std::rc::Rc::clone(&i18n)))
}
