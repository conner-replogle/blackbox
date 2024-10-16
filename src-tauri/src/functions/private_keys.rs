use diesel::prelude::*;
use crate::db::Database;
use crate::models::{NewPrivateKey, PrivateKey};
use pgp::{ types::PublicKeyTrait as _, Deserializable, SignedSecretKey};
use tauri::State;

#[tauri::command]
pub fn get_private_keys(state: State<'_, Database>) -> Result<Vec<PrivateKey>,String> {
    let Ok(Some(state)) = state.read().map(|a| a.clone()) else{
        return Err("Error reading state".to_string());
    };
    use crate::schema::private_keys::dsl::private_keys;
    tracing::debug!("Gettings private key ");
  
    let results = private_keys
      
        .select(PrivateKey::as_select())
        .load(&mut state.get().unwrap())
        .expect("Error loading private_keys");
    Ok(results)
}


#[tauri::command]
pub fn add_private_key(state: State<'_, Database>,nickname: &str,private_key: &str) -> Result<String, String> {
    let Ok(Some(state)) = state.read().map(|a| a.clone()) else{
        return Err("Error reading state".to_string());
    };
    tracing::debug!("Adding private key {}", nickname);
    let key = match SignedSecretKey::from_string(private_key){
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

    

    let new_key = NewPrivateKey{ nickname, private_key,key_id: &decrypt_id};
  
    let key: PrivateKey = diesel::insert_into(crate::schema::private_keys::table)
        .values(&new_key)
        .returning(PrivateKey::as_returning())
        .get_result(&mut state.get().unwrap())
        .expect("Error saving new private key");

    Ok(key.key_id)
}