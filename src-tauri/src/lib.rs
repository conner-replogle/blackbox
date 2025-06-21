use db::Database;
use monero::MoneroWallet;
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

use std::env;
use std::sync::{Arc, RwLock};
use tauri::Manager;
mod db;
mod monero;
mod functions;
pub mod models;
pub mod schema;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let builder = tauri::Builder::default()
        .plugin(tauri_plugin_log::Builder::new().build())
        .setup(|app| {
            let main_window = app.get_webview_window("main").unwrap();
            main_window.eval("window.location.href= '/unlock';").unwrap();
            app.manage(Arc::new(RwLock::new(None)) as Database);
            app.manage(Arc::new(tokio::sync::RwLock::new(None)) as MoneroWallet);
            Ok(())
        })
        .plugin(tauri_plugin_shell::init());

    functions::register(builder)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
