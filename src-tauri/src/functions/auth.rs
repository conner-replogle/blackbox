use diesel::{prelude::*, r2d2::Pool};
use crate::db::{self, Database};
use tauri::async_runtime::spawn_blocking;
use tauri::{AppHandle, Manager};
use tauri::State;
use diesel::r2d2::ConnectionManager;

#[tauri::command]
pub fn check_auth(state: State<'_, Database>) -> Result<bool,String> {
    let unlocked = state.read().unwrap().is_some() ;
    tracing::debug!("Checking auth: {}",unlocked);

    Ok(unlocked)

}

#[tauri::command]
pub async fn unlock(state: State<'_, Database>,app:AppHandle, password: String) -> Result<bool, String> {
    tracing::debug!("Unlocking");

    //attempting not to block thread maybe just async function does this but idc I already wrote this
    let a = spawn_blocking::<_,Result<Pool<ConnectionManager<SqliteConnection>>,String>>(move ||{
        let pool = db::establish_connection(&password,app.path().app_data_dir().unwrap()).map_err(|a| a.to_string())?;
        Ok(pool)

    });
    let pool = a.await.unwrap()?;

    state.write().map(|mut a| a.replace(pool)).unwrap();


    Ok(true)
}

#[tauri::command]
pub fn lock(state: State<'_, Database>) -> Result<(), String> {
    tracing::debug!("Locking");

    state.write().map(|mut a| a.take()).unwrap();

    Ok(())
}

