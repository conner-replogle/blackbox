use crate::db::Database;
use crate::models::{Key, NewPublicKey, Key as PublicKey};
use diesel::prelude::*;
use pgp::{types::PublicKeyTrait as _, Deserializable, SignedPublicKey};
use tauri::State;

#[tauri::command]
pub fn get_public_keys(state: State<'_, Database>) -> Result<Vec<PublicKey>, String> {
    let Ok(Some(state)) = state.read().map(|a| a.clone()) else {
        return Err("Error reading state".to_string());
    };
   
    log::debug!("Getting public keys ");

    let results = Key::public_keys()
        .load(&mut state.get().unwrap())
        .expect("Error loading public_keys");
    Ok(results)
}
#[tauri::command]
pub fn remove_key(state: State<'_, Database>, key_id: &str) -> Result<(), String> {
    let Ok(Some(state)) = state.read().map(|a| a.clone()) else {
        return Err("Error reading state".to_string());
    };
    let del_key_id = key_id;
    {
    use crate::schema::keys::dsl::*;
    log::debug!("Deleting key {}", del_key_id);
    let id = diesel::delete(keys.filter(key_id.eq(del_key_id).and(is_me.eq(false)))).execute(&mut state.get().unwrap()).unwrap();
    if (id ==0){
        return Err("Either failed to find key or attempted to delete a private key's public key".to_string());
    }
    }
    Ok(())
}

#[tauri::command]
pub fn add_public_key(
    state: State<'_, Database>,
    nickname: &str,
    public_key: &str,
    is_me: Option<bool>
) -> Result<String, String> {
    let Ok(Some(state)) = state.read().map(|a| a.clone()) else {
        return Err("Error reading state".to_string());
    };
    log::debug!("Adding public key {}", nickname);
    let key = match SignedPublicKey::from_string(public_key) {
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

    let decrypt_id = hex::encode(key.key_id());

    let new_key = NewPublicKey {
        nickname,
        metadata: None,
        public_key,
        is_me: is_me.unwrap_or(false),
        key_id: &decrypt_id,
    };

    let key: Key = diesel::insert_into(crate::schema::keys::table)
        .values(&new_key)
        .returning(PublicKey::as_returning())
        .get_result(&mut state.get().unwrap()).map_err(|err| {
            log::error!("Error inserting public key: {}", err);
            "Failed to insert public key".to_string()
        })?;
    log::debug!("Added public key with ID: {}", key.key_id);

    Ok(key.key_id)
}
