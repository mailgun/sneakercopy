use std::io;
use std::io::{ Write };

use super::{
    Attributes,
    TARBOX_MAGIC,
};

#[derive(Clone, Debug)]
pub struct Encoder {
    inner: Vec<u8>,
    attributes: Attributes,
}

impl Encoder {
    /// Returns a new Encoder including a new, empty `AttributesV1` instance.
    pub fn new(attrs: Attributes) -> Encoder {
        Encoder {
            inner: Vec::new(),
            attributes: attrs,
        }
    }

    pub fn attributes(self) -> Attributes {
        self.attributes
    }

    /// Writes the tarbox header and then the wrapped content.
    /// The header will look like the following:
    /// 
    /// ```text
    /// +--------------+--------+-------+-----+
    /// | TARBOX MAGIC |  VERS  | ATTRS | NUL |
    /// +--------------+--------+-------+-----+
    /// |    [u8; 2]   |   u8   | [u8]  | u8  |
    /// +--------------+--------+-------+-----+
    /// ```
    pub fn finish(self) -> Vec<u8> {
        let mut final_buf = Vec::new();
        final_buf.extend_from_slice(&TARBOX_MAGIC);

        // Push header version
        let attrs_version = Attributes::version();
        final_buf.push(attrs_version);

        // Push attributes including version header
        let attrs_data = self.attributes.to_bytes().unwrap();
        final_buf.extend(attrs_data.iter());

        // Push the end of header byte
        final_buf.push(0);

        final_buf.extend(self.inner.into_iter());
        return final_buf;
    }
}

impl Write for Encoder {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let bytes_len = buf.len();
        self.inner.extend(buf);
        Ok(bytes_len)
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::io::Write;
    use super::{
        Attributes,
        Encoder,
        TARBOX_MAGIC,
    };

    #[test]
    fn test_encoder() {
        let attrs = Attributes::empty();
        let mut enc = Encoder::new(attrs.clone());

        let inner: [u8; 2] = [0xca, 0xfe];
        assert_eq!(2, enc.write(&inner).unwrap());

        let version = Attributes::version();
        let data = enc.finish();

        let mut expected_payload = Vec::new();
        expected_payload.extend(&TARBOX_MAGIC);
        expected_payload.push(version);
        expected_payload.extend(attrs.to_bytes().unwrap().as_slice());
        expected_payload.extend(&[0x0, 0xca, 0xfe]);
        let expected_payload = expected_payload.as_slice();

        assert_eq!(expected_payload, data.as_slice());
    }
}