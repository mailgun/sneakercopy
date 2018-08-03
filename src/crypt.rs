use base64;
use sodiumoxide::crypto::secretbox;
use sodiumoxide::crypto::secretbox::xsalsa20poly1305::{ Key, Nonce };
use std::convert::{ TryFrom };

use super::{ BufResult, errors };

#[derive(Clone, Debug)]
pub struct BoxSecret {
    key: Key,
    nonce: Nonce,
}

impl BoxSecret {
    /// Make a brand new _random_ `BoxSecret` to use for encrypting a tarbox.
    pub fn generate() -> BoxSecret {
        BoxSecret {
            key: secretbox::gen_key(),
            nonce: secretbox::gen_nonce(),
        }
    }
}

impl TryFrom<String> for BoxSecret {
    type Error = errors::Error;

    /// Load a `BoxSecret` from a key data string.
    /// A key data string takes the form of "<key>.<nonce>".
    fn try_from(kd: String) -> errors::Result<BoxSecret> {
        let split_at = kd.find(".");
        if split_at.is_none() {
            bail!(errors::ErrorKind::InvalidKeyData(kd));
        }

        let split_at = split_at.unwrap();

        let key = base64::decode(&String::from(&kd[..split_at]))?;
        let key = key.as_slice();

        let nonce = base64::decode(&String::from(&kd[split_at + 1..]))?;
        let nonce = nonce.as_slice();

        // Create Key/Nonce structs from the split KD
        let key = Key::from_slice(&key);
        let nonce = Nonce::from_slice(&nonce);

        Ok(BoxSecret {
            key: key.unwrap(),
            nonce: nonce.unwrap(),
        })
    }

}

impl Into<String> for BoxSecret {
    fn into(self) -> String {
        let key = base64::encode(&self.key.0);
        let nonce = base64::encode(&self.nonce.0);
        String::from(format!("{}.{}", key, nonce))
    }
}

pub fn encrypt_buffer(buf: &Vec<u8>, secret: &BoxSecret) -> BufResult {
    Ok(secretbox::seal(buf.as_slice(), &secret.nonce, &secret.key))
}

pub fn decrypt_buffer(buf: &Vec<u8>, secret: &BoxSecret) -> BufResult {
    let res = secretbox::open(buf.as_slice(), &secret.nonce, &secret.key);
    if res.is_err() {
        bail!(errors::ErrorKind::SecretBoxOpenFail);
    }

    Ok(res.unwrap())
}