use std::io;
use std::io::{ Read };

use super::{
    TARBOX_MAGIC,
    attributes::Attributes,
    errors,
};

#[derive(Clone, Debug)]
pub struct Decoder {
    inner: Vec<u8>,
}

impl Decoder {
    pub fn new() -> Decoder {
        Decoder {
            inner: Vec::new(),
        }
    }

    /// Processes the header and attributes off the inner stream
    /// and unwraps it, returning the inner stream to the caller
    pub fn finish(mut self) -> errors::Result<(Vec<u8>, Attributes)> {
        // Before we can unwrap the stream to the data chunk,
        // we need to read the magic bytes off the front and
        // determine which header version was in use.

        // Read and check the magic bytes
        let prelude: Vec<_> = self.inner.drain(..2).collect();
        let prelude = prelude.as_slice();
        if prelude != TARBOX_MAGIC {
            let mut actual: [u8; 2] = Default::default();
            actual.copy_from_slice(&prelude);
            bail!(errors::ErrorKind::HeaderMismatch(TARBOX_MAGIC, actual));
        }

        // Get the version byte
        let version = self.inner.remove(0);
        if version != Attributes::version() {
            bail!(errors::ErrorKind::VersionMismatch(Attributes::version(), version));
        }

        // Now that we have verified the version of the header,
        // read all bytes until a `NUL` is encountered.
        let attrs_data: Vec<_> = self.inner.clone()
            .into_iter()
            .take_while(|b| *b != 0x0)
            .collect();
        let attrs = Attributes::from_bytes(attrs_data.clone())?;
        self.inner.drain(..attrs_data.len());

        // The next byte we read should be a `NUL`.
        let next = self.inner.remove(0);
        if next != 0x0 {
            bail!(errors::ErrorKind::ExpectedNullByte(next));
        }

        // The remaining data is the encrypted archive.
        // Unwrap it and pass ownership out.
        Ok((self.inner.into_iter().collect(), attrs))
    }
}

impl Read for Decoder {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let size = buf.len();
        self.inner.extend(buf.iter());
        Ok(size)
    }
}

#[cfg(test)]
mod tests {
    use std::io::Read;
    use super::{
        errors,
        Decoder,
        TARBOX_MAGIC,
    };
    use ::tarbox::secret::{
        KEYBYTES, NONCEBYTES, SALTBYTES,
        Key, Nonce, Salt,
        TarboxSecret,
    };

    fn make_tarbox_secret() -> TarboxSecret {
        let key = Key::from_slice(&[0xca; KEYBYTES]).unwrap();
        let nonce = Nonce::from_slice(&[0xfe; NONCEBYTES]).unwrap();
        let salt = Salt::from_slice(&[0xba; SALTBYTES]).unwrap();
        TarboxSecret::new(key, nonce, salt)
    }

    fn make_header(secret: &TarboxSecret) -> Vec<u8> {
        let mut payload: Vec<u8> = Vec::new();
        payload.extend(&TARBOX_MAGIC);
        payload.push(0x1);
        payload.extend(secret.nonce().0.into_iter());
        payload.extend(secret.salt().0.into_iter());
        payload.push(0x0); // NUL -- end of header
        payload
    }

    #[test]
    fn test_decoder() {
        let secret = make_tarbox_secret();

        // Manually construct a payload to decode
        let mut payload = make_header(&secret);
        payload.extend(&[0xfa, 0xce]);

        // Create a decoder and read it in
        let mut dec = Decoder::new();
        let expected_size = payload.len();
        let actual_size = dec.read(payload.as_mut_slice()).expect("error reading data into decoder");
        assert_eq!(actual_size, expected_size);

        let (data, attrs) = dec.finish().unwrap();
        assert_eq!(data.as_slice(), &[0xfa, 0xce]);
        assert_eq!(attrs.nonce(), &[0xfe; NONCEBYTES]);
        assert_eq!(attrs.salt(), &[0xba; SALTBYTES]);
    }

    #[test]
    fn test_decode_missing_header_delimiter() {
        let secret = make_tarbox_secret();

        // Manually construct a payload to decode
        let mut payload = make_header(&secret);
        assert_eq!(payload.pop().unwrap(), 0x0);
        payload.extend(&[0xfa, 0xce]);

        // Create a decoder and read it in
        let mut dec = Decoder::new();
        let expected_size = payload.len();
        let actual_size = dec.read(payload.as_mut_slice()).expect("error reading data into decoder");
        assert_eq!(actual_size, expected_size);

        let res = dec.finish();
        assert!(res.is_err());
        let err = res.unwrap_err();
        if let errors::Error(errors::ErrorKind::SourceNotFullyDrained(num), _) = err {
            assert_eq!(2, num, "expected undrained data to be length of inner file");
        } else {
            panic!(format!("expected `SourceNotFullyDrained` error, got: {:?}", err));
        }
    }
}