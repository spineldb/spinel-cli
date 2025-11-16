# Set Commands

Sets are unordered collections of unique strings.

| Command | Usage | Summary |
| --- | --- | --- |
| `SADD` | `SADD key member [member ...]` | Adds one or more members to a set. |
| `SCARD` | `SCARD key` | Gets the number of members in a set. |
| `SDIFF` | `SDIFF key [key ...]` | Subtracts multiple sets. |
| `SDIFFSTORE` | `SDIFFSTORE destination key [key ...]` | Subtracts multiple sets and stores the resulting set in a key. |
| `SINTER` | `SINTER key [key ...]` | Intersects multiple sets. |
| `SINTERSTORE` | `SINTERSTORE destination key [key ...]` | Intersects multiple sets and stores the resulting set in a key. |
| `SISMEMBER` | `SISMEMBER key member` | Determines if a given value is a member of a set. |
| `SMEMBERS` | `SMEMBERS key` | Gets all the members in a set. |
| `SMOVE` | `SMOVE source destination member` | Moves a member from one set to another. |
| `SPOP` | `SPOP key [count]` | Removes and returns one or more random members from a set. |
| `SRANDMEMBER` | `SRANDMEMBER key [count]` | Gets one or more random members from a set. |
| `SREM` | `SREM key member [member ...]` | Removes one or more members from a set. |
| `SUNION` | `SUNION key [key ...]` | Adds multiple sets. |
| `SUNIONSTORE` | `SUNIONSTORE destination key [key ...]` | Adds multiple sets and stores the resulting set in a key. |
