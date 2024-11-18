use crate::db::Database;
use crate::models::{NewPrivateKey, PrivateKey};
use diesel::prelude::*;
use pgp::crypto::hash::HashAlgorithm;
use pgp::crypto::rsa;
use pgp::crypto::sym::SymmetricKeyAlgorithm;
use pgp::types::{public, CompressionAlgorithm, SecretKeyTrait};
use pgp::{types::PublicKeyTrait as _, Deserializable, SignedSecretKey};
use pgp::{ArmorOptions, KeyType, SecretKeyParamsBuilder};
use rand::rngs::ThreadRng;
use serde::{Deserialize, Serialize};
use smallvec::smallvec;
use tauri::State;
#[derive(Debug, Deserialize, Serialize)]
pub struct Output {
    pub private_key: String,
    pub public_key: String,
}

#[tauri::command]
pub async fn generate_key(password: String) -> Result<Output, String> {
    let mut rng = ThreadRng::default();
    let mut key_params = SecretKeyParamsBuilder::default();
    key_params
        .key_type(KeyType::Rsa(2048))
        .can_certify(false)
        .can_sign(true)
        .primary_user_id("Me <me@example.com>".into())
        .created_at(chrono::Utc::now())
        .preferred_symmetric_algorithms(smallvec![SymmetricKeyAlgorithm::AES256])
        .preferred_hash_algorithms(smallvec![HashAlgorithm::SHA2_256])
        .preferred_compression_algorithms(smallvec![CompressionAlgorithm::ZLIB]);
    let secret_key_params = key_params.build().map_err(|a| a.to_string())?;
    let secret_key = secret_key_params
        .generate(&mut rng)
        .map_err(|e| e.to_string())?;
    let passwd_fn = || password;
    let signed_secret_key = secret_key
        .sign(&mut rng, passwd_fn.clone())
        .map_err(|a| a.to_string())?;

    let public_key = signed_secret_key.public_key();
    let private_key = signed_secret_key
        .to_armored_string(ArmorOptions::default())
        .map_err(|a| a.to_string())?;
    let public_key = public_key
        .sign(rng, &signed_secret_key, passwd_fn)
        .map_err(|a| a.to_string())?;
    let public_key = public_key
        .to_armored_string(ArmorOptions::default())
        .map_err(|a| a.to_string())?;
    Ok(Output {
        private_key,
        public_key,
    })
}
