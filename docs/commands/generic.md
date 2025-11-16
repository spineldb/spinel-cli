# Generic Commands

Generic commands can be used on keys of any type.

| Command | Usage | Summary |
| --- | --- | --- |
| `DEL` | `DEL key [key ...]` | Deletes one or more keys. |
| `EXISTS` | `EXISTS key [key ...]` | Checks if one or more keys exist. |
| `EXPIRE` | `EXPIRE key seconds` | Sets a key's time to live in seconds. |
| `EXPIREAT` | `EXPIREAT key timestamp` | Sets a key's expiration time to a specific Unix timestamp. |
| `KEYS` | `KEYS pattern` | Finds all keys matching the given pattern. **Warning**: May be slow on large databases. |
| `PERSIST` | `PERSIST key` | Removes the expiration from a key. |
| `PEXPIRE` | `PEXPIRE key milliseconds` | Sets a key's time to live in milliseconds. |
| `PEXPIREAT`| `PEXPIREAT key milliseconds-timestamp` | Sets a key's expiration time to a specific Unix timestamp in milliseconds. |
| `PTTL` | `PTTL key` | Gets the time to live for a key in milliseconds. |
| `RANDOMKEY`| `RANDOMKEY` | Returns a random key from the keyspace. |
| `RENAME` | `RENAME key newkey` | Renames a key. |
| `RENAMENX` | `RENAMENX key newkey` | Renames a key, only if the new key does not exist. |
| `TTL` | `TTL key` | Gets the time to live for a key in seconds. |
| `TYPE` | `TYPE key` | Determines the type of a key. |
