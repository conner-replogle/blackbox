use crate::db::Database;
use crate::functions::public_keys::{add_public_key, remove_public_key};
use crate::models::{NewPrivateKey,PrivateKey};
use diesel::prelude::*;
use pgp::types::{public, SecretKeyTrait};
use pgp::{ArmorOptions, PublicKey};
use pgp::{types::PublicKeyTrait as _, Deserializable, SignedSecretKey};
use rand::rngs::ThreadRng;
use tauri::State;

#[tauri::command]
pub fn get_private_keys(state: State<'_, Database>) -> Result<Vec<PrivateKey>, String> {
    let Ok(Some(state)) = state.read().map(|a| a.clone()) else {
        return Err("Error reading state".to_string());
    };
    use crate::schema::private_keys::dsl::private_keys;
    log::debug!("Gettings private key ");

    let results = private_keys
        .select(PrivateKey::as_select())
        .load(&mut state.get().unwrap())
        .expect("Error loading private_keys");
    Ok(results)
}
#[tauri::command]
pub fn remove_private_key(state: State<'_, Database>, key_id: &str) -> Result<(), String> {
    let Ok(Some(state)) = state.read().map(|a| a.clone()) else {
        return Err("Error reading state".to_string());
    };
    let del_key_id = key_id;
    {
        use crate::schema::private_keys::dsl::*;
        log::debug!("Deleting private key {}", del_key_id);
        diesel::delete(private_keys.filter(key_id.eq(del_key_id))).execute(&mut state.get().unwrap()).unwrap();
    }
    Ok(())
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


    let public_key_id = &add_public_key(fstate, &format!("{nickname}"), &public_key,Some(true)).unwrap();

    let decrypt_id = hex::encode(key.key_id());

    let new_key = NewPrivateKey {
        key_id: &decrypt_id,
        nickname,
        metadata: None,
        private_key,
        public_key_id,
       
    };

    let key: PrivateKey = diesel::insert_into(crate::schema::private_keys::table)
        .values(&new_key)
        .returning(PrivateKey::as_returning())
        .get_result(&mut state.get().unwrap())
        .expect("Error saving new private key");

    Ok(key.key_id)
}
