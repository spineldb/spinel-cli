# Development Setup

This guide will walk you through setting up your local environment to contribute to Spinel CLI.

## 1. Install Rust

Spinel CLI is written in Rust. If you don't have Rust installed, you can install it using `rustup`.

```sh
# Install rustup (the Rust toolchain manager)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Follow the on-screen instructions
```

## 2. Clone the Repository

```sh
git clone https://github.com/spineldb/spinel-cli.git
cd spinel-cli
```

## 3. Build the Project

You can build the project using `cargo`, the Rust build tool and package manager.

```sh
# Build the project in debug mode
cargo build
```

The resulting binary will be located at `target/debug/spinel-cli`.

## 4. Run the CLI

You can run the CLI directly using `cargo run`.

### Running the REPL
```sh
# Pass -- to separate cargo's arguments from the application's arguments
cargo run -- -H <your-db-host>
```

### Executing a single command
```sh
cargo run -- PING
```

## 5. Running Tests

To run the test suite:

```sh
cargo test
```

Now you are all set to start developing!
