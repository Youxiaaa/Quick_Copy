use tauri::{SystemTray, SystemTrayMenu, SystemTrayEvent, CustomMenuItem, SystemTrayMenuItem};
use tauri::Manager;

#[tauri::command]
fn hide_window(window: tauri::Window) {
    window.hide().unwrap();
}

fn main() {
    let quit = CustomMenuItem::new("quit".to_string(), "關閉程式");
    let hide = CustomMenuItem::new("hide".to_string(), "隱藏視窗");
    let tray_menu = SystemTrayMenu::new()
        .add_item(quit)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(hide);

    tauri::Builder::default()
        .system_tray(SystemTray::new().with_menu(tray_menu))
        .on_system_tray_event(|app, event|
            {
                match event {
                SystemTrayEvent::LeftClick {
                    position,
                    size: _,
                    ..
                } => {
                    let window = app.get_window("main").unwrap();
                    match window.is_visible() {
                        Ok(visible) => {
                            if visible {
                                window.hide().unwrap();
                                return
                            }
                            window.show().unwrap();
                            window.set_focus().unwrap();
                            window.set_position(position).unwrap();
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
                        window.hide().unwrap();
                      }
                      _ => {}
                    }
                  }
                _ => {}
            }
        })
        .plugin(tauri_plugin_positioner::init())
        .invoke_handler(tauri::generate_handler![hide_window])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
