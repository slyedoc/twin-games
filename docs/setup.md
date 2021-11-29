# Setup Notes

## Install node and pnpm

- // TODO HERE

## Install node packages

- Run ```pnpm i``` or ```pnpm install```

## Rust Setup (Frist Time)

- Install Rustup ```curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh``` accept defaults
- Run ```rustup default nightly```
- Run ```rustup target add wasm32-unknown-unknown```
- Run ```cargo update```
- Run ```cargo build --release``` to make sure everything works

## Useful Cargo commands

- ```cargo update``` if your changing crates
- ```cargo run --release --example=SOMETHING``` to run an example
- ```cargo run --release --package=SOMETHING``` to run one of the games
- ```cargo watch --clear -x "run --release --package=SOMETHING"``` for hot reloading
