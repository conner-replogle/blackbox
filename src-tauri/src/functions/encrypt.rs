use crate::db::Database;
use crate::models::Key;
use diesel::prelude::*;
use pgp::crypto::hash::HashAlgorithm;
use pgp::crypto::sym::SymmetricKeyAlgorithm;
use pgp::types::{CompressionAlgorithm};
use pgp::ArmorOptions;
use pgp::{Deserializable, Message, SignedPublicKey, SignedSecretKey};
use rand::rngs::ThreadRng;
use tauri::State;

#[tauri::command]
pub fn encrypt_message(
    state: State<'_, Database>,
    pkey_id: &str,
    message: &str,
    pass_key: &str,
    signer: Option<&str>,
) -> Result<String, String> {
    let Ok(Some(state)) = state.read().map(|a| a.clone()) else {
        return Err("Error reading state".to_string());
    };
    use crate::schema::keys::dsl::*;
    let result = Key::public_keys()
        .filter(key_id.eq(pkey_id))
        .load(&mut state.get().unwrap())
        .expect("Error loading public ");

    log::debug!("Looking for key {}", pkey_id);

    if result.is_empty() {
        return Err("Public key not found".to_string());
    } else if result.len() > 1 {
        return Err("Multiple Public keys found".to_string());
    }
    let key = &result[0];
    let key = match SignedPublicKey::from_string(&key.public_key) {
        Ok(key) => key.0,
        Err(err) => {
            log::debug!("Error parsing key: {}", err);
            return Err(err.to_string());
        }
    };
    let mut rng = ThreadRng::default();

    let signer_key = match signer {
        Some(signer) => {
            let result:Vec<Key> = Key::private_keys()
                .filter(key_id.eq(signer))
                .load(&mut state.get().unwrap())
                .expect("Error loading private_keys");
            if result.is_empty() {
                return Err("Private key not found".to_string());
            } else if result.len() > 1 {
                return Err("Multiple private keys found".to_string());
            }
            let key = &result[0];
            let key = match SignedSecretKey::from_string(key.private_key.as_ref().unwrap()) {
                Ok(key) => key.0,
                Err(err) => {
                    log::debug!("Error parsing key: {}", err);
                    return Err(err.to_string());
                }
            };
            Some(key)
        }
        None => None,
    };
    //SHOUDL I HAVE FILE NAME IDC IT WORKS BUT LIKE WHY IT HERE
    let mut message = Message::new_literal("msg.txt", message);
    if let Some(sign) = signer_key {
        message = message
            .sign(
                &mut rng,
                &sign,
                || pass_key.to_string(),
                HashAlgorithm::SHA2_256,
            )
            .map_err(|a| a.to_string())?;
    }

    // Encrypt the message
    let encrypted_message = message
        .compress(CompressionAlgorithm::ZLIB)
        .map_err(|a| a.to_string())?
        // WHAT THE FUCK ARE THESE OPTIONS NEED TO FIND OUT STANDARD
        .encrypt_to_keys_seipdv1(&mut rng, SymmetricKeyAlgorithm::AES128, &[&key])
        // I belive this is not commonly used
        // .encrypt_to_keys_seipdv2(
        //     &mut rng,
        //     SymmetricKeyAlgorithm::AES128,
        //     AeadAlgorithm::Ocb,
        //     0x06,// THIS ONE ESPCIALLY LUCKY FUCKING GUESS maybe it needs to be a 1000 or -6 or 1 I have no clue
        //     &[&key],
        // )
        .expect("Encryption failed");

    //WHAT IS ARMOR
    let text = encrypted_message
        .to_armored_string(ArmorOptions::default())
        .map_err(|a| a.to_string())?;

    Ok(text)
}
