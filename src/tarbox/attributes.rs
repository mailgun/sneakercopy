use sodiumoxide::crypto::{
    pwhash::scryptsalsa208sha256::SALTBYTES,
    secretbox::xsalsa20poly1305::NONCEBYTES,
};

pub type NonceBytes = [u8; NONCEBYTES];
pub type SaltBytes = [u8; SALTBYTES];

#[derive(Clone, Debug)]
pub struct Attributes {
    nonce: NonceBytes,
    salt: SaltBytes,
}

impl Attributes {
    fn new(crypto_nonce: NonceBytes, kdf_salt: SaltBytes) -> Attributes {
        Attributes {
            nonce: crypto_nonce,
            salt: kdf_salt,
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        let b = Vec::new();
        b.extend(self.nonce.to_bytes().into_iter());
        b.extend(self.salt.to_bytes().into_iter());
        return b;
    }
}
