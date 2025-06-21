use tauri::{generate_handler, Builder, Wry};

mod auth;
mod decrypt;
mod encrypt;
mod generate;
mod private_keys;
mod public_keys;
mod wallet;
pub fn register(builder: Builder<Wry>) -> Builder<Wry> {
    builder.invoke_handler(generate_handler![
        private_keys::get_private_keys,
        private_keys::add_private_key,
        public_keys::get_public_keys,
        public_keys::add_public_key,
        public_keys::remove_key,
        auth::lock,
        auth::unlock,
        auth::check_auth,
        encrypt::encrypt_message,
        decrypt::decrypt_message,
        generate::generate_key,
        wallet::open_rpc,
        wallet::close_rpc,
        wallet::check_rpc,
        wallet::test_rpc
    ])
}
