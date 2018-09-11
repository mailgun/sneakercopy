#![recursion_limit = "1024"]
#![feature(int_to_from_bytes)]

extern crate base64;
#[macro_use]
extern crate error_chain;
extern crate libflate;
#[macro_use]
extern crate log;
extern crate rand;
extern crate sodiumoxide;
extern crate spinners;
extern crate tar;

#[macro_use]
mod builder;
pub mod crypt;
pub mod errors;
pub mod flate;
pub mod pack;
pub mod password;
pub mod tarbox;

use spinners::{Spinner, Spinners};
use std::env;
use std::ffi::OsStr;
use std::fs::{DirBuilder, File, OpenOptions};
use std::io::prelude::*;
use std::path::PathBuf;

pub type BufResult = errors::Result<Vec<u8>>;

fn build_output_file_name(path: &PathBuf) -> PathBuf {
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

    PathBuf::from(target_path.file_name().unwrap())
}

fn build_output_path(input: &PathBuf, output: &Option<PathBuf>) -> PathBuf {
    let target_file_name = build_output_file_name(input);
    let output = output.clone().unwrap_or(env::current_dir().unwrap());
    if output.is_dir() {
        return output.join(target_file_name);
    }

    output
}

/// Given a `path`, reads the resulting file or directory into a
/// `tar` archive, encrypts the resulting archive, removes the unencrypted
/// archive, and then compresses the encrypted archive, resulting in
/// a "tarbox".
pub fn seal_path(
    path: &PathBuf,
    output: &Option<PathBuf>,
    force: bool,
) -> errors::Result<tarbox::TarboxSecret> {
    let target_path = build_output_path(path, output);

    let mut target_file = OpenOptions::new();
    target_file.create(true).write(true);

    if force {
        target_file.create_new(false).truncate(true);
    } else {
        target_file.create_new(true).truncate(false);
    }

    let mut target_file = target_file.open(target_path)?;

    // Make a new `BoxSecret`
    let password = password::generate_password();
    let secret = tarbox::TarboxSecret::generate(password);

    let waiter = Spinner::new(Spinners::Dots12, "Prepping...".into());

    // Pack the target files to the tar archive
    debug!("packing path {:?} to archive buffer", path);
    waiter.message("Packing...".into());
    let buf = pack::pack_archive(&path)?;

    debug!("compressing buf of length {}", buf.len());
    waiter.message("Compressing...".into());
    let buf = flate::compress_buffer(&buf)?;

    debug!("encrypting compressed buf (size {})", buf.len());
    waiter.message("Encrypting...".into());
    let buf = crypt::encrypt_buffer(&buf, &secret)?;

    debug!("finalizing tarbox (size {})", buf.len());
    waiter.message("Finishing up...".into());
    let buf = tarbox::wrap_buffer(&buf, &secret)?;

    target_file.write_all(&buf)?;
    waiter.stop();

    Ok(secret)
}

pub fn unseal_path(
    path: &PathBuf,
    dest: &PathBuf,
    sb: tarbox::TarboxSecretBuilder,
) -> errors::Result<()> {
    DirBuilder::new().recursive(true).create(&dest)?;

    let mut source_file = File::open(path)?;
    let source_meta = source_file.metadata()?;
    debug!(
        "reading {} bytes from tarbox: {:?}",
        source_meta.len(),
        path
    );

    let waiter = Spinner::new(Spinners::Dots12, "Prepping...".into());

    let mut buf = Vec::new();
    waiter.message("Reading file...".into());
    source_file.read_to_end(&mut buf)?;

    debug!("unwrapping tarbox (size {})", buf.len());
    waiter.message("Unwrapping...".into());
    let (buf, attrs) = tarbox::unwrap_buffer(&buf)?;

    let nonce = attrs.nonce();
    let salt = attrs.salt();
    let secret = sb
        .nonce(tarbox::secret::Nonce::from_slice(nonce).unwrap())
        .salt(tarbox::secret::Salt::from_slice(salt).unwrap())
        .build()?;

    debug!("decrypting compressed buf (size {})", buf.len());
    waiter.message("Decrypting...".into());
    let buf = crypt::decrypt_buffer(&buf, &secret)?;

    debug!("inflating buf of length {}", buf.len());
    waiter.message("Inflating...".into());
    let buf = flate::inflate_buffer(&buf)?;

    debug!("unpacking archive to path: {:?}", dest);
    waiter.message("Unpacking...".into());
    pack::unpack_archive(&buf, &dest)?;

    waiter.stop();

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{build_output_file_name, build_output_path};
    use std::path::PathBuf;

    #[test]
    fn test_build_output_file_name() {
        // (input, expectation)
        let results = [
            ("/tmp", "tmp.tarbox"),
            ("/tmp/test.txt", "test.txt.tarbox"),
            ("/tmp/тест.txt", "тест.txt.tarbox"),
        ];

        for (path, result) in results.iter() {
            let output_name = PathBuf::from(path);
            assert_eq!(PathBuf::from(result), build_output_file_name(&output_name));
        }
    }

    #[test]
    fn test_build_output_path() {
        // (input path, output path, expectation)
        let results = [
            ("/tmp", "/", "/tmp.tarbox"),
            ("/tmp/test.txt", "/", "/test.txt.tarbox"),
            ("/tmp/тест.txt", "/", "/тест.txt.tarbox"),
            (
                "/tmp/example.txt",
                "/usr/local",
                "/usr/local/example.txt.tarbox",
            ),
            (
                "/tmp/example.txt",
                "/usr/local/example.tarbox",
                "/usr/local/example.tarbox",
            ),
        ];

        for (input_path, output_path, result) in results.iter() {
            let input_path = PathBuf::from(input_path);
            let output_path = PathBuf::from(output_path);
            assert_eq!(
                PathBuf::from(result),
                build_output_path(&input_path, &Some(output_path))
            );
        }
    }
}
