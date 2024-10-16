use diesel::prelude::*;
use crate::db::Database;
use crate::models::{NewPublicKey, PublicKey};
use pgp::{ types::PublicKeyTrait as _, Deserializable, SignedPublicKey};
use tauri::State;


#[tauri::command]
pub fn get_public_keys(state: State<'_, Database>) -> Result<Vec<PublicKey>,String> {
    let Ok(Some(state)) = state.read().map(|a| a.clone()) else{
        return Err("Error reading state".to_string());
    };
    use crate::schema::public_keys::dsl::public_keys;
    tracing::debug!("Getting public keys ");
  
    let results = public_keys
      
        .select(PublicKey::as_select())
        .load(&mut state.get().unwrap())
        .expect("Error loading public_keys");
    Ok(results)
}

#[tauri::command]
pub fn add_public_key(state: State<'_, Database>,nickname: &str,public_key: &str) -> Result<String, String> {
    let Ok(Some(state)) = state.read().map(|a| a.clone()) else{
        return Err("Error reading state".to_string());
    };
    tracing::debug!("Adding public key {}", nickname);
    let key = match SignedPublicKey::from_string(public_key){
        Ok(key) => key.0,
        Err(err) =>{
            tracing::debug!("Error parsing key: {}",err);
            return Err(err.to_string());
        }
    };
    if let Err(err) = key.verify(){
        tracing::debug!("Error verifying key: {}",err);
        return Err(err.to_string());
    }

    let decrypt_id = hex::encode(key.key_id());

    

    let new_key = NewPublicKey{ nickname, public_key,key_id: &decrypt_id};
  
    let key: PublicKey = diesel::insert_into(crate::schema::public_keys::table)
        .values(&new_key)
        .returning(PublicKey::as_returning())
        .get_result(&mut state.get().unwrap())
        .expect("Error saving new public key");

    Ok(key.key_id)
}
