use chrono::NaiveDateTime;
use diesel::{dsl::{AsSelect, Filter, IsNotNull, Select}, prelude::*, sqlite::Sqlite};
use serde::{Deserialize, Serialize};

use crate::schema::keys;

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::schema::keys)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Key {
    pub key_id: String,
    pub nickname: String,
    pub metadata: Option<String>,
    pub is_me: bool,
    pub private_key: Option<String>,
    pub public_key: String,
    pub created_at: NaiveDateTime,
}

impl Key{
    pub fn is_private(&self) -> bool {
        self.private_key.is_some()
    }

    pub fn public_keys() -> All {
        keys::table.select(Key::as_select())
    }
    pub fn private_keys() -> Private {
        Self::public_keys()
            .filter(keys::private_key.is_not_null())
            
    }
    
}
type All = Select<keys::table, AsSelect<Key, Sqlite>>;
type Private =Filter<All, IsNotNull<keys::private_key>>;




#[derive(Insertable)]
#[diesel(table_name = crate::schema::keys)]
pub struct NewPrivateKey<'a> {
    pub key_id: &'a str,
    pub nickname: &'a str,
    pub metadata: Option<&'a str>,
    pub private_key: &'a str,
    pub public_key: &'a str,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::keys)]
pub struct NewPublicKey<'a> {
    pub key_id: &'a str,
    pub nickname: &'a str,
    pub is_me: bool,
    pub metadata: Option<&'a str>,
    pub public_key: &'a str,
}
