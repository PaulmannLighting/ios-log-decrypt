use clap::Parser;
use clap_stdin::FileOrStdin;
use generic_array::{ArrayLength, GenericArray};
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
    let encrypted_log = EncryptedLog::new(args.logfile.to_string());

    for block in encrypted_log.decrypt(
        generic_array_try_from_slice(args.key().as_slice()).unwrap_or_else(|| {
            error!("Invalid key size.");
            exit(3);
        }),
    ) {
        match block {
            Ok(ref bytes) => stdout().write_all(bytes).expect("could not write bytes"),
            Err(error) => error!("{error}"),
        }
    }
}

/// Convert slices to `GenericArray`s without panicking.
fn generic_array_try_from_slice<T, N>(data: &[T]) -> Option<&GenericArray<T, N>>
where
    N: ArrayLength<T>,
{
    if data.len() == N::to_usize() {
        Some(data.into())
    } else {
        None
    }
}
