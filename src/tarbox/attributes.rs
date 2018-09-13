use super::{
    errors,
    secret::{TarboxSecret, NONCEBYTES, SALTBYTES},
};

pub type NonceBytes = [u8; NONCEBYTES];
pub type SaltBytes = [u8; SALTBYTES];

pub const VERSION: u8 = 0x1;

#[derive(Clone, Debug)]
pub struct Attributes {
    nonce: NonceBytes,
    salt: SaltBytes,
}

impl Attributes {
    pub fn new(crypto_nonce: NonceBytes, kdf_salt: SaltBytes) -> Attributes {
        Attributes {
            nonce: crypto_nonce,
            salt: kdf_salt,
        }
    }

    pub fn empty() -> Self {
        Attributes {
            nonce: [0; NONCEBYTES],
            salt: [0; SALTBYTES],
        }
    }

    pub fn version() -> u8 {
        VERSION
    }

    pub fn attr_block_size() -> usize {
        (NONCEBYTES + SALTBYTES) as usize
    }

    pub fn nonce(&self) -> &NonceBytes {
        &self.nonce
    }

    pub fn salt(&self) -> &SaltBytes {
        &self.salt
    }

    pub fn from_bytes(source: Vec<u8>) -> errors::Result<Attributes> {
        let expected: usize = NONCEBYTES + SALTBYTES;
        let actual: usize = source.len();
        if actual > expected {
            bail!(errors::ErrorKind::SourceTooLarge(expected, actual));
        }

        let mut nonce = [0; NONCEBYTES];
        nonce.copy_from_slice(&source[..NONCEBYTES]);

        let mut salt = [0; SALTBYTES];
        salt.copy_from_slice(&source[NONCEBYTES..NONCEBYTES + SALTBYTES]);

        Ok(Attributes::new(nonce, salt))
    }

    pub fn to_bytes(&self) -> errors::Result<Vec<u8>> {
        let mut b = Vec::new();
        b.extend(self.nonce.into_iter());
        b.extend(self.salt.into_iter());
        Ok(b)
    }
}

impl From<TarboxSecret> for Attributes {
    fn from(s: TarboxSecret) -> Self {
        Attributes {
            nonce: s.nonce().0,
            salt: s.salt().0,
        }
    }
}

impl<'a> From<&'a TarboxSecret> for Attributes {
    fn from(s: &'a TarboxSecret) -> Self {
        Attributes {
            nonce: s.nonce().0.clone(),
            salt: s.salt().0.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_data() -> (NonceBytes, SaltBytes) {
        let nonce = [0xbe; NONCEBYTES];
        let salt = [0x5a; SALTBYTES];
        return (nonce, salt);
    }

    fn make_source(nonce: NonceBytes, salt: SaltBytes) -> Vec<u8> {
        let mut source = Vec::new();
        source.extend_from_slice(&nonce);
        source.extend_from_slice(&salt);
        return source;
    }

    #[test]
    fn test_from_bytes() {
        let (nonce, salt) = make_data();
        let source = make_source(nonce, salt);

        let attrs = Attributes::from_bytes(source).unwrap();
        assert_eq!(attrs.nonce, nonce);
        assert_eq!(attrs.salt, salt);
    }

    #[test]
    fn test_to_bytes() {
        let (nonce, salt) = make_data();
        let attrs = Attributes::new(nonce, salt);

        let mut expected = Vec::new();
        expected.extend_from_slice(&nonce);
        expected.extend_from_slice(&salt);

        let encoded = attrs.to_bytes().unwrap();
        assert_eq!(encoded, expected);
    }

    #[test]
    fn test_source_unconsumed() {
        let (nonce, salt) = make_data();
        let mut source = make_source(nonce, salt);
        source.extend_from_slice(&[0xca, 0xfe]);

        let res = Attributes::from_bytes(source);
        assert!(res.is_err());
        let err = res.unwrap_err();
        if let errors::Error(errors::ErrorKind::SourceTooLarge(_, actual), _) = err {
            assert_eq!(
                58, actual,
                "only 56 bytes were expected in attrs source (2 undrained)"
            );
        } else {
            panic!(format!(
                "expected `SourceNotFullyDrained` error, got: {:?}",
                err
            ));
        }
    }
}
