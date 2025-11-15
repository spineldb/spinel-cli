use crate::commands::CommandHelp;
use std::collections::HashMap;

pub fn get_commands() -> HashMap<&'static str, CommandHelp> {
    let mut m = HashMap::new();
    m.insert(
        "scan",
        CommandHelp {
            summary: "Incrementally iterates over keys in the current database.",
            usage: "SCAN <cursor> [MATCH <pattern>] [COUNT <count>]",
            since: "1.0.0",
            group: "scan",
        },
    );
    m.insert(
        "hscan",
        CommandHelp {
            summary: "Incrementally iterates over fields and values of a hash.",
            usage: "HSCAN <key> <cursor> [MATCH <pattern>] [COUNT <count>]",
            since: "1.0.0",
            group: "scan",
        },
    );
    m.insert(
        "sscan",
        CommandHelp {
            summary: "Incrementally iterates over members of a set.",
            usage: "SSCAN <key> <cursor> [MATCH <pattern>] [COUNT <count>]",
            since: "1.0.0",
            group: "scan",
        },
    );
    m.insert(
        "zscan",
        CommandHelp {
            summary: "Incrementally iterates over members and their scores of a sorted set.",
            usage: "ZSCAN <key> <cursor> [MATCH <pattern>] [COUNT <count>]",
            since: "1.0.0",
            group: "scan",
        },
    );
    m
}
