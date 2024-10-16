use db::Database;
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

use tauri::Manager;
use std::sync::{Arc, RwLock};
use std::env;
pub mod models;
pub mod schema;
mod functions;
mod db;







#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {


    let builder = tauri::Builder::default()
        .setup(|app| {
            
            let main_window = app.get_webview_window("main").unwrap();
            main_window.eval("window.location.href= '/';").unwrap();
            app.manage(Arc::new(RwLock::new(None)) as Database);
            Ok(())
        })
        .plugin(tauri_plugin_shell::init());

        functions::register(builder).run(tauri::generate_context!())
        .expect("error while running tauri application");
}
