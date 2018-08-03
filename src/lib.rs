#![recursion_limit = "1024"]
#![feature(try_from)]
#![feature(int_to_from_bytes)]

extern crate base64;
#[macro_use] extern crate error_chain;
extern crate libflate;
#[macro_use] extern crate log;
extern crate sodiumoxide;
extern crate tar;

pub mod crypt;
pub mod errors;
pub mod flate;
pub mod pack;
pub mod tarbox;

use std::ffi::OsStr;
use std::fs::{ DirBuilder, File, OpenOptions };
use std::io::prelude::*;
use std::path::PathBuf;

pub type BufResult = errors::Result<Vec<u8>>;

/// Given a `path`, reads the resulting file or directory into a
/// `tar` archive, encrypts the resulting archive, removes the unencrypted
/// archive, and then compresses the encrypted archive, resulting in
/// a "tarbox".
pub fn seal_path(path: &PathBuf) -> errors::Result<crypt::BoxSecret> {
    let extension = path.extension().unwrap_or(OsStr::new(""));
    let mut extension = extension.to_os_string().into_string().unwrap();

    // Build the _final_ extension
    if extension == "" {
        extension = "tarbox".to_string();
    } else {
        extension = format!("{}.tarbox", extension);
    }

    // Build the target path
    let mut target_path = path.clone();
    target_path.set_extension(extension);

    let mut target_file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(false)
        .open(target_path)?;

    // Make a new `BoxSecret`
    let secret = crypt::BoxSecret::generate();

    // Pack the target files to the tar archive
    debug!("packing path {:?} to archive buffer", path);
    let buf = pack::pack_archive(&path)?;

    debug!("compressing buf of length {}", buf.len());
    let buf = flate::compress_buffer(&buf)?;

    debug!("encrypting compressed buf (size {})", buf.len());
    let buf = crypt::encrypt_buffer(&buf, &secret)?;

    target_file.write_all(&buf)?;

    Ok(secret)
}

pub fn unseal_path(path: &PathBuf, dest: &PathBuf, secret: &crypt::BoxSecret) -> errors::Result<()> {
    DirBuilder::new()
        .recursive(true)
        .create(&dest)?;

    let mut source_file = File::open(path)?;
    let source_meta = source_file.metadata()?;
    debug!("reading {} bytes from tarbox: {:?}", source_meta.len(), path);

    let mut buf = Vec::new();
    source_file.read_to_end(&mut buf)?;

    debug!("decrypting compressed buf (size {})", buf.len());
    let buf = crypt::decrypt_buffer(&buf, &secret)?;

    debug!("inflating buf of length {}", buf.len());
    let buf = flate::inflate_buffer(&buf)?;

    debug!("unpacking archive to path: {:?}", dest);
    pack::unpack_archive(&buf, &dest)?;

    Ok(())
}