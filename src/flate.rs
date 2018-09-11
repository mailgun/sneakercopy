use libflate::gzip::{Decoder, Encoder};
use std::io::{Read, Write};
use std::vec::Vec;

use super::BufResult;

pub fn compress_buffer(buf: &Vec<u8>) -> BufResult {
    let mut compressor = Encoder::new(Vec::new())?;
    compressor.write_all(buf.as_slice())?;

    debug!("write {} bytes into compressor", buf.len());

    // Finish the compression stream
    let (buf, err) = compressor.finish().unwrap();
    if let Some(e) = err {
        bail!(e);
    }

    Ok(buf)
}

pub fn inflate_buffer(buf: &Vec<u8>) -> BufResult {
    let mut inflater = Decoder::new(buf.as_slice())?;

    let mut outbuf = Vec::new();
    let read_sz = inflater.read_to_end(&mut outbuf)?;

    debug!("read {} bytes from inflater", read_sz);

    // Finish the inflation stream
    Ok(outbuf)
}
