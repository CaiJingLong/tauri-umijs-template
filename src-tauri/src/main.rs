// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod fs;
use fs::*;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            is_dir,
            is_file,
            is_link,
            exists,
            create_dir,
            child_list,
            read_buffer,
            write_buffer,
            append_buffer,
            cp_file,
            cp_dir,
            remove_dir,
            remove_file,
            rename_file,
            open_file,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
