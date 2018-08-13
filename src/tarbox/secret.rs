use base64;
use sodiumoxide::crypto::pwhash;
pub use sodiumoxide::crypto::pwhash::scryptsalsa208sha256::{ SALTBYTES, Salt };
use sodiumoxide::crypto::secretbox;
pub use sodiumoxide::crypto::secretbox::xsalsa20poly1305::{ KEYBYTES, NONCEBYTES, Key, Nonce };

pub fn decode_key(key: String) -> Option<Key> {
    let bytes = base64::decode(&key).unwrap();
    Key::from_slice(bytes.as_slice())
}

pub fn decode_nonce(nonce: String) -> Option<Nonce> {
    let bytes = base64::decode(&nonce).unwrap();
    Nonce::from_slice(bytes.as_slice())
}

pub fn decode_salt(salt: String) -> Option<Salt> {
    let bytes = base64::decode(&salt).unwrap();
    Salt::from_slice(bytes.as_slice())
}

builder!(pub : TarboxSecretBuilder => TarboxSecret {
    key: Key = None,
    nonce: Nonce = None,
    salt: Salt = None
});

impl TarboxSecret {
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

    pub fn encoded_key(&self) -> String {
        String::from(base64::encode(&self.key.0))
    }

    pub fn nonce(&self) -> &Nonce {
        &self.nonce
    }

    pub fn encoded_nonce(&self) -> String {
        String::from(base64::encode(&self.nonce.0))
    }

    pub fn salt(&self) -> &Salt {
        &self.salt
    }

    pub fn encoded_salt(&self) -> String {
        String::from(base64::encode(&self.salt.0))
    }
}