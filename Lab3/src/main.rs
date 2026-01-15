// Hide console window on Windows in release mode
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::config::Config;
use crate::logs::Logger;
use crate::ui::Ui;

const PROJECT_TITLE: &str = "Лабораторна робота №3";

fn main() {
    let config = Config::from_file().unwrap_or_else(|error| {
        Ui::native_panic_message(error);
        std::process::exit(1);
    });

    Logger::from_config(&config)
        .setup()
        .unwrap_or_else(|error| {
            Ui::native_panic_message(error);
            std::process::exit(1);
        });

    Ui::default().start(config).unwrap_or_else(|error| {
        Ui::native_panic_message(error);
        std::process::exit(1);
    });
}

pub mod config;
pub mod context;
pub mod errors;
pub mod io;
pub mod logs;
pub mod ui;
pub mod utils;
