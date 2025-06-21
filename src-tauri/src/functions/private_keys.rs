use crate::db::Database;
use crate::functions::public_keys;
use crate::models::{Key, NewPrivateKey, Key as PrivateKey};
use diesel::prelude::*;
use pgp::types::{public, SecretKeyTrait};
use pgp::{ArmorOptions};
use pgp::{types::PublicKeyTrait as _, Deserializable, SignedSecretKey};
use rand::rngs::ThreadRng;
use tauri::State;

#[tauri::command]
pub fn get_private_keys(state: State<'_, Database>) -> Result<Vec<PrivateKey>, String> {
    let Ok(Some(state)) = state.read().map(|a| a.clone()) else {
        return Err("Error reading state".to_string());
    };
    log::debug!("Gettings private key ");

    let results = Key::private_keys()
        .load(&mut state.get().unwrap())
        .expect("Error loading private_keys");
    Ok(results)
}


#[tauri::command]
pub fn add_private_key(
    state: State<'_, Database>,
    nickname: &str,
    private_key: &str,
    password: &str,
) -> Result<String, String> {
    let fstate = state.clone();
    let Ok(Some(state)) = state.read().map(|a| a.clone()) else {
        return Err("Error reading state".to_string());
    };
    log::debug!("Adding private key {}", nickname);
    let key = match SignedSecretKey::from_string(private_key) {
        Ok(key) => key.0,
        Err(err) => {
            log::debug!("Error parsing key: {}", err);
            return Err(err.to_string());
        }
    };
    if let Err(err) = key.verify() {
        log::debug!("Error verifying key: {}", err);
        return Err(err.to_string());
    }
    let rng = ThreadRng::default();
    let public_key = key.public_key();
    let passwd_fn = || password.to_string();
    let public_key = public_key
    .sign(rng, &key, passwd_fn)
    .map_err(|a| a.to_string())?;
    let public_key = public_key
    .to_armored_string(ArmorOptions::default())
    .map_err(|a| a.to_string())?;



    let decrypt_id = hex::encode(key.key_id());

    let new_key = NewPrivateKey {
        key_id: &decrypt_id,
        nickname,
        metadata: None,
        private_key,
        public_key: &public_key,
       
    };

    let key: PrivateKey = diesel::insert_into(crate::schema::keys::table)
        .values(&new_key)
        .returning(PrivateKey::as_returning())
        .get_result(&mut state.get().unwrap()).map_err(|err| {
            log::error!("Error inserting public key: {}", err);
            "Failed to insert public key".to_string()
        })?;
    log::debug!("Added public key with ID: {}", key.key_id);


    Ok(key.key_id)
}
