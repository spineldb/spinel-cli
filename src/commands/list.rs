use crate::commands::CommandHelp;
use std::collections::HashMap;

pub fn get_commands() -> HashMap<&'static str, CommandHelp> {
    let mut m = HashMap::new();
    m.insert("blmove", CommandHelp {
        summary: "Atomically returns and removes the first/last element of the list stored at source, pushes the element to the first/last element of the list stored at destination, or blocks until one is available.",
        usage: "BLMOVE <source> <destination> LEFT|RIGHT LEFT|RIGHT <timeout>",
        since: "1.0.0",
        group: "list",
    });
    m.insert(
        "blpop",
        CommandHelp {
            summary:
                "Removes and gets the first element in a list, or blocks until one is available.",
            usage: "BLPOP <key> [key ...] <timeout>",
            since: "1.0.0",
            group: "list",
        },
    );
    m.insert(
        "brpop",
        CommandHelp {
            summary:
                "Removes and gets the last element in a list, or blocks until one is available.",
            usage: "BRPOP <key> [key ...] <timeout>",
            since: "1.0.0",
            group: "list",
        },
    );
    m.insert(
        "lindex",
        CommandHelp {
            summary: "Gets an element from a list by its index.",
            usage: "LINDEX <key> <index>",
            since: "1.0.0",
            group: "list",
        },
    );
    m.insert(
        "linsert",
        CommandHelp {
            summary: "Inserts an element before or after another element in a list.",
            usage: "LINSERT <key> BEFORE|AFTER <pivot> <element>",
            since: "1.0.0",
            group: "list",
        },
    );
    m.insert(
        "llen",
        CommandHelp {
            summary: "Returns the length of the list stored at key.",
            usage: "LLEN <key>",
            since: "1.0.0",
            group: "list",
        },
    );
    m.insert("lmove", CommandHelp {
        summary: "Atomically returns and removes the first/last element of the list stored at source, and pushes the element to the first/last element of the list stored at destination.",
        usage: "LMOVE <source> <destination> LEFT|RIGHT LEFT|RIGHT",
        since: "1.0.0",
        group: "list",
    });
    m.insert(
        "lpop",
        CommandHelp {
            summary: "Removes and gets the first element in a list.",
            usage: "LPOP <key>",
            since: "1.0.0",
            group: "list",
        },
    );
    m.insert(
        "lpos",
        CommandHelp {
            summary: "Returns the index of the first occurrence of the element in the list.",
            usage: "LPOS <key> <element> [RANK <rank>] [COUNT <num-matches>] [MAXLEN <len>]",
            since: "1.0.0",
            group: "list",
        },
    );
    m.insert(
        "lpush",
        CommandHelp {
            summary: "Inserts all the specified values at the head of the list stored at key.",
            usage: "LPUSH <key> <value> [value ...]",
            since: "1.0.0",
            group: "list",
        },
    );
    m.insert("lpushx", CommandHelp {
        summary: "Inserts all the specified values at the head of the list stored at key, only if key already exists and holds a list.",
        usage: "LPUSHX <key> <value> [value ...]",
        since: "1.0.0",
        group: "list",
    });
    m.insert(
        "lrange",
        CommandHelp {
            summary: "Returns the specified elements of the list stored at key.",
            usage: "LRANGE <key> <start> <stop>",
            since: "1.0.0",
            group: "list",
        },
    );
    m.insert("lrem", CommandHelp {
        summary: "Removes the first count occurrences of elements equal to element from the list stored at key.",
        usage: "LREM <key> <count> <element>",
        since: "1.0.0",
        group: "list",
    });
    m.insert(
        "lset",
        CommandHelp {
            summary: "Sets the list element at index to element.",
            usage: "LSET <key> <index> <element>",
            since: "1.0.0",
            group: "list",
        },
    );
    m.insert(
        "ltrim",
        CommandHelp {
            summary: "Trims a list to the specified range of elements.",
            usage: "LTRIM <key> <start> <stop>",
            since: "1.0.0",
            group: "list",
        },
    );
    m.insert(
        "rpop",
        CommandHelp {
            summary: "Removes and gets the last element in a list.",
            usage: "RPOP <key>",
            since: "1.0.0",
            group: "list",
        },
    );
    m.insert(
        "rpush",
        CommandHelp {
            summary: "Inserts all the specified values at the tail of the list stored at key.",
            usage: "RPUSH <key> <value> [value ...]",
            since: "1.0.0",
            group: "list",
        },
    );
    m.insert("rpushx", CommandHelp {
        summary: "Inserts all the specified values at the tail of the list stored at key, only if key already exists and holds a list.",
        usage: "RPUSHX <key> <value> [value ...]",
        since: "1.0.0",
        group: "list",
    });
    m
}
