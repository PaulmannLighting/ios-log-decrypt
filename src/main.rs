use clap::Parser;
use ios_log_decrypt::EncryptedLog;
use log::error;
use std::fs::read_to_string;
use std::io::{stdout, Write};
use std::path::PathBuf;
use std::process::exit;

#[derive(Debug, Parser)]
struct Args {
    #[arg(index = 1, help = "path to the encrypted log file")]
    filename: PathBuf,
    #[arg(long, short, help = "hexadecimal decryption key")]
    key: String,
}

fn main() {
    env_logger::init();

    let args = Args::parse();
    let encrypted_log = EncryptedLog::new(read_to_string(&args.filename).unwrap_or_else(|error| {
        error!("{error}");
        exit(1);
    }));
    let key = hex::decode(&args.key).unwrap_or_else(|error| {
        error!("{error}");
        exit(2);
    });

    for block in encrypted_log.decrypt(&key) {
        match block {
            Ok(ref bytes) => stdout().write_all(bytes).expect("could not write bytes"),
            Err(error) => error!("{error}"),
        }
    }
}
