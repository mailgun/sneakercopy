use base64;
use sodiumoxide::crypto::pwhash;
pub use sodiumoxide::crypto::pwhash::scryptsalsa208sha256::{ SALTBYTES, Salt };
use sodiumoxide::crypto::secretbox;
pub use sodiumoxide::crypto::secretbox::xsalsa20poly1305::{ KEYBYTES, NONCEBYTES, Key, Nonce };
use std::convert::{ Into, TryFrom };

use super::errors;

#[derive(Clone, Debug)]
pub struct TarboxSecret {
    key: Key,
    nonce: Nonce,
    salt: Salt,
}

impl TarboxSecret {
    /// Create a `TarboxSecret` with known values.
    pub fn new(key: Key, nonce: Nonce, salt: Salt) -> TarboxSecret {
        TarboxSecret {
            key: key,
            nonce: nonce,
            salt: salt,
        }
    }

    /// Make a brand new _random_ `TarboxSecret` to use for encrypting a tarbox.
    pub fn generate() -> TarboxSecret {
        TarboxSecret {
            key: secretbox::gen_key(),
            nonce: secretbox::gen_nonce(),
            salt: pwhash::gen_salt(),
        }
    }

    pub fn key(&self) -> &Key {
        &self.key
    }

    pub fn nonce(&self) -> &Nonce {
        &self.nonce
    }

    pub fn salt(&self) -> &Salt {
        &self.salt
    }
}

impl TryFrom<String> for TarboxSecret {
    type Error = errors::Error;

    /// Load a `BoxSecret` from a key data string.
    /// A key data string takes the form of "<key>.<nonce>".
    fn try_from(kd: String) -> errors::Result<TarboxSecret> {
        unimplemented!();
    }

}

impl Into<String> for TarboxSecret {
    fn into(self) -> String {
        let key = base64::encode(&self.key.0);
        let nonce = base64::encode(&self.nonce.0);
        String::from(format!("{}.{}", key, nonce))
    }
}