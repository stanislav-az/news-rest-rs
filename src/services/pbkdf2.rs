use ring::{digest, pbkdf2};
use std::num::NonZeroU32;

static PBKDF2_ALG: pbkdf2::Algorithm = pbkdf2::PBKDF2_HMAC_SHA256;
pub const CREDENTIAL_LEN: usize = digest::SHA256_OUTPUT_LEN;
pub const PBKDF2_ITERATIONS: u32 = 100_000;
pub type Credential = [u8; CREDENTIAL_LEN];
pub type Salt = [u8; 16];

#[derive(Debug, PartialEq, Eq)]
pub enum Pbkdf2Error {
    WrongUsernameOrPassword,
}

pub fn hash_with_salt(entropy: &Salt, username: &str, password: &str) -> Credential {
    let salt = salt(entropy, username);
    let mut to_store: Credential = [0u8; CREDENTIAL_LEN];
    let pbkdf2_iterations = NonZeroU32::new(PBKDF2_ITERATIONS).unwrap();
    pbkdf2::derive(
        PBKDF2_ALG,
        pbkdf2_iterations,
        &salt,
        password.as_bytes(),
        &mut to_store,
    );
    to_store
}

pub fn verify_password(
    entropy: &Salt,
    username: &str,
    attempted_password: &str,
    actual_password: &Credential,
) -> Result<(), Pbkdf2Error> {
    let salt = salt(entropy, username);
    let pbkdf2_iterations = NonZeroU32::new(PBKDF2_ITERATIONS).unwrap();
    pbkdf2::verify(
        PBKDF2_ALG,
        pbkdf2_iterations,
        &salt,
        attempted_password.as_bytes(),
        actual_password,
    )
    .map_err(|_| Pbkdf2Error::WrongUsernameOrPassword)
}

// The salt should have a user-specific component so that an attacker
// cannot crack one password for multiple users in the database. It
// should have a database-unique component so that an attacker cannot
// crack the same user's password across databases in the unfortunate
// but common case that the user has used the same password for
// multiple systems.
fn salt(entropy: &Salt, username: &str) -> Vec<u8> {
    let mut salt = Vec::with_capacity(entropy.len() + username.as_bytes().len());
    salt.extend(entropy);
    salt.extend(username.as_bytes());
    salt
}
