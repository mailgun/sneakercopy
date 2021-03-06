#![recursion_limit = "1024"]
#![feature(try_from)]

extern crate rpassword;
#[macro_use]
extern crate quicli;
extern crate sneakercopy;
extern crate sodiumoxide;
extern crate structopt;

use quicli::prelude::*;
use std::path::PathBuf;

use sneakercopy::{errors::*, tarbox, *};

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

        #[structopt(
            short = "o",
            long = "output",
            help = "Optional output location",
            parse(from_os_str)
        )]
        output: Option<PathBuf>,

        #[structopt(
            short = "f",
            long = "force",
            help = "Force overwriting of output"
        )]
        force: bool,
    },

    #[structopt(name = "unseal", about = "Unseal an encrypted archive")]
    Unseal {
        #[structopt(help = "Path to encrypted archive", parse(from_os_str))]
        path: PathBuf,

        #[structopt(help = "Password used for encryption")]
        password: Option<String>,

        #[structopt(
            short = "C",
            long = "extract-to",
            help = "Directory to extract archive to",
            parse(from_os_str)
        )]
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
        Subcommand::Seal {
            path,
            output,
            force,
        } => seal_subcmd(&args, &path.canonicalize().unwrap(), output, force)?,
        Subcommand::Unseal {
            path,
            password,
            dest,
        } => unseal_subcmd(&args, &path.canonicalize().unwrap(), dest, password)?,
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

fn seal_subcmd(
    _args: &Cli,
    path: &PathBuf,
    output: &Option<PathBuf>,
    force: &bool,
) -> sneakercopy::errors::Result<()> {
    check_path(&path)?;

    let secret = seal_path(&path, &output, *force)?;
    println!("\nsecret: {}", secret.password());

    Ok(())
}

fn unseal_subcmd(
    _args: &Cli,
    path: &PathBuf,
    dest: &Option<PathBuf>,
    password: &Option<String>,
) -> sneakercopy::errors::Result<()> {
    check_path(&path)?;

    let password = password.clone().unwrap_or_else(|| {
        return rpassword::prompt_password_stdout("secret: ")
            .expect("can't open tarbox without a secret!");
    });

    let sb = tarbox::TarboxSecretBuilder::new();
    let sb = sb.password(password);
    let dest = dest.clone().unwrap_or(path.parent().unwrap().to_path_buf());

    unseal_path(&path, &dest, sb)?;

    Ok(())
}
