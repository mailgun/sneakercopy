use sodiumoxide::crypto::secretbox;

use super::{errors, tarbox, BufResult};

pub fn encrypt_buffer(buf: &Vec<u8>, secret: &tarbox::TarboxSecret) -> BufResult {
    Ok(secretbox::seal(
        buf.as_slice(),
        &secret.nonce(),
        &secret.key(),
    ))
}

pub fn decrypt_buffer(buf: &Vec<u8>, secret: &tarbox::TarboxSecret) -> BufResult {
    secretbox::open(buf.as_slice(), &secret.nonce(), &secret.key())
        .or_else(|_| bail!(errors::ErrorKind::SecretBoxOpenFail))
}
