use crate::commands::CommandHelp;
use std::collections::HashMap;

pub fn get_commands() -> HashMap<&'static str, CommandHelp> {
    let mut m = HashMap::new();
    m.insert(
        "hdel",
        CommandHelp {
            summary: "Deletes one or more hash fields.",
            usage: "HDEL <key> <field> [field ...]",
            since: "1.0.0",
            group: "hash",
        },
    );
    m.insert(
        "hexists",
        CommandHelp {
            summary: "Determines if a hash field exists.",
            usage: "HEXISTS <key> <field>",
            since: "1.0.0",
            group: "hash",
        },
    );
    m.insert(
        "hget",
        CommandHelp {
            summary: "Gets the value of a hash field.",
            usage: "HGET <key> <field>",
            since: "1.0.0",
            group: "hash",
        },
    );
    m.insert(
        "hgetall",
        CommandHelp {
            summary: "Gets all the fields and values in a hash.",
            usage: "HGETALL <key>",
            since: "1.0.0",
            group: "hash",
        },
    );
    m.insert(
        "hincrby",
        CommandHelp {
            summary: "Increments the integer value of a hash field by the given number.",
            usage: "HINCRBY <key> <field> <increment>",
            since: "1.0.0",
            group: "hash",
        },
    );
    m.insert(
        "hincrbyfloat",
        CommandHelp {
            summary: "Increments the float value of a hash field by the given amount.",
            usage: "HINCRBYFLOAT <key> <field> <increment>",
            since: "1.0.0",
            group: "hash",
        },
    );
    m.insert(
        "hkeys",
        CommandHelp {
            summary: "Gets all the field names in a hash.",
            usage: "HKEYS <key>",
            since: "1.0.0",
            group: "hash",
        },
    );
    m.insert(
        "hlen",
        CommandHelp {
            summary: "Gets the number of fields in a hash.",
            usage: "HLEN <key>",
            since: "1.0.0",
            group: "hash",
        },
    );
    m.insert(
        "hmget",
        CommandHelp {
            summary: "Gets the values of all the given hash fields.",
            usage: "HMGET <key> <field> [field ...]",
            since: "1.0.0",
            group: "hash",
        },
    );
    m.insert(
        "hrandfield",
        CommandHelp {
            summary: "Returns a random field from the hash stored at key.",
            usage: "HRANDFIELD <key> [count [WITHVALUES]]",
            since: "1.0.0",
            group: "hash",
        },
    );
    m.insert("hset", CommandHelp {
        summary: "Sets the string value of a hash field. If the key does not exist, a new hash is created.",
        usage: "HSET <key> <field> <value> [field <value> ...]",
        since: "1.0.0",
        group: "hash",
    });
    m.insert(
        "hsetnx",
        CommandHelp {
            summary: "Sets the value of a hash field, only if the field does not exist.",
            usage: "HSETNX <key> <field> <value>",
            since: "1.0.0",
            group: "hash",
        },
    );
    m.insert(
        "hstrlen",
        CommandHelp {
            summary: "Gets the string length of the value associated with a hash field.",
            usage: "HSTRLEN <key> <field>",
            since: "1.0.0",
            group: "hash",
        },
    );
    m.insert(
        "hvals",
        CommandHelp {
            summary: "Gets all the values in a hash.",
            usage: "HVALS <key>",
            since: "1.0.0",
            group: "hash",
        },
    );
    m
}
