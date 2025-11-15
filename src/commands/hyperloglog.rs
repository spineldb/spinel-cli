use crate::commands::CommandHelp;
use std::collections::HashMap;

pub fn get_commands() -> HashMap<&'static str, CommandHelp> {
    let mut m = HashMap::new();
    m.insert(
        "pfadd",
        CommandHelp {
            summary:
                "Adds all the element arguments to the HyperLogLog data structure stored at key.",
            usage: "PFADD <key> <element> [element ...]",
            since: "1.0.0",
            group: "hyperloglog",
        },
    );
    m.insert("pfcount", CommandHelp {
        summary: "Returns the approximated cardinality of the set(s) observed by the HyperLogLog at key(s).",
        usage: "PFCOUNT <key> [key ...]",
        since: "1.0.0",
        group: "hyperloglog",
    });
    m.insert(
        "pfmerge",
        CommandHelp {
            summary: "Merges multiple HyperLogLog values into a single HyperLogLog value.",
            usage: "PFMERGE <destkey> <sourcekey> [sourcekey ...]",
            since: "1.0.0",
            group: "hyperloglog",
        },
    );
    m
}
