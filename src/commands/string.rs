use crate::commands::CommandHelp;
use std::collections::HashMap;

pub fn get_commands() -> HashMap<&'static str, CommandHelp> {
    let mut m = HashMap::new();
    m.insert(
        "append",
        CommandHelp {
            summary: "Appends a value to a key.",
            usage: "APPEND <key> <value>",
            since: "1.0.0",
            group: "string",
        },
    );
    m.insert(
        "bitcount",
        CommandHelp {
            summary: "Counts the number of set bits (1s) in a string.",
            usage: "BITCOUNT <key> [start] [end]",
            since: "1.0.0",
            group: "string",
        },
    );
    m.insert("bitfield", CommandHelp {
        summary: "Performs arbitrary bitfield operations on a string value.",
        usage: "BITFIELD <key> [GET <type> <offset>] [SET <type> <offset> <value>] [INCRBY <type> <offset> <increment>] [OVERFLOW WRAP|SAT|FAIL]",
        since: "1.0.0",
        group: "string",
    });
    m.insert("bitop", CommandHelp {
        summary: "Performs a bitwise operation between multiple string keys and stores the result in a destination key.",
        usage: "BITOP AND|OR|XOR <destkey> <key> [key ...] | BITOP NOT <destkey> <key>",
        since: "1.0.0",
        group: "string",
    });
    m.insert(
        "bitpos",
        CommandHelp {
            summary: "Finds the position of the first bit set to 1 or 0 in a string.",
            usage: "BITPOS <key> <bit> [start] [end]",
            since: "1.0.0",
            group: "string",
        },
    );
    m.insert(
        "decr",
        CommandHelp {
            summary: "Decrements the integer value of a key by one.",
            usage: "DECR <key>",
            since: "1.0.0",
            group: "string",
        },
    );
    m.insert(
        "decrby",
        CommandHelp {
            summary: "Decrements the integer value of a key by the given amount.",
            usage: "DECRBY <key> <decrement>",
            since: "1.0.0",
            group: "string",
        },
    );
    m.insert(
        "get",
        CommandHelp {
            summary: "Gets the value of a key.",
            usage: "GET <key>",
            since: "1.0.0",
            group: "string",
        },
    );
    m.insert(
        "getbit",
        CommandHelp {
            summary: "Returns the bit value at offset in the string value stored at key.",
            usage: "GETBIT <key> <offset>",
            since: "1.0.0",
            group: "string",
        },
    );
    m.insert(
        "getdel",
        CommandHelp {
            summary: "Gets the value of a key and deletes the key.",
            usage: "GETDEL <key>",
            since: "1.0.0",
            group: "string",
        },
    );
    m.insert("getex", CommandHelp {
        summary: "Gets the value of a key and optionally sets its expiration.",
        usage: "GETEX <key> [EX <seconds> | PX <milliseconds> | EXAT <unix-time-seconds> | PXAT <unix-time-milliseconds> | PERSIST]",
        since: "1.0.0",
        group: "string",
    });
    m.insert("getrange", CommandHelp {
        summary: "Returns the substring of the string value stored at key, determined by the offsets start and end (both inclusive).",
        usage: "GETRANGE <key> <start> <end>",
        since: "1.0.0",
        group: "string",
    });
    m.insert(
        "getset",
        CommandHelp {
            summary: "Atomically sets a key to a new value and returns the old value.",
            usage: "GETSET <key> <value>",
            since: "1.0.0",
            group: "string",
        },
    );
    m.insert(
        "incr",
        CommandHelp {
            summary: "Increments the integer value of a key by one.",
            usage: "INCR <key>",
            since: "1.0.0",
            group: "string",
        },
    );
    m.insert(
        "incrby",
        CommandHelp {
            summary: "Increments the integer value of a key by the given amount.",
            usage: "INCRBY <key> <increment>",
            since: "1.0.0",
            group: "string",
        },
    );
    m.insert(
        "incrbyfloat",
        CommandHelp {
            summary: "Increments the float value of a key by the given amount.",
            usage: "INCRBYFLOAT <key> <increment>",
            since: "1.0.0",
            group: "string",
        },
    );
    m.insert(
        "mget",
        CommandHelp {
            summary: "Gets the values of all the given keys.",
            usage: "MGET <key> [key ...]",
            since: "1.0.0",
            group: "string",
        },
    );
    m.insert(
        "mset",
        CommandHelp {
            summary: "Sets multiple keys to multiple values.",
            usage: "MSET <key> <value> [key <value> ...]",
            since: "1.0.0",
            group: "string",
        },
    );
    m.insert("msetnx", CommandHelp {
        summary: "Sets multiple keys to multiple values, only if none of the keys already exist.",
        usage: "MSETNX <key> <value> [key <value> ...]",
        since: "1.0.0",
        group: "string",
    });
    m.insert(
        "psetex",
        CommandHelp {
            summary: "Sets the value and expiration of a key in milliseconds.",
            usage: "PSETEX <key> <milliseconds> <value>",
            since: "1.0.0",
            group: "string",
        },
    );
    m.insert("set", CommandHelp {
        summary: "Sets the string value of a key, ignoring its type. The key is created if it doesn't exist.",
        usage: "SET <key> <value> [EX <seconds> | PX <milliseconds> | EXAT <unix-time-seconds> | PXAT <unix-time-milliseconds> | KEEPTTL | PERSIST] [NX|XX] [GET]",
        since: "1.0.0",
        group: "string",
    });
    m.insert(
        "setbit",
        CommandHelp {
            summary: "Sets or clears the bit at offset in the string value stored at key.",
            usage: "SETBIT <key> <offset> <value>",
            since: "1.0.0",
            group: "string",
        },
    );
    m.insert(
        "setex",
        CommandHelp {
            summary: "Sets the value and expiration of a key in seconds.",
            usage: "SETEX <key> <seconds> <value>",
            since: "1.0.0",
            group: "string",
        },
    );
    m.insert(
        "setrange",
        CommandHelp {
            summary: "Overwrites part of a string at key starting at the specified offset.",
            usage: "SETRANGE <key> <offset> <value>",
            since: "1.0.0",
            group: "string",
        },
    );
    m.insert(
        "strlen",
        CommandHelp {
            summary: "Returns the length of the string value stored at a key.",
            usage: "STRLEN <key>",
            since: "1.0.0",
            group: "string",
        },
    );
    m
}
