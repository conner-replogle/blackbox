use db::Database;
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

use pgp::types::{CompressionAlgorithm, SecretKeyTrait, SignatureBytes};
use pgp::{message, ArmorOptions, Signature};
use pgp::{ types::PublicKeyTrait as _, Deserializable, Message, SignedPublicKey, SignedSecretKey};
use rand::rngs::ThreadRng;
use tauri::async_runtime::spawn_blocking;
use tauri::{AppHandle, Emitter, Manager};
use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};
use std::{env, str::from_utf8};
use tauri::State;
use diesel::r2d2::{ConnectionManager, CustomizeConnection};
pub mod models;
pub mod schema;
mod functions;
mod db;


use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};





#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {


    let builder = tauri::Builder::default()
        .setup(|app| {
            
            let main_window = app.get_webview_window("main").unwrap();
            main_window.eval(&"window.location.href= '/';".to_string()).unwrap();
            app.manage(Arc::new(RwLock::new(None)) as Database);
            Ok(())
        })
        .plugin(tauri_plugin_shell::init());

        functions::register(builder).run(tauri::generate_context!())
        .expect("error while running tauri application");
}
