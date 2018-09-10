use std::cmp;
use std::io;
use std::io::{ Read, Write };

use super::{
    TARBOX_MAGIC,
    attributes::Attributes,
    errors,
};

#[derive(Clone, Debug)]
pub struct Decoder {
    inner: Vec<u8>,
    attrs: Attributes,
}

impl Decoder {
    pub fn new(mut inner: Vec<u8>) -> errors::Result<Decoder> {
        // Before we can unwrap the stream to the data chunk,
        // we need to read the magic bytes off the front and
        // determine which header version was in use.

        // Read and check the magic bytes
        let prelude: Vec<_> = inner.drain(..2).collect();
        let prelude = prelude.as_slice();
        if prelude != TARBOX_MAGIC {
            let mut actual: [u8; 2] = Default::default();
            actual.copy_from_slice(&prelude);
            bail!(errors::ErrorKind::HeaderMismatch(TARBOX_MAGIC, actual));
        }

        // Get the version byte
        let version = inner.remove(0);
        if version != Attributes::version() {
            bail!(errors::ErrorKind::VersionMismatch(Attributes::version(), version));
        }

        // Now that we have verified the version of the header,
        // read all bytes until a `NUL` is encountered.
        let attrs_data: Vec<_> = inner.clone()
            .into_iter()
            .take_while(|b| *b != 0x0)
            .collect();

        let attrs = Attributes::from_bytes(attrs_data.clone())?;
        inner.drain(..attrs_data.len());

        // The next byte we read should be a `NUL`.
        let next = inner.remove(0);
        if next != 0x0 {
            bail!(errors::ErrorKind::ExpectedNullByte(next));
        }

        debug!("unwrapped inner data length: {}", inner.len());

        // The remaining data in `inner` is the encrypted archive.
        // Move it into the `Decoder` instance along with `attrs`.
        Ok(Decoder {
            inner: inner,
            attrs: attrs,
        })
    }

    pub fn attributes(&self) -> &Attributes {
        &self.attrs
    }

    pub fn attributes_into(self) -> Attributes {
        self.attrs
    }
}

impl Read for Decoder {
    // Reads bytes out of the inner container into an output buffer.
    fn read(&mut self, mut outbuf: &mut [u8]) -> io::Result<usize> {
        let buffer_size = outbuf.len();
        let inner_size = self.inner.len();
        debug!("{} bytes remaining in inner", inner_size);

        let drain_size = cmp::min(inner_size, buffer_size);
        debug!("draining {} bytes from inner into outbuf", drain_size);

        let drain = self.inner.drain(..drain_size);
        debug!("drain currently holding {} bytes", drain.len());

        let written = outbuf.write(&drain.collect::<Vec<_>>())?;
        debug!("wrote {} bytes to outbuf", written);

        Ok(written)
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
    use ::password;
    use ::tarbox::secret::{
        NONCEBYTES, SALTBYTES,
        Nonce, Salt,
        TarboxSecret,
        TarboxSecretBuilder,
    };

    fn make_tarbox_secret() -> TarboxSecret {
        let nonce = Nonce::from_slice(&[0xfe; NONCEBYTES]).unwrap();
        let salt = Salt::from_slice(&[0xba; SALTBYTES]).unwrap();
        TarboxSecretBuilder::new()
            .password(password::generate_password())
            .nonce(nonce)
            .salt(salt)
            .build().unwrap()
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

        // Create a decoder and read the inner data from it
        let mut dec = Decoder::new(payload).unwrap();
        let mut data = Vec::new();
        let actual_size = dec.read_to_end(&mut data).expect("error reading data into decoder");
        let attrs = dec.attributes_into();

        assert_eq!(2, actual_size);
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

        // Create a decoder and read the inner data from it
        let res = Decoder::new(payload);

        assert!(res.is_err());
        let err = res.unwrap_err();
        if let errors::Error(errors::ErrorKind::SourceNotFullyDrained(num), _) = err {
            assert_eq!(2, num, "expected undrained data to be length of inner file");
        } else {
            panic!(format!("expected `SourceNotFullyDrained` error, got: {:?}", err));
        }
    }
}