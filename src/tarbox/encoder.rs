use std::io;
use std::io::{ Write };

use super::Attributes;

const TARBOX_MAGIC: [u8; 2] = [0x7A, 0xB0];

fn system_byteness() -> u8 {
    #[cfg(target_arch = "x86")]
    return 4;
    #[cfg(target_arch = "x86_64")]
    return 8;
}

#[derive(Clone, Debug)]
pub struct Encoder {
    inner: Vec<u8>,
    content_size: usize,
    attributes: Option<Attributes>,
}

impl Encoder {
    fn new() -> Encoder {
        Encoder {
            inner: Vec::new(),
            content_size: 0,
            attributes: None,
        }
    }

    /// Writes the tarbox header and then the wrapped content.
    /// The header will look like the following:
    /// +--------------+------------+----------+-------+------------+
    /// | TARBOX MAGIC | SYS USZ SZ | ATTRS SZ | ATTRS | CONTENT SZ |
    /// +--------------+------------+----------+-------+------------+
    /// |    [u8; 2]   |     u8     |  usize   | [u8]  |   usize    |
    /// +--------------+------------+----------+-------+------------+
    fn unwrap(self) -> Vec<u8> {
        let final_buf = Vec::new();
        final_buf.extend_from_slice(TARBOX_MAGIC);
        final_buf.append(system_byteness());

        if let Some(attrs) = self.attributes {
            let attr_bytes = attrs.to_bytes();
            final_buf.append(attr_bytes.len());
            final_buf.extend(attr_bytes.into_iter());
        } else {
            final_buf.append(0 as usize);
        }

        final_buf.extend(self.content_size.to_le().to_bytes().iter());
        final_buf.extend(self.inner.into_iter());

        final_buf
    }
}

impl Write for Encoder {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let bytes_len = buf.len();

        self.content_size += bytes_len;
        self.inner.extend(buf);

        Ok(bytes_len)
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_encoder() {
        let enc = Encoder::new();
    }
}