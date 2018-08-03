//! This module defines a thin file container for
//! holding tarbox metadata.

use std::io::{ Read, Write };
use std::vec::Vec;

use super::{ BufResult, errors };

pub mod attributes;
pub mod decoder;
pub mod encoder;

pub use self::attributes::Attributes;
pub use self::decoder::Decoder;
pub use self::encoder::Encoder;