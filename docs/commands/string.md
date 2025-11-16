# String Commands

String commands are used to manage string values in the database.

| Command | Usage | Summary |
| --- | --- | --- |
| `APPEND` | `APPEND key value` | Appends a value to a key. |
| `DECR` | `DECR key` | Decrements the integer value of a key by one. |
| `DECRBY` | `DECRBY key decrement` | Decrements the integer value of a key by the given number. |
| `GET` | `GET key` | Gets the value of a key. |
| `GETSET` | `GETSET key value` | Sets the string value of a key and returns its old value. |
| `INCR` | `INCR key` | Increments the integer value of a key by one. |
| `INCRBY` | `INCRBY key increment` | Increments the integer value of a key by the given number. |
| `INCRBYFLOAT` | `INCRBYFLOAT key increment` | Increments the float value of a key by the given amount. |
| `MGET` | `MGET key [key ...]` | Gets the values of all the given keys. |
| `MSET` | `MSET key value [key value ...]` | Sets multiple keys to multiple values. |
| `MSETNX` | `MSETNX key value [key value ...]` | Sets multiple keys to multiple values, only if none of the keys exist. |
| `PSETEX` | `PSETEX key milliseconds value` | Sets the value and expiration of a key in milliseconds. |
| `SET` | `SET key value [EX seconds | PX milliseconds] [NX|XX]` | Sets the string value of a key. |
| `SETEX` | `SETEX key seconds value` | Sets the value and expiration of a key. |
| `SETNX` | `SETNX key value` | Sets the value of a key, only if the key does not exist. |
| `STRLEN` | `STRLEN key` | Gets the length of the value of a key. |
