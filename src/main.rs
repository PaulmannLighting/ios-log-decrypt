use clap::Parser;
use clap_stdin::FileOrStdin;
use ios_log_decrypt::EncryptedLog;
use log::error;
use rpassword::prompt_password;
use std::io::{stdout, Write};
use std::process::exit;

const KEY_SIZE: usize = 32;

#[derive(Debug, Parser)]
struct Args {
    #[arg(index = 1, help = "path to the encrypted log file")]
    logfile: FileOrStdin,
    #[arg(long, short, help = "hexadecimal decryption key")]
    key: Option<String>,
}

fn main() {
    env_logger::init();

    let args = Args::parse();
    let encrypted_log = EncryptedLog::new(args.logfile.to_string());
    let key: [u8; KEY_SIZE] = hex::decode(args.key.unwrap_or_else(|| {
        prompt_password("Decryption key: ").unwrap_or_else(|error| {
            error!("{error}");
            exit(1)
        })
    }))
    .unwrap_or_else(|error| {
        error!("{error}");
        exit(2);
    })
    .try_into()
    .unwrap_or_else(|vec: Vec<u8>| {
        error!("Invalid key size: {}", vec.len());
        exit(3);
    });

    for block in encrypted_log.decrypt(&key.into()) {
        match block {
            Ok(ref bytes) => stdout().write_all(bytes).expect("could not write bytes"),
            Err(error) => error!("{error}"),
        }
    }
}
