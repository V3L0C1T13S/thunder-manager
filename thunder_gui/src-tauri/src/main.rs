// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use commands::build_commands;

mod commands;

fn main() {
    let builder = tauri::Builder::default();
    build_commands(builder)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
