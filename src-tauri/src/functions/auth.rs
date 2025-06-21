use std::path::PathBuf;
use std::str::FromStr;

use crate::db::{self, Database};
use diesel::r2d2::ConnectionManager;
use diesel::{prelude::*, r2d2::Pool};
use tauri::async_runtime::spawn_blocking;
use tauri::State;
use tauri::{AppHandle, Manager};

#[tauri::command]
pub fn check_auth(state: State<'_, Database>) -> Result<bool, String> {
    let unlocked = state.read().unwrap().is_some();
    log::debug!("Checking auth: {}", unlocked);

    Ok(unlocked)
}

#[tauri::command]
pub async fn unlock(
    state: State<'_, Database>,
    app: AppHandle,
    password: String,
) -> Result<bool, String> {
    log::debug!("Unlocking");
    #[cfg(debug_assertions)]
    let path = PathBuf::from_str("../data").unwrap();
    #[cfg(not(debug_assertions))]
    let path = app.path().app_data_dir().unwrap();

    //attempting not to block thread maybe just async function does this but idc I already wrote this
    let a =
        spawn_blocking::<_, Result<Pool<ConnectionManager<SqliteConnection>>, String>>(move || {
            let pool = db::establish_connection(&password, path).map_err(|a| a.to_string())?;
            return Ok(pool);
        });

    let pool = a.await.unwrap()?;

    state.write().map(|mut a| a.replace(pool)).unwrap();

    Ok(true)
}

#[tauri::command]
pub fn lock(state: State<'_, Database>) -> Result<(), String> {
    log::debug!("Locking");

    state.write().map(|mut a| a.take()).unwrap();

    Ok(())
}
