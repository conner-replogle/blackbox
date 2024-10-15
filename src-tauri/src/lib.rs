// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
use diesel::{connection::SimpleConnection, prelude::*, r2d2::Pool};
use dotenvy::dotenv;
use models::{NewPrivateKey, NewPublicKey, PrivateKey, PublicKey};
use pgp::crypto::aead::AeadAlgorithm;
use pgp::crypto::hash::HashAlgorithm;
use pgp::crypto::sym::SymmetricKeyAlgorithm;
use pgp::ser::Serialize;
use pgp::types::{CompressionAlgorithm, SecretKeyTrait, SignatureBytes};
use pgp::{message, ArmorOptions, Signature};
use pgp::{ types::PublicKeyTrait as _, Deserializable, Message, SignedPublicKey, SignedSecretKey};
use rand::rngs::ThreadRng;
use schema::private_keys;
use tauri::async_runtime::spawn_blocking;
use tauri::{AppHandle, Emitter, Manager};
use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};
use std::{env, str::from_utf8};
use tauri::State;
use diesel::r2d2::{ConnectionManager, CustomizeConnection};
pub mod models;
pub mod schema;
mod functions;
pub type Database = Arc<RwLock<Option<Pool<ConnectionManager<SqliteConnection>>>>>;
use functions::private_keys::*;
use functions::public_keys::*;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations/");

#[derive(Clone,Debug)]
pub struct EncryptedCustomizer{
    password: String
}

