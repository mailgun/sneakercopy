use base64;
use sodiumoxide::crypto::pwhash;
pub use sodiumoxide::crypto::pwhash::scryptsalsa208sha256::{
    Salt, MEMLIMIT_INTERACTIVE, OPSLIMIT_INTERACTIVE, SALTBYTES,
};
use sodiumoxide::crypto::secretbox;
pub use sodiumoxide::crypto::secretbox::xsalsa20poly1305::{Key, Nonce, KEYBYTES, NONCEBYTES};

pub fn decode_nonce(nonce: String) -> Option<Nonce> {
    let bytes = base64::decode(&nonce).unwrap();
    Nonce::from_slice(bytes.as_slice())
}

pub fn decode_salt(salt: String) -> Option<Salt> {
    let bytes = base64::decode(&salt).unwrap();
    Salt::from_slice(bytes.as_slice())
}

builder!(pub : TarboxSecretBuilder => TarboxSecret {
    password: String = None,
    nonce: Nonce = None,
    salt: Salt = None
});

impl TarboxSecret {
    /// Make a brand new _random_ `TarboxSecret` to use for encrypting a tarbox.
    pub fn generate(password: String) -> TarboxSecret {
        TarboxSecret {
            password: password.clone(),
            nonce: secretbox::gen_nonce(),
            salt: pwhash::gen_salt(),
        }
    }

    pub fn key(&self) -> Key {
        // derive the actual key from the password and salt
        let mut key = secretbox::Key([0; secretbox::KEYBYTES]);

        {
            let secretbox::Key(ref mut buffer) = key;
            pwhash::derive_key(
                buffer,
                self.password.as_bytes(),
                &self.salt,
                OPSLIMIT_INTERACTIVE,
                MEMLIMIT_INTERACTIVE,
            ).unwrap();
        }

        key
    }

    pub fn password(&self) -> &String {
        &self.password
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
