# Sorted Set (ZSet) Commands

Sorted Sets are collections of unique strings (members) where each member is associated with a score.

| Command | Usage | Summary |
| --- | --- | --- |
| `ZADD` | `ZADD key [NX|XX] [CH] [INCR] score member [score member ...]` | Adds one or more members to a sorted set, or updates its score if it already exists. |
| `ZCARD` | `ZCARD key` | Gets the number of members in a sorted set. |
| `ZCOUNT` | `ZCOUNT key min max` | Counts the members in a sorted set with scores within the given values. |
| `ZINCRBY` | `ZINCRBY key increment member` | Increments the score of a member in a sorted set. |
| `ZRANGE` | `ZRANGE key start stop [WITHSCORES]` | Returns a range of members in a sorted set, by index. |
| `ZRANGEBYSCORE` | `ZRANGEBYSCORE key min max [WITHSCORES] [LIMIT offset count]` | Returns a range of members in a sorted set, by score. |
| `ZRANK` | `ZRANK key member` | Determines the index of a member in a sorted set. |
| `ZREM` | `ZREM key member [member ...]` | Removes one or more members from a sorted set. |
| `ZREMRANGEBYRANK` | `ZREMRANGEBYRANK key start stop` | Removes all members in a sorted set within the given indices. |
| `ZREMRANGEBYSCORE` | `ZREMRANGEBYSCORE key min max` | Removes all members in a sorted set within the given scores. |
| `ZREVRANGE` | `ZREVRANGE key start stop [WITHSCORES]` | Returns a range of members in a sorted set, by index, with scores ordered from high to low. |
| `ZREVRANGEBYSCORE` | `ZREVRANGEBYSCORE key max min [WITHSCORES] [LIMIT offset count]` | Returns a range of members in a sorted set, by score, with scores ordered from high to low. |
| `ZREVRANK` | `ZREVRANK key member` | Determines the index of a member in a sorted set, with scores ordered from high to low. |
| `ZSCORE` | `ZSCORE key member` | Gets the score associated with the given member in a sorted set. |
