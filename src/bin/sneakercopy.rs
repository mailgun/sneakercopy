#![recursion_limit = "1024"]
#![feature(try_from)]

#[macro_use] extern crate quicli;
extern crate sneakercopy;
extern crate sodiumoxide;
#[macro_use] extern crate structopt;

use quicli::prelude::*;
use std::convert::TryFrom;
use std::path::PathBuf;

use sneakercopy::{
    *,
    errors::*,
};

#[derive(Debug, StructOpt)]
#[structopt(raw(setting = "structopt::clap::AppSettings::ColoredHelp"))]
struct Cli {
    #[structopt(flatten)]
    verbosity: Verbosity,

    #[structopt(subcommand)]
    subcmd: Subcommand,
}

#[derive(Debug, StructOpt)]
enum Subcommand {
    #[structopt(name = "seal", about = "Seal an encrypted archive")]
    Seal {
        #[structopt(help = "File/folder path to archive", parse(from_os_str))]
        path: PathBuf,
    },

    #[structopt(name = "unseal", about = "Unseal an encrypted archive")]
    Unseal {
        #[structopt(help = "Path to encrypted archive", parse(from_os_str))]
        path: PathBuf,

        #[structopt(help = "Key used for encryption")]
        key: String,

        #[structopt(short = "C", long = "extract-to", help = "Directory to extract archive to", parse(from_os_str))]
        dest: Option<PathBuf>,
    },
}

main!(|args: Cli, log_level: verbosity| {
    sodiumoxide::init().expect("could not init sodiumoxide lib");

    if let Err(ref e) = entrypoint(args) {
        println!("error: {}", e);
        for e in e.iter().skip(1) {
            println!("caused by: {}", e);
        }

        if let Some(backtrace) = e.backtrace() {
            println!("backtrace: {:?}", backtrace);
        }

        std::process::exit(1);
    }
});

fn entrypoint(args: Cli) -> sneakercopy::errors::Result<()> {
    let action = &args.subcmd;
    match action {
        Subcommand::Seal{ path } => seal_subcmd(&args, path)?,
        Subcommand::Unseal{ path, key, dest } => unseal_subcmd(&args, path, dest, key)?,
    }

    Ok(())
}

fn check_path(path: &PathBuf) -> sneakercopy::errors::Result<()> {
    debug!("checking path existence: {:?}", path);
    if !path.exists() {
        let path = path.to_str().unwrap();
        let path = String::from(path);
        return Err(ErrorKind::PathDoesNotExist(path).into());
    }

    Ok(())
}

fn seal_subcmd(_args: &Cli, path: &PathBuf) -> sneakercopy::errors::Result<()> {
    check_path(&path)?;
    let secret = seal_path(&path)?;

    let secret_str: String = secret.into();
    println!("secret: {}", secret_str);

    Ok(())
}

fn unseal_subcmd(_args: &Cli, path: &PathBuf, dest: &Option<PathBuf>, key: &String) -> sneakercopy::errors::Result<()> {
    check_path(&path)?;
    let secret = crypt::BoxSecret::try_from(key.clone())?;
    let dest = dest.clone().unwrap_or(path.parent().unwrap().to_path_buf());

    unseal_path(&path, &dest, &secret)?;

    Ok(())
}