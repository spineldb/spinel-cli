# Streams Commands

Streams are append-only logs of data. They are a powerful data structure for managing high-throughput, real-time data.

| Command | Usage | Summary |
| --- | --- | --- |
| `XADD` | `XADD key ID field value [field value ...]` | Appends a new entry to a stream. |
| `XREAD` | `XREAD [COUNT count] [BLOCK milliseconds] STREAMS key [key ...] ID [ID ...]` | Reads data from one or more streams, starting from a given ID. |
| `XRANGE` | `XRANGE key start end [COUNT count]` | Returns the stream entries matching a given range of IDs. |
| `XREVRANGE` | `XREVRANGE key end start [COUNT count]` | Returns the stream entries matching a given range of IDs, in reverse order. |
| `XLEN` | `XLEN key` | Gets the number of entries in a stream. |
| `XDEL` | `XDEL key ID [ID ...]` | Removes the specified entries from a stream. |
| `XTRIM` | `XTRIM key MAXLEN [~] count` | Trims the stream to a given number of items. |
| `XGROUP CREATE` | `XGROUP CREATE key groupname id-or-$` | Creates a new consumer group associated with a stream. |
| `XGROUP DESTROY` | `XGROUP DESTROY key groupname` | Destroys a consumer group. |
| `XREADGROUP` | `XREADGROUP GROUP group consumer [COUNT count] [BLOCK milliseconds] STREAMS key [key ...] id [id ...]` | Reads from a stream for a consumer group. |
| `XACK` | `XACK key group ID [ID ...]` | Acknowledges the processing of one or more messages. |
| `XPENDING` | `XPENDING key group` | Shows pending messages for a consumer group. |
