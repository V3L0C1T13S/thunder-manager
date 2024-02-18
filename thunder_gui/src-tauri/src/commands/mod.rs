use tauri::Wry;

mod install_file;
use install_file::*;

pub fn build_commands(builder: tauri::Builder<Wry>) -> tauri::Builder<Wry> {
    builder.invoke_handler(tauri::generate_handler![install_file])
}
