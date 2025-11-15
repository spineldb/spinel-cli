use crate::commands::CommandHelp;
use std::collections::HashMap;

pub fn get_commands() -> HashMap<&'static str, CommandHelp> {
    let mut m = HashMap::new();
    m.insert("bzpopmax", CommandHelp {
        summary: "Removes and returns the member with the highest score from the first non-empty sorted set, or blocks until one is available.",
        usage: "BZPOPMAX <key> [key ...] <timeout>",
        since: "1.0.0",
        group: "zset",
    });
    m.insert("bzpopmin", CommandHelp {
        summary: "Removes and returns the member with the lowest score from the first non-empty sorted set, or blocks until one is available.",
        usage: "BZPOPMIN <key> [key ...] <timeout>",
        since: "1.0.0",
        group: "zset",
    });
    m.insert("zadd", CommandHelp {
        summary: "Adds one or more members to a sorted set, or updates their scores if they already exist.",
        usage: "ZADD <key> [NX|XX] [GT|LT] [CH] [INCR] <score> <member> [score <member> ...]",
        since: "1.0.0",
        group: "zset",
    });
    m.insert(
        "zcard",
        CommandHelp {
            summary: "Returns the number of members in a sorted set.",
            usage: "ZCARD <key>",
            since: "1.0.0",
            group: "zset",
        },
    );
    m.insert(
        "zcount",
        CommandHelp {
            summary:
                "Returns the number of members in a sorted set with scores within the given range.",
            usage: "ZCOUNT <key> <min> <max>",
            since: "1.0.0",
            group: "zset",
        },
    );
    m.insert(
        "zincrby",
        CommandHelp {
            summary: "Increments the score of a member in a sorted set by the given amount.",
            usage: "ZINCRBY <key> <increment> <member>",
            since: "1.0.0",
            group: "zset",
        },
    );
    m.insert("zinterstore", CommandHelp {
        summary: "Computes the intersection of multiple sorted sets and stores the result in a new sorted set.",
        usage: "ZINTERSTORE <destination> <numkeys> <key> [key ...] [WEIGHTS <weight> [weight ...]] [AGGREGATE SUM|MIN|MAX]",
        since: "1.0.0",
        group: "zset",
    });
    m.insert("zlexcount", CommandHelp {
        summary: "Counts the number of members in a sorted set between a given lexicographical range.",
        usage: "ZLEXCOUNT <key> <min> <max>",
        since: "1.0.0",
        group: "zset",
    });
    m.insert(
        "zmscore",
        CommandHelp {
            summary: "Returns the scores of the specified members in a sorted set.",
            usage: "ZMSCORE <key> <member> [member ...]",
            since: "1.0.0",
            group: "zset",
        },
    );
    m.insert(
        "zpopmax",
        CommandHelp {
            summary: "Removes and returns the member with the highest score from a sorted set.",
            usage: "ZPOPMAX <key> [count]",
            since: "1.0.0",
            group: "zset",
        },
    );
    m.insert(
        "zpopmin",
        CommandHelp {
            summary: "Removes and returns the member with the lowest score from a sorted set.",
            usage: "ZPOPMIN <key> [count]",
            since: "1.0.0",
            group: "zset",
        },
    );
    m.insert(
        "zrange",
        CommandHelp {
            summary: "Returns a range of members from a sorted set, by index.",
            usage: "ZRANGE <key> <start> <stop> [WITHSCORES]",
            since: "1.0.0",
            group: "zset",
        },
    );
    m.insert(
        "zrangebylex",
        CommandHelp {
            summary:
                "Returns all the members in a sorted set between a given lexicographical range.",
            usage: "ZRANGEBYLEX <key> <min> <max> [LIMIT <offset> <count>]",
            since: "1.0.0",
            group: "zset",
        },
    );
    m.insert(
        "zrangebyscore",
        CommandHelp {
            summary: "Returns all the members in a sorted set with scores within the given range.",
            usage: "ZRANGEBYSCORE <key> <min> <max> [WITHSCORES] [LIMIT <offset> <count>]",
            since: "1.0.0",
            group: "zset",
        },
    );
    m.insert("zrangestore", CommandHelp {
        summary: "Stores the result of a ZRANGE, ZRANGEBYLEX, or ZRANGEBYSCORE operation into a new sorted set.",
        usage: "ZRANGESTORE <destination> <source> <min> <max> [BYLEX|BYSCORE] [REV] [LIMIT <offset> <count>]",
        since: "1.0.0",
        group: "zset",
    });
    m.insert("zrank", CommandHelp {
        summary: "Determines the index of a member in a sorted set, with scores ordered from low to high.",
        usage: "ZRANK <key> <member>",
        since: "1.0.0",
        group: "zset",
    });
    m.insert(
        "zrem",
        CommandHelp {
            summary: "Removes one or more members from a sorted set.",
            usage: "ZREM <key> <member> [member ...]",
            since: "1.0.0",
            group: "zset",
        },
    );
    m.insert(
        "zremrangebylex",
        CommandHelp {
            summary: "Removes all members in a sorted set between a given lexicographical range.",
            usage: "ZREMRANGEBYLEX <key> <min> <max>",
            since: "1.0.0",
            group: "zset",
        },
    );
    m.insert(
        "zremrangebyrank",
        CommandHelp {
            summary: "Removes all members in a sorted set within the given rank range.",
            usage: "ZREMRANGEBYRANK <key> <start> <stop>",
            since: "1.0.0",
            group: "zset",
        },
    );
    m.insert(
        "zremrangebyscore",
        CommandHelp {
            summary: "Removes all members in a sorted set within the given score range.",
            usage: "ZREMRANGEBYSCORE <key> <min> <max>",
            since: "1.0.0",
            group: "zset",
        },
    );
    m.insert("zrevrange", CommandHelp {
        summary: "Returns a range of members from a sorted set, by index, with scores ordered from high to low.",
        usage: "ZREVRANGE <key> <start> <stop> [WITHSCORES]",
        since: "1.0.0",
        group: "zset",
    });
    m.insert("zrevrank", CommandHelp {
        summary: "Determines the index of a member in a sorted set, with scores ordered from high to low.",
        usage: "ZREVRANK <key> <member>",
        since: "1.0.0",
        group: "zset",
    });
    m.insert(
        "zscore",
        CommandHelp {
            summary: "Returns the score of a member in a sorted set.",
            usage: "ZSCORE <key> <member>",
            since: "1.0.0",
            group: "zset",
        },
    );
    m.insert("zunionstore", CommandHelp {
        summary: "Computes the union of multiple sorted sets and stores the result in a new sorted set.",
        usage: "ZUNIONSTORE <destination> <numkeys> <key> [key ...] [WEIGHTS <weight> [weight ...]] [AGGREGATE SUM|MIN|MAX]",
        since: "1.0.0",
        group: "zset",
    });
    m
}
