use clap::Parser;
use clap_stdin::FileOrStdin;
use ios_log_decrypt::EncryptedLog;
use log::error;
use rpassword::prompt_password;
use std::io::{stdout, Write};
use std::process::exit;

#[derive(Debug, Parser)]
struct Args {
    #[arg(index = 1, help = "path to the encrypted log file")]
    logfile: FileOrStdin,
    #[arg(long, short, help = "hexadecimal decryption key")]
    key: Option<String>,
}

impl Args {
    #[must_use]
    pub fn hex_key(&self) -> String {
        self.key.clone().unwrap_or_else(|| {
            prompt_password("Decryption key: ").unwrap_or_else(|error| {
                error!("{error}");
                exit(1)
            })
        })
    }

    #[must_use]
    pub fn key(&self) -> Vec<u8> {
        hex::decode(self.hex_key()).unwrap_or_else(|error| {
            error!("{error}");
            exit(2);
        })
    }
}

fn main() {
    env_logger::init();

    let args = Args::parse();
    let encrypted_log =
        EncryptedLog::new(args.logfile.clone().contents().unwrap_or_else(|error| {
            error!("{error}");
            exit(3)
        }));

    for block in encrypted_log.decrypt(args.key().as_slice().into()) {
        match block {
            Ok(ref bytes) => stdout().write_all(bytes).expect("could not write bytes"),
            Err(error) => error!("{error}"),
        }
    }
}
