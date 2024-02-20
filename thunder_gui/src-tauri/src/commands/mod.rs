use tauri::Wry;

mod install_file;
mod list_communities;
mod fetch_community;
use install_file::*;
use list_communities::*;
use fetch_community::*;

pub fn build_commands(builder: tauri::Builder<Wry>) -> tauri::Builder<Wry> {
    builder.invoke_handler(tauri::generate_handler![install_file, list_communities, fetch_community])
}
