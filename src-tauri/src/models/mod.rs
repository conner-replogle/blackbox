use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable,Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::schema::private_keys)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct PrivateKey {
    pub key_id: String,
    pub nickname: String,
    pub metadata: Option<String>,
    pub private_key: String,
    pub public_key_id: String,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::private_keys)]
pub struct NewPrivateKey<'a> {
    pub key_id: &'a str,
    pub nickname: &'a str,
    pub metadata: Option<&'a str>,
    pub private_key: &'a str,
    pub public_key_id: &'a str,
}

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::schema::public_keys)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct PublicKey {
    pub key_id: String,
    pub nickname: String,
    pub metadata: Option<String>,
    pub is_me: bool,
    pub public_key: String,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::public_keys)]
pub struct NewPublicKey<'a> {
    pub key_id: &'a str,
    pub nickname: &'a str,
    pub is_me: bool,
    pub metadata: Option<&'a str>,
    pub public_key: &'a str,
}
