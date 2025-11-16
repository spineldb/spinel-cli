# Spinel CLI

Spinel CLI is a modern, user-friendly command-line interface for interacting with SpinelDB (or any Redis-compatible) servers. Written in Rust, it's designed for performance, safety, and a superior user experience compared to traditional database CLIs.

[![GitHub release](https://img.shields.io/github/v/release/spineldb/spinel-cli)](https://github.com/spineldb/spinel-cli/releases/latest)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

## Features

- **Interactive REPL**: A powerful interactive shell with command history and support for connection management within the CLI.
- **Multiple Output Formats**: Get your data in the format you need. Spinel CLI supports standard pretty-printed text, **JSON** (`--json`), and **CSV** (`--csv`).
- **User-Friendly Display**: Responses are formatted for human readability, with clear type indicators (e.g., `(integer)`) and pretty-printing for arrays.
- **Modern Tech Stack**: Built with Rust for high performance and memory safety.
- **Redis-CLI Compatibility**: Supports familiar modes of operation like `--pipe`, `--scan`, and `--latency`.
- **Secure Password Entry**: Prompts for a password when using the `--user` flag without `--password` to avoid exposing credentials in your shell history.

## Quick Install

### Using curl

```bash
sh -c "$(curl -fsSL https://raw.githubusercontent.com/spineldb/spinel-cli/main/install.sh)"
```

### Using wget

```bash
sh -c "$(wget -qO- https://raw.githubusercontent.com/spineldb/spinel-cli/main/install.sh)"
```

## Usage

### Interactive Mode (REPL)

For an interactive session, simply run `spinel-cli` without any commands.

```bash
spinel-cli -H my.database.host -p 7878
```

Once inside the REPL, you can run commands directly:

```
127.0.0.1:7878> PING
"PONG"
127.0.0.1:7878> SET mykey "Hello, Spinel!"
"OK"
127.0.0.1:7878> GET mykey
"Hello, Spinel!"
127.0.0.1:7878> HELP GET

  GET key
  summary: Get the value of a key
  since: 1.0.0
  group: string

127.0.0.1:7878> exit
```

### Non-Interactive Mode

Execute a single command and exit. This is useful for scripting.

```bash
$ spinel-cli GET mykey
"Hello, Spinel!"
```

### Using Different Output Formats

The `--json` flag is perfect for piping output to tools like `jq`.

```bash
$ spinel-cli --json HGETALL myhash
{
  "field1": "value1",
  "field2": "value2"
}
```

The `--csv` flag is useful for generating comma-separated data.

```bash
$ spinel-cli --csv MGET key1 key2
"value1","value2"
```

### Pipe Mode

You can pipe commands into `spinel-cli` using the `--pipe` flag.

```bash
$ echo "SET key1 123\nGET key1" | spinel-cli --pipe
"OK"
"123"
```

## Documentation

For a comprehensive guide, command reference, and contribution guidelines, please see our **[Full Documentation Hub](./docs/introduction.md)**.
