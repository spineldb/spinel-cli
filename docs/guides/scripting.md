# Guide: Scripting and Output Formats

One of the most powerful features of Spinel CLI is its ability to integrate with shell scripts and other command-line tools. This is made possible by its support for non-interactive execution and multiple output formats.

## Non-Interactive Mode

To execute a single command and immediately exit, simply pass the command as arguments to `spinel-cli`:

```sh
$ spinel-cli PING
"PONG"

$ spinel-cli SET mykey "hello from script"
"OK"

$ spinel-cli GET mykey
"hello from script"
```

## Output Formats

Spinel CLI supports three output formats, which can be controlled with flags.

### Default (Pretty-Printed Text)

This is the default format, designed for human readability.

```sh
$ spinel-cli HGETALL user:1
1) "name"
2) "Alice"
3) "email"
4) "alice@example.com"
```

### JSON (`--json`)

The `--json` flag formats the output as a JSON object or value. This is extremely useful for programmatic processing, especially with tools like `jq`.

**Example: Get a user's name using `jq`**
```sh
$ spinel-cli --json HGETALL user:1 | jq -r '.name'
Alice
```

**Example: Get all values from a list as a JSON array**
```sh
$ spinel-cli --json LRANGE mylist 0 -1
[
  "item1",
  "item2",
  "item3"
]
```

### CSV (`--csv`)

The `--csv` flag formats array replies as a single, comma-separated line.

**Example: Get multiple values as a CSV row**
```sh
$ spinel-cli --csv MGET key1 key2 key3
"value1","value2","value3"
```

## Pipe Mode (`--pipe`)

The `--pipe` flag allows you to feed a sequence of commands to `spinel-cli` via standard input. This is more efficient than running `spinel-cli` for each command, as it uses a single connection.

Each command must be on a new line.

```sh
$ printf "SET name Spinel\nGET name\nINCR counter" | spinel-cli --pipe
"OK"
"Spinel"
(integer) 1
```

This mode is ideal for bulk-loading data or running a series of maintenance commands from a script.
