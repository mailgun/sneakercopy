//! This module defines a thin file container for
//! holding tarbox metadata.

use std::io::{ Read, Write };

pub mod attributes;
pub mod errors;
pub mod decoder;
pub mod encoder;
pub mod secret;

pub use self::attributes::Attributes;
pub use self::decoder::Decoder;
pub use self::encoder::Encoder;
pub use self::secret::{ TarboxSecret, TarboxSecretBuilder };

pub const TARBOX_MAGIC: [u8; 2] = [0x7a, 0xb0];

/// Wraps a buffer with a tarbox header, encoding a crypto salt and nonce into
/// the attributes field of the header.
pub fn wrap_buffer(buf: &Vec<u8>, secret: &TarboxSecret) -> errors::Result<Vec<u8>> {
    let mut enc = Encoder::new(secret.into());
    enc.write_all(buf.as_slice())?;
    Ok(enc.finish())
}

/// Unwraps any attributes stored in the tarbox header and returns
/// the wrapped body along with all attributes.
pub fn unwrap_buffer(buf: &Vec<u8>) -> errors::Result<(Vec<u8>, Attributes)> {
    // Let's try and optimize this more later, but we should try
    // to minimize copying wherever possible
    let mut dec = Decoder::new(buf.to_vec())?;

    let mut buf = Vec::new();
    let read_size = dec.read_to_end(&mut buf)?;
    debug!("read {} bytes into buf", read_size);

    Ok((buf, dec.attributes_into()))
}