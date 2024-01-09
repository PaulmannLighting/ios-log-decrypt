# ios-log-decrypt
Decrypt iOS logs

## Installation
You need to have a [Rust toolchain](https://www.rust-lang.org/) installed.

```
$ git clone https://github.com/PaulmannLighting/ios-log-decrypt.git
$ cd ios-log-decrypt
$ cargo build --release
```

The built binary can be found under `target/release/ios-log-decrypt{,.exe}`.

## Contribution guidelines
* Use `cargo fmt`
* Use `cargo clippy -- -W clippy::pedantic -W clippy::nursery -W clippy::unwrap_used -W clippy::cargo`