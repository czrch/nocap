mod commands;
mod models;
mod utils;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    use tauri::menu::{Menu, MenuItem};
    use tauri::tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};
    use tauri::{Emitter, Manager};

    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            let show = MenuItem::with_id(app, "tray_show", "Show", true, None::<&str>)?;
            let preferences =
                MenuItem::with_id(app, "tray_preferences", "Preferences…", true, None::<&str>)?;
            let quit = MenuItem::with_id(app, "tray_quit", "Quit", true, None::<&str>)?;

            let menu = Menu::with_items(app, &[&show, &preferences, &quit])?;

            let icon = match app.default_window_icon().cloned() {
                Some(icon) => icon,
                None => {
                    // No icon configured; skip tray setup.
                    return Ok(());
                }
            };

            TrayIconBuilder::with_id("main")
                .menu(&menu)
                .icon(icon)
                .tooltip("nocap")
                // Prefer left click to show the window (menu still available via right click).
                .show_menu_on_left_click(false)
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click {
                        button,
                        button_state,
                        ..
                    } = event
                    {
                        if button == MouseButton::Left && button_state == MouseButtonState::Down {
                            if let Some(window) = tray.app_handle().get_webview_window("main") {
                                let _ = window.show();
                                let _ = window.set_focus();
                            }
                        }
                    }
                })
                .on_menu_event(|app, event| match event.id().as_ref() {
                    "tray_show" => {
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                    "tray_preferences" => {
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                        let _ = app.emit("nocap://open-settings", ());
                    }
                    "tray_quit" => {
                        app.exit(0);
                    }
                    _ => {}
                })
                .build(app)?;

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::get_adjacent_images,
            commands::scan_folder_for_images,
            commands::get_image_metadata,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
