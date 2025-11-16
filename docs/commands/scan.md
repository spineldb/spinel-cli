# Scan Commands

The `SCAN` family of commands provides a way to iterate through the keyspace or elements of a collection without blocking the server. This is the recommended way to traverse data in a production environment.

The `cursor` returned by each call must be passed as the first argument to the next call. Iteration begins when `cursor` is 0 and terminates when the server returns a `cursor` of 0.

| Command | Usage | Summary |
| --- | --- | --- |
| `SCAN` | `SCAN cursor [MATCH pattern] [COUNT count]` | Iterates through the keys in the current database. |
| `HSCAN` | `HSCAN key cursor [MATCH pattern] [COUNT count]` | Iterates through the fields and values of a hash. |
| `SSCAN` | `SSCAN key cursor [MATCH pattern] [COUNT count]` | Iterates through the members of a set. |
| `ZSCAN` | `ZSCAN key cursor [MATCH pattern] [COUNT count]` | Iterates through the members and scores of a sorted set. |
