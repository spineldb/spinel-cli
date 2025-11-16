# List Commands

Lists are lists of strings, sorted by insertion order.

| Command | Usage | Summary |
| --- | --- | --- |
| `LINDEX` | `LINDEX key index` | Gets an element from a list by its index. |
| `LINSERT` | `LINSERT key BEFORE|AFTER pivot value` | Inserts an element before or after another element in a list. |
| `LLEN` | `LLEN key` | Gets the length of a list. |
| `LPOP` | `LPOP key` | Removes and gets the first element in a list. |
| `LPUSH` | `LPUSH key value [value ...]` | Prepends one or multiple values to a list. |
| `LPUSHX` | `LPUSHX key value` | Prepends a value to a list, only if the list exists. |
| `LRANGE` | `LRANGE key start stop` | Gets a range of elements from a list. |
| `LREM` | `LREM key count value` | Removes elements from a list. |
| `LSET` | `LSET key index value` | Sets the value of an element in a list by its index. |
| `LTRIM` | `LTRIM key start stop` | Trims a list to the specified range. |
| `RPOP` | `RPOP key` | Removes and gets the last element in a list. |
| `RPOPLPUSH` | `RPOPLPUSH source destination` | Removes the last element in a list, prepends it to another list, and returns it. |
| `RPUSH` | `RPUSH key value [value ...]` | Appends one or multiple values to a list. |
| `RPUSHX` | `RPUSHX key value` | Appends a value to a list, only if the list exists. |
