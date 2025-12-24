# Guessing Game

A small command-line Rust program that asks the player to guess a randomly generated number between 1 and 100.

This is a compact example project used for learning Rust and the basics of stdin/stdout interaction.

## Requirements

- Rust toolchain (rustc and cargo). Install from https://rustup.rs/

## Build

From the project root (where `Cargo.toml` lives):

```
cargo build --release
```

This builds an optimized executable in `target/release/`.

## Run

During development you can run the program with:

```
cargo run
```

Or run the already-built binary directly:

- Debug: `target/debug/guessing_game.exe` (Windows)
- Release: `target/release/guessing_game.exe` (Windows)

## How it works

- The program picks a secret number between 1 and 100.
- You type guesses into stdin.
- The program responds with `Too small!`, `Too big!`, or `You win!` when you guess correctly.

## Contributing

Small, focused PRs are welcome. Keep changes simple and add tests where appropriate.

## License

This repository contains example code — choose a license that fits your needs.

