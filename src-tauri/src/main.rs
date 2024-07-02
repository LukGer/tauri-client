// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{CustomMenuItem, Manager, Menu, MenuItem, Submenu};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn get_username() -> String {
    whoami::username()
}

#[tauri::command]
fn get_devicename() -> String {
    whoami::devicename()
}

#[tauri::command]
fn get_realname() -> String {
    whoami::realname()
}

fn main() {
    let stamp_in = CustomMenuItem::new("stamp_in".to_string(), "Einstempeln");

    let stamp_out = CustomMenuItem::new("stamp_out".to_string(), "Ausstempeln").disabled();

    let stamp_menu = Submenu::new(
        "Stempeln",
        Menu::new().add_item(stamp_in).add_item(stamp_out),
    );

    let menu = Menu::new()
        .add_native_item(MenuItem::Separator)
        .add_submenu(stamp_menu);

    tauri::Builder::default()
        .menu(menu)
        .on_menu_event(|event| {
            let app = event.window().app_handle();

            match event.menu_item_id() {
                "stamp_in" => {
                    app.emit_all("stamp_in", {}).unwrap();
                    app.get_window("main")
                        .unwrap()
                        .menu_handle()
                        .get_item("stamp_in")
                        .set_enabled(false);
                    app.get_window("main")
                        .unwrap()
                        .menu_handle()
                        .get_item("stamp_out")
                        .set_enabled(true);
                }
                "stamp_out" => {
                    app.emit_all("stamp_out", {}).unwrap();
                    app.get_window("main")
                        .unwrap()
                        .menu_handle()
                        .get_item("stamp_in")
                        .set_enabled(true);
                    app.get_window("main")
                        .unwrap()
                        .menu_handle()
                        .get_item("stamp_out")
                        .set_enabled(false);
                }
                _ => {}
            }
        })
        .invoke_handler(tauri::generate_handler![
            get_username,
            get_devicename,
            get_realname
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
