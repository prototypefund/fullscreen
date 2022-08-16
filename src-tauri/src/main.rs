#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
mod menu;

use menu::MenuBuilder;

fn main() {
    let context = tauri::generate_context!();

    let menu = MenuBuilder::new()
        .add_application_menu(&context.package_info().name)
        .add_file_menu()
        .add_edit_menu()
        .add_view_menu()
        .add_window_menu()
        .build();

    tauri::Builder::default()
        .menu(menu)
        .run(context)
        .expect("error while running tauri application");
}
