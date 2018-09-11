use std::fs::File;
use std::path::PathBuf;
use std::vec::Vec;
use tar;

use super::{errors, BufResult};

pub fn pack_archive(src: &PathBuf) -> BufResult {
    let mut archive = tar::Builder::new(Vec::new());
    let file_name = src.file_name().unwrap();

    if src.is_dir() {
        debug!("recursively adding contents of {:?} to archive", file_name);
        archive.append_dir_all(".", &src)?;
    } else {
        let mut src_file = File::open(src.clone())?;
        archive.append_file(PathBuf::from(file_name), &mut src_file)?;
    }

    archive.into_inner().map_err(|e| e.into())
}

pub fn unpack_archive(buf: &Vec<u8>, dest: &PathBuf) -> errors::Result<()> {
    let mut archive = tar::Archive::new(buf.as_slice());
    archive.unpack(&dest)?;

    Ok(())
}
