use tauri::{SystemTray, SystemTrayMenu, SystemTrayEvent, CustomMenuItem, SystemTrayMenuItem, GlobalShortcutManager};
use tauri::Manager;
use std::sync::Mutex;

struct LastPosition(Mutex<Option<(f64, f64)>>);

fn save_and_hide(window: &tauri::Window, state: &LastPosition) {
    if let Ok(pos) = window.outer_position() {
        *state.0.lock().unwrap() = Some((pos.x as f64, pos.y as f64));
    }
    let _ = window.hide();
}

fn restore_and_show(window: &tauri::Window, state: &LastPosition) {
    let pos = state.0.lock().unwrap().clone();
    match pos {
        Some((x, y)) => {
            let _ = window.set_position(tauri::Position::Physical(tauri::PhysicalPosition {
                x: x as i32,
                y: y as i32,
            }));
        }
        None => {
            let _ = window.center();
        }
    }
    let _ = window.show();
    let _ = window.set_focus();
}

#[tauri::command]
fn hide_window(window: tauri::Window, state: tauri::State<LastPosition>) {
    save_and_hide(&window, &state);
}

fn main() {
    let quit = CustomMenuItem::new("quit".to_string(), "關閉程式");
    let hide = CustomMenuItem::new("hide".to_string(), "隱藏視窗");
    let tray_menu = SystemTrayMenu::new()
        .add_item(quit)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(hide);

    tauri::Builder::default()
        .manage(LastPosition(Mutex::new(None)))
        .system_tray(SystemTray::new().with_menu(tray_menu))
        .on_system_tray_event(|app, event|
            {
                match event {
                SystemTrayEvent::LeftClick {
                    ..
                } => {
                    let window = app.get_window("main").unwrap();
                    let state = app.state::<LastPosition>();
                    match window.is_visible() {
                        Ok(visible) => {
                            if visible {
                                save_and_hide(&window, &state);
                                return
                            }
                            restore_and_show(&window, &state);
                        },
                        Err(e) => eprintln!("Error checking window visibility: {:?}", e),
                    }
                }
                SystemTrayEvent::MenuItemClick { id, .. } => {
                    match id.as_str() {
                      "quit" => {
                        std::process::exit(0);
                      }
                      "hide" => {
                        let window = app.get_window("main").unwrap();
                        let state = app.state::<LastPosition>();
                        save_and_hide(&window, &state);
                      }
                      _ => {}
                    }
                  }
                _ => {}
            }
        })
        .plugin(tauri_plugin_positioner::init())
        .setup(|app| {
            let window = app.get_window("main").unwrap();
            let app_handle = app.handle();
            app.global_shortcut_manager()
                .register("CmdOrCtrl+Shift+V", move || {
                    let state = app_handle.state::<LastPosition>();
                    if let Ok(visible) = window.is_visible() {
                        if visible {
                            save_and_hide(&window, &state);
                        } else {
                            restore_and_show(&window, &state);
                        }
                    }
                })
                .expect("failed to register global shortcut");
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![hide_window])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
