use diesel::{connection::SimpleConnection, prelude::*, r2d2::Pool};
use dotenvy::dotenv;
use crate::db::{self, Database};
use crate::models::{NewPrivateKey, NewPublicKey, PrivateKey, PublicKey};
use pgp::crypto::aead::AeadAlgorithm;
use pgp::crypto::hash::HashAlgorithm;
use pgp::crypto::sym::SymmetricKeyAlgorithm;
use pgp::ser::Serialize;
use pgp::types::{CompressionAlgorithm, SecretKeyTrait, SignatureBytes};
use pgp::{message, ArmorOptions, Signature};
use pgp::{ types::PublicKeyTrait as _, Deserializable, Message, SignedPublicKey, SignedSecretKey};
use rand::rngs::ThreadRng;
use crate::schema::private_keys;
use tauri::async_runtime::spawn_blocking;
use tauri::{AppHandle, Emitter, Manager};
use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};
use std::{env, str::from_utf8};
use tauri::State;
use diesel::r2d2::{ConnectionManager, CustomizeConnection};

#[tauri::command]
pub fn check_auth(state: State<'_, Database>) -> Result<bool,String> {
    let unlocked = state.read().unwrap().is_some() ;
    tracing::debug!("Checking auth: {}",unlocked);

    return Ok(unlocked);

}

#[tauri::command]
pub async fn unlock(state: State<'_, Database>,app:AppHandle, password: String) -> Result<bool, String> {
    tracing::debug!("Unlocking");

    //attempting not to block thread maybe just async function does this but idc I already wrote this
    let a = spawn_blocking::<_,Result<Pool<ConnectionManager<SqliteConnection>>,String>>(move ||{
        let pool = db::establish_connection(&password,app.path().app_data_dir().unwrap()).map_err(|a| a.to_string())?;
        return Ok(pool);

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

