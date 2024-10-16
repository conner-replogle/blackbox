use std::path::PathBuf;
use std::sync::{Arc, RwLock};
use diesel::r2d2::{ConnectionManager, CustomizeConnection, Pool};
use diesel::{ConnectionError, SqliteConnection};
use diesel::connection::SimpleConnection;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use dotenvy::dotenv;

pub type Database = Arc<RwLock<Option<Pool<ConnectionManager<SqliteConnection>>>>>;
const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations/");

#[derive(Clone,Debug)]
pub struct EncryptedCustomizer{
    password: String
}

impl CustomizeConnection<SqliteConnection, diesel::r2d2::Error> for EncryptedCustomizer{
    fn on_acquire(&self, conn: &mut SqliteConnection) -> Result<(), diesel::r2d2::Error> {
        tracing::debug!("Acquiring connection");
        conn.batch_execute(&format!("PRAGMA key='{}'",self.password)).map_err(diesel::r2d2::Error::QueryError)?;
        conn.batch_execute("
            PRAGMA busy_timeout = 10;
            PRAGMA journal_mode = WAL;
            PRAGMA synchronous = NORMAL;
            PRAGMA wal_autocheckpoint = 1000;
            PRAGMA wal_checkpoint(TRUNCATE);
            PRAGMA foreign_keys = ON;
        ").map_err(diesel::r2d2::Error::QueryError)?;
        conn.run_pending_migrations(MIGRATIONS).unwrap();

        Ok(())
    }
}



pub fn establish_connection(password: &str,path:PathBuf) -> Result<Pool<ConnectionManager<SqliteConnection>>,diesel::r2d2::Error> {
    dotenv().ok();

    if !path.exists(){
        tracing::debug!("Creating directory");
        std::fs::create_dir_all(path.as_path()).unwrap();

    }

    let db_path = path.join("blackbox.db");
    tracing::debug!("Connecting to {}", db_path.as_path().to_str().unwrap());
    let pool = match Pool::builder()
        .connection_customizer(Box::new(EncryptedCustomizer{password: password.to_string()}))
        .max_size(1)
        .build(ConnectionManager::<SqliteConnection>::new(db_path.as_path().to_str().unwrap())){
        Ok(pool) => pool,
        Err(err) => {
            tracing::debug!("Error creating pool: {:?}",err);
            return Err(diesel::r2d2::Error::ConnectionError(ConnectionError::BadConnection("Incorrect Password or connection issue".to_string())))
        }
    };







    Ok(pool)
}
