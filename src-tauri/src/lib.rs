mod commands;
mod models;
mod utils;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            commands::get_adjacent_images,
            commands::scan_folder_for_images,
            commands::get_image_metadata,
            commands::get_exif_summary,
            commands::rename_image_file,
            commands::save_as_copy,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