impl CustomizeConnection<SqliteConnection, diesel::r2d2::Error> for EncryptedCustomizer{
    fn on_acquire(&self, conn: &mut SqliteConnection) -> Result<(), diesel::r2d2::Error> {
        println!("Acquiring connection");
        conn.batch_execute(&format!("PRAGMA key='{}'",self.password)).map_err(|a| diesel::r2d2::Error::QueryError(a))?;
        conn.batch_execute("
            PRAGMA busy_timeout = 10;
            PRAGMA journal_mode = WAL;
            PRAGMA synchronous = NORMAL;
            PRAGMA wal_autocheckpoint = 1000;
            PRAGMA wal_checkpoint(TRUNCATE);
            PRAGMA foreign_keys = ON;
        ").map_err(|a| diesel::r2d2::Error::QueryError(a))?;
        conn.run_pending_migrations(MIGRATIONS).unwrap();
 
        Ok(())
    }
}



pub fn establish_connection(password: &str,path:PathBuf) -> Result<Pool<ConnectionManager<SqliteConnection>>,diesel::r2d2::Error> {
    dotenv().ok();

    if !path.exists(){
        println!("Creating directory");
        std::fs::create_dir_all(path.as_path()).unwrap();

    }

    let db_path = path.join("blackbox.db");
    println!("Connecting to {}", db_path.as_path().to_str().unwrap());
    let pool = match Pool::builder()
        .connection_customizer(Box::new(EncryptedCustomizer{password: password.to_string()}))
        .max_size(1)
        .build(ConnectionManager::<SqliteConnection>::new(db_path.as_path().to_str().unwrap())){
            Ok(pool) => pool,
            Err(err) => {
                println!("Error creating pool: {:?}",err);
                return Err(diesel::r2d2::Error::ConnectionError(ConnectionError::BadConnection("Incorrect Password or connection issue".to_string())))
            }
        };

        
        
    
    
    

    return Ok(pool);
}

#[tauri::command]
fn decrypt_message(state: State<'_, Database>,pkey_id: &str,message: &str,pass_key:&str,signer: Option<&str>) -> Result<String, String> {
    let Ok(Some(state)) = state.read().map(|a| a.clone()) else{
        return Err("Error reading state".to_string());
    };
    use crate::private_keys::dsl::*;
    let result = private_keys
        .filter(key_id.eq(pkey_id))
        .select(PrivateKey::as_select())
        .load(&mut state.get().unwrap())
        .expect("Error loading private_keys");

    println!("Looking for key {}", pkey_id);
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
            println!("Error parsing key: {}",err);
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
                    println!("Error parsing key: {}",err);
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
                    println!("{:?}",signature);
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

#[tauri::command]
fn encrypt_message(state: State<'_, Database>,pkey_id: &str,message: &str,pass_key:&str,signer: Option<&str>) -> Result<String, String> {
    let Ok(Some(state)) = state.read().map(|a| a.clone()) else{
        return Err("Error reading state".to_string());
    };
    use crate::schema::public_keys::dsl::*;
    let result = public_keys
        .filter(key_id.eq(pkey_id))
        .select(PublicKey::as_select())
        .load(&mut state.get().unwrap())
        .expect("Error loading public ");

    println!("Looking for key {}", pkey_id);
    

    if result.len() == 0 {
        return Err("Public key not found".to_string());
    }else if result.len() > 1{
        return Err("Multiple Public keys found".to_string());
    }
    let key = &result[0];
    let key = match SignedPublicKey::from_string(&key.public_key){
        Ok(key) => key.0,
        Err(err) =>{
            println!("Error parsing key: {}",err);
            return Err(err.to_string());
        }
    };
    let mut rng = ThreadRng::default();

    
    let signer_key = match signer{
        Some(signer) => {
            use crate::schema::private_keys::dsl::*;
            let result = private_keys
                .filter(key_id.eq(signer))
                .select(PrivateKey::as_select())
                .load(&mut state.get().unwrap())
                .expect("Error loading private_keys");
            if result.len() == 0 {
                return Err("Private key not found".to_string());
            }else if result.len() > 1{
                return Err("Multiple private keys found".to_string());
            }
            let key = &result[0];
            let key = match SignedSecretKey::from_string(&key.private_key){
                Ok(key) => key.0,
                Err(err) =>{
                    println!("Error parsing key: {}",err);
                    return Err(err.to_string());
                }
            };
            Some(key)
        },
        None => {
            
            None
        }
    };
    //SHOUDL I HAVE FILE NAME IDC IT WORKS BUT LIKE WHY IT HERE
    let mut message = Message::new_literal("msg.txt", message);
    if let Some(sign) = signer_key{
        message = message.sign(&mut rng, &sign, || pass_key.to_string(), HashAlgorithm::SHA2_256).map_err(|a| a.to_string())?;
    }
    
       // Encrypt the message
    let encrypted_message = message.compress(CompressionAlgorithm::ZLIB).map_err(|a| a.to_string())?

    // WHAT THE FUCK ARE THESE OPTIONS NEED TO FIND OUT STANDARD
    .encrypt_to_keys_seipdv2(
    &mut rng,
        SymmetricKeyAlgorithm::AES128,
        AeadAlgorithm::Ocb,
        0x06,// THIS ONE ESPCIALLY LUCKY FUCKING GUESS maybe it needs to be a 1000 or -6 or 1 I have no clue
        &[&key],      
    )
    .expect("Encryption failed");

    //WHAT IS ARMOR
    let text = encrypted_message.to_armored_string(ArmorOptions::default()).map_err(|a| a.to_string())?;


    return Ok(text);
}

#[derive(Clone, serde::Serialize)]
struct LockChange {
  unlocked: bool,
}
#[tauri::command]
fn check_auth(app: AppHandle,state: State<'_, Database>) -> Result<bool,String> {
    let unlocked = state.read().unwrap().is_some() ;
    println!("Checking auth: {}",unlocked);


    return Ok(unlocked);

}

#[tauri::command]
async fn unlock(state: State<'_, Database>,app: tauri::AppHandle, password: String) -> Result<bool, String> {
    println!("Unlocking");

    //attempting not to block thread maybe just async function does this but idc I already wrote this
    let a = spawn_blocking::<_,Result<Pool<ConnectionManager<SqliteConnection>>,String>>(move ||{
        let pool = establish_connection(&password,app.path().app_data_dir().unwrap()).map_err(|a| a.to_string())?;
        return Ok(pool);
        
    });
    let pool = a.await.unwrap()?;

    state.write().map(|mut a| a.replace(pool)).unwrap();

    
    Ok(true)
}

#[tauri::command]
fn lock(state: State<'_, Database>,app: tauri::AppHandle) -> Result<(), String> {
    println!("Locking");

    state.write().map(|mut a| a.take()).unwrap();

    Ok(())
}





#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {


    tauri::Builder::default()
        .setup(|app| {
            
            let main_window = app.get_webview_window("main").unwrap();
            main_window.eval(&format!("window.location.href= '/';")).unwrap();
            app.manage(Arc::new(RwLock::new(None)) as Database);
            Ok(())
        })
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![encrypt_message,add_private_key,get_private_keys,decrypt_message,add_public_key,get_public_keys,check_auth,unlock,lock])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
