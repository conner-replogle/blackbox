use diesel::{connection::SimpleConnection, prelude::*, r2d2::Pool};
use dotenvy::dotenv;
use crate::db::Database;
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
pub fn decrypt_message(state: State<'_, Database>,pkey_id: &str,message: &str,pass_key:&str,signer: Option<&str>) -> Result<String, String> {
    let Ok(Some(state)) = state.read().map(|a| a.clone()) else{
        return Err("Error reading state".to_string());
    };
    use crate::schema::private_keys::dsl::*;
    let result = private_keys
        .filter(key_id.eq(pkey_id))
        .select(PrivateKey::as_select())
        .load(&mut state.get().unwrap())
        .expect("Error loading private_keys");

    tracing::debug!("Looking for key {}", pkey_id);
    let message = Message::from_string(message).map_err(|a| a.to_string())?.0;


    if result.len() == 0 {
        return Err("Private key not found".to_string());
    }else if result.len() > 1{
        return Err("Multiple private keys found".to_string());
    }
    let key = &result[0];
    let key = match SignedSecretKey::from_string(&key.private_key){
        Ok(key) => key.0,
        Err(err) =>{
            tracing::debug!("Error parsing key: {}",err);
            return Err(err.to_string());
        }
    };

    let (decrypted_msg,_keys) = message.decrypt(||{
        pass_key.to_string()
    }, &[&key]).map_err(|err| err.to_string())?;

    let signer_key = match signer{
        Some(signer) => {
            use crate::schema::public_keys::dsl::*;
            let result = public_keys
                .filter(key_id.eq(signer))
                .select(PublicKey::as_select())
                .load(&mut state.get().unwrap())
                .expect("Error loading public_keys");
            if result.len() == 0 {
                return Err("Public key not found".to_string());
            }else if result.len() > 1{
                return Err("Multiple public keys found".to_string());
            }
            let key = &result[0];
            let key = match SignedPublicKey::from_string(&key.public_key){
                Ok(key) => key.0,
                Err(err) =>{
                    tracing::debug!("Error parsing key: {}",err);
                    return Err(err.to_string());
                }
            };
            Some(key)
        },
        None => {

            None
        }
    };


    let mut header = String::new();
    let raw;
    let mut msg = decrypted_msg.clone();
    loop{

        match msg {
            Message::Literal(data) => {raw = data; break;},
            Message::Signed { message, one_pass_signature: _, signature } =>{
                let key = signer_key.as_ref().ok_or("No signer key")?;
                if let Some(message) = message {
                    match *message {
                        Message::Literal(ref data) => signature.verify(key, data.data()),
                        _ => {
                            let data = message.to_bytes().unwrap();
                            signature.verify(key, &data[..])
                        }
                    }
                        .map_err(|err| err.to_string())?;
                    tracing::debug!("{:?}",signature);
                    header += &format!("Signed by {:?} \n",signature.issuer());
                    if let Some(date) = signature.created() {
                        header += &format!("Created on {} \n",signature.created().unwrap().to_rfc3339())
                    };
                    header += &format!("Fingerprint: {:?} \n",signature.issuer_fingerprint());
                    header += &format!("Signers Userid: {:?} \n",signature.signers_userid());
                    header += "--------------\n";




                    msg = *message;
                } else {
                    panic!("no message, what to do?");
                }

            }
            Message::Compressed(data) => {
                let m = Message::from_bytes(data.decompress().unwrap()).unwrap();
                msg = m;

            },
            a => return Err(format!("Invalid message {a:?}"))
        }
    };

    let text = header + from_utf8(raw.data()).map_err(|a| a.to_string())?;



    return Ok(text);
}

