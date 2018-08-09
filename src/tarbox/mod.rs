//! This module defines a thin file container for
//! holding tarbox metadata.

pub mod attributes;
pub mod errors;
pub mod decoder;
pub mod encoder;
pub mod secret;

pub use self::attributes::Attributes;
pub use self::decoder::Decoder;
pub use self::encoder::Encoder;
pub use self::secret::TarboxSecret;

pub const TARBOX_MAGIC: [u8; 2] = [0x7a, 0xb0];

pub fn wrap_buffer(buf: &Vec<u8>, secret: &TarboxSecret) -> errors::Result<Vec<u8>> {
    Ok(Vec::new())
}

pub fn unwrap_buffer(buf: &Vec<u8>) -> errors::Result<(Vec<u8>, TarboxSecret)> {
    Ok((Vec::new(), TarboxSecret::generate()))
}