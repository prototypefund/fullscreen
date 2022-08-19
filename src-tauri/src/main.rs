#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::{CustomMenuItem, Menu, Submenu, WindowMenuEvent};

fn create_application_menu() -> Menu {

    let top_level = Submenu::new(
        "Fullscreen",
        Menu::new()
            .add_item(CustomMenuItem::new("preferences".to_string(), "Preferences").into())
            .add_item(CustomMenuItem::new("about".to_string(), "About").into())
            .add_item(CustomMenuItem::new("quit".to_string(), "Quit").into())
    );
    let file_menu= Submenu::new(
        "File",
        Menu::new()
            .add_item(CustomMenuItem::new("open".to_string(), "Open...").into())
            .add_item(CustomMenuItem::new("save".to_string(), "Save as...").into())
            .add_item(CustomMenuItem::new("link".to_string(), "Share link...").into())
    );

    Menu::new()
        .add_submenu(top_level)
        .add_submenu(file_menu)
}

fn handle_menu_event(event: WindowMenuEvent) {
    match event.menu_item_id() {
        "quit" => {
            std::process::exit(0);
        }
        _ => {}
    }
}

fn main() {
    tauri::Builder::default()
        .menu(create_application_menu())
        .on_menu_event(handle_menu_event)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
