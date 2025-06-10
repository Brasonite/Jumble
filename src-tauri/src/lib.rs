mod jumbler;
mod lang;
mod processor;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_os::init())
        .invoke_handler(tauri::generate_handler![
            jumbler::jumble,
            lang::native_localize
        ])
        .run(tauri::generate_context!())
        .expect("Error while running Tauri application");
}
