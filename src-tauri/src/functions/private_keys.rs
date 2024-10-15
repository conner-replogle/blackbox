use diesel::{connection::SimpleConnection, prelude::*, r2d2::Pool};
use dotenvy::dotenv;
use crate::models::{NewPrivateKey, NewPublicKey, PrivateKey, PublicKey};
use pgp::{ types::PublicKeyTrait as _, Deserializable, Message, SignedPublicKey, SignedSecretKey};
use crate::schema::private_keys;
use tauri::Manager;
use std::{env, str::from_utf8};
use tauri::State;
use diesel::r2d2::ConnectionManager;

use crate::Database;

#[tauri::command]
pub fn get_private_keys(state: State<'_, Database>) -> Vec<PrivateKey> {
    use crate::private_keys::dsl::private_keys;
    println!("Gettings private key ");
  
    let results = private_keys
      
        .select(PrivateKey::as_select())
        .load(&mut state.get().unwrap())
        .expect("Error loading private_keys");
    return results;
}


#[tauri::command]
pub fn add_private_key(state: State<'_, Database>,nickname: &str,private_key: &str) -> Result<String, String> {

    println!("Adding private key {}", nickname);
    let key = match SignedSecretKey::from_string(private_key){
        Ok(key) => key.0,
        Err(err) =>{
            println!("Error parsing key: {}",err);
            return Err(err.to_string());
        }
    };
    if let Err(err) = key.verify(){
        println!("Error verifying key: {}",err);
        return Err(err.to_string());
    }

    let decrypt_id = hex::encode(key.key_id());

    

    let new_key = NewPrivateKey{ nickname, private_key,key_id: &decrypt_id};
  
    let key: PrivateKey = diesel::insert_into(crate::schema::private_keys::table)
        .values(&new_key)
        .returning(PrivateKey::as_returning())
        .get_result(&mut state.get().unwrap())
        .expect("Error saving new private key");

    return Ok(key.key_id);
}