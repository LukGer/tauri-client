// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{
    App, AppHandle, CustomMenuItem, GlobalWindowEvent, Manager, Menu, MenuEvent, MenuItem, Submenu,
    SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem, WindowEvent, WindowMenuEvent,
};

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
    let menu = configure_menu();

    let tray = configure_system_tray();

    tauri::Builder::default()
        .menu(menu)
        .system_tray(tray)
        .on_menu_event(on_menu_event)
        .on_window_event(on_window_event)
        .on_system_tray_event(on_system_tray_event)
        .invoke_handler(tauri::generate_handler![
            get_username,
            get_devicename,
            get_realname
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn on_window_event(event: GlobalWindowEvent) {
    match event.event() {
        tauri::WindowEvent::CloseRequested { api, .. } => {
            event.window().hide().unwrap();
            api.prevent_close();
        }
        _ => {}
    }
}

fn configure_menu() -> Menu {
    let stamp_in = CustomMenuItem::new("stamp_in".to_string(), "Einstempeln");

    let stamp_out = CustomMenuItem::new("stamp_out".to_string(), "Ausstempeln").disabled();

    let stamp_menu = Submenu::new(
        "Stempeln",
        Menu::new().add_item(stamp_in).add_item(stamp_out),
    );

    let menu = Menu::new()
        .add_native_item(MenuItem::Separator)
        .add_submenu(stamp_menu);

    menu
}

fn on_menu_event(event: WindowMenuEvent) {
    let app = event.window().app_handle();

    let menu_handle = app.get_window("main").unwrap().menu_handle();

    match event.menu_item_id() {
        "stamp_in" => {
            app.emit_all("stamp_in", {}).unwrap();

            menu_handle.get_item("stamp_in").set_enabled(false).unwrap();

            menu_handle.get_item("stamp_out").set_enabled(true).unwrap();
        }
        "stamp_out" => {
            app.emit_all("stamp_out", {}).unwrap();
            menu_handle.get_item("stamp_in").set_enabled(true).unwrap();

            menu_handle
                .get_item("stamp_out")
                .set_enabled(false)
                .unwrap();
        }
        _ => {}
    }
}

fn configure_system_tray() -> SystemTray {
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let hide = CustomMenuItem::new("hide".to_string(), "Hide");
    let show = CustomMenuItem::new("show".to_string(), "Show");
    let tray_menu = SystemTrayMenu::new()
        .add_item(show)
        .add_item(hide)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(quit);

    let tray = SystemTray::new().with_menu(tray_menu);

    tray
}

fn on_system_tray_event<'a>(app: &'a AppHandle, event: SystemTrayEvent) {
    match event {
        SystemTrayEvent::LeftClick {
            position: _,
            size: _,
            ..
        } => {
            println!("system tray received a left click");
        }
        SystemTrayEvent::RightClick {
            position: _,
            size: _,
            ..
        } => {
            println!("system tray received a right click");
        }
        SystemTrayEvent::DoubleClick {
            position: _,
            size: _,
            ..
        } => {
            println!("system tray received a double click");
        }
        SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
            "quit" => {
                std::process::exit(0);
            }
            "hide" => {
                let window = app.get_window("main").unwrap();
                window.hide().unwrap();
            }
            "show" => {
                let window = app.get_window("main").unwrap();
                window.show().unwrap();
            }
            _ => {}
        },
        _ => {}
    }
}
