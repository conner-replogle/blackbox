use tauri::{generate_handler, ipc::Invoke, Builder, Runtime, Wry};

mod private_keys;
mod public_keys;
mod auth;
mod encrypt;
mod decrypt;
mod generate;

pub fn register(builder: Builder<Wry>) -> Builder<Wry>
{
    builder.invoke_handler(generate_handler![
        private_keys::get_private_keys,
        private_keys::add_private_key,
        public_keys::get_public_keys,
        public_keys::add_public_key,
        auth::lock,
        auth::unlock,
        auth::check_auth,
        encrypt::encrypt_message,
        decrypt::decrypt_message
    ])

}