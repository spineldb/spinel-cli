# Hash Commands

Hashes are maps between string fields and string values.

| Command | Usage | Summary |
| --- | --- | --- |
| `HDEL` | `HDEL key field [field ...]` | Deletes one or more hash fields. |
| `HEXISTS` | `HEXISTS key field` | Determines if a hash field exists. |
| `HGET` | `HGET key field` | Gets the value of a hash field. |
| `HGETALL` | `HGETALL key` | Gets all the fields and values in a hash. |
| `HINCRBY` | `HINCRBY key field increment` | Increments the integer value of a hash field by the given number. |
| `HINCRBYFLOAT` | `HINCRBYFLOAT key field increment` | Increments the float value of a hash field by the given amount. |
| `HKEYS` | `HKEYS key` | Gets all the fields in a hash. |
| `HLEN` | `HLEN key` | Gets the number of fields in a hash. |
| `HMGET` | `HMGET key field [field ...]` | Gets the values of all the given hash fields. |
| `HMSET` | `HMSET key field value [field value ...]` | Sets multiple hash fields to multiple values. |
| `HSET` | `HSET key field value` | Sets the string value of a hash field. |
| `HSETNX` | `HSETNX key field value` | Sets the value of a hash field, only if the field does not exist. |
| `HVALS` | `HVALS key` | Gets all the values in a hash. |
