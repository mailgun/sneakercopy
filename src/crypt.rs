use sodiumoxide::crypto::secretbox;

use super::{ BufResult, errors, tarbox };

pub fn encrypt_buffer(buf: &Vec<u8>, secret: &tarbox::TarboxSecret) -> BufResult {
    Ok(secretbox::seal(buf.as_slice(), &secret.nonce(), &secret.key()))
}

pub fn decrypt_buffer(buf: &Vec<u8>, secret: &tarbox::TarboxSecret) -> BufResult {
    let res = secretbox::open(buf.as_slice(), &secret.nonce(), &secret.key());
    if res.is_err() {
        bail!(errors::ErrorKind::SecretBoxOpenFail);
    }

    Ok(res.unwrap())
}