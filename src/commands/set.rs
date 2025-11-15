use crate::commands::CommandHelp;
use std::collections::HashMap;

pub fn get_commands() -> HashMap<&'static str, CommandHelp> {
    let mut m = HashMap::new();
    m.insert(
        "sadd",
        CommandHelp {
            summary: "Adds one or more members to a set.",
            usage: "SADD <key> <member> [member ...]",
            since: "1.0.0",
            group: "set",
        },
    );
    m.insert(
        "scard",
        CommandHelp {
            summary: "Returns the number of elements in a set.",
            usage: "SCARD <key>",
            since: "1.0.0",
            group: "set",
        },
    );
    m.insert("sdiff", CommandHelp {
        summary: "Returns the members of the set resulting from the difference between the first set and all the successive sets.",
        usage: "SDIFF <key> [key ...]",
        since: "1.0.0",
        group: "set",
    });
    m.insert("sdiffstore", CommandHelp {
        summary: "Stores the members of the set resulting from the difference between the first set and all the successive sets in a new key.",
        usage: "SDIFFSTORE <destination> <key> [key ...]",
        since: "1.0.0",
        group: "set",
    });
    m.insert("sinter", CommandHelp {
        summary: "Returns the members of the set resulting from the intersection of all the given sets.",
        usage: "SINTER <key> [key ...]",
        since: "1.0.0",
        group: "set",
    });
    m.insert("sinterstore", CommandHelp {
        summary: "Stores the members of the set resulting from the intersection of all the given sets in a new key.",
        usage: "SINTERSTORE <destination> <key> [key ...]",
        since: "1.0.0",
        group: "set",
    });
    m.insert(
        "sismember",
        CommandHelp {
            summary: "Determines if a member is a member of a set.",
            usage: "SISMEMBER <key> <member>",
            since: "1.0.0",
            group: "set",
        },
    );
    m.insert(
        "smembers",
        CommandHelp {
            summary: "Returns all the members of the set value stored at key.",
            usage: "SMEMBERS <key>",
            since: "1.0.0",
            group: "set",
        },
    );
    m.insert(
        "smismember",
        CommandHelp {
            summary: "Returns whether each member is a member of a set.",
            usage: "SMISMEMBER <key> <member> [member ...]",
            since: "1.0.0",
            group: "set",
        },
    );
    m.insert(
        "smove",
        CommandHelp {
            summary: "Moves a member from one set to another.",
            usage: "SMOVE <source> <destination> <member>",
            since: "1.0.0",
            group: "set",
        },
    );
    m.insert(
        "spop",
        CommandHelp {
            summary: "Removes and returns one or more random members from a set.",
            usage: "SPOP <key> [count]",
            since: "1.0.0",
            group: "set",
        },
    );
    m.insert(
        "srandmember",
        CommandHelp {
            summary: "Returns one or more random members from a set.",
            usage: "SRANDMEMBER <key> [count]",
            since: "1.0.0",
            group: "set",
        },
    );
    m.insert(
        "srem",
        CommandHelp {
            summary: "Removes one or more members from a set.",
            usage: "SREM <key> <member> [member ...]",
            since: "1.0.0",
            group: "set",
        },
    );
    m.insert(
        "sunion",
        CommandHelp {
            summary:
                "Returns the members of the set resulting from the union of all the given sets.",
            usage: "SUNION <key> [key ...]",
            since: "1.0.0",
            group: "set",
        },
    );
    m.insert("sunionstore", CommandHelp {
        summary: "Stores the members of the set resulting from the union of all the given sets in a new key.",
        usage: "SUNIONSTORE <destination> <key> [key ...]",
        since: "1.0.0",
        group: "set",
    });
    m
}
