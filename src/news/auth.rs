use crate::services::pbkdf2;
use base64::{engine::general_purpose, Engine as _};
use dotenv::dotenv;
use std::env;

pub fn load_salt() -> pbkdf2::Salt {
    dotenv().ok();
    let salt_base64 = env::var("SALT_16_BYTES_BASE_64").expect("SALT_16_BYTES_BASE_64 must be set");
    let salt_arr = general_purpose::STANDARD.decode(salt_base64).unwrap();
    let mut salt = [0u8; 16];
    salt.copy_from_slice(&salt_arr[..16]);

    salt
}
