use crate::commands::CommandHelp;
use std::collections::HashMap;

pub fn get_commands() -> HashMap<&'static str, CommandHelp> {
    let mut m = HashMap::new();
    m.insert("bf.add", CommandHelp {
        summary: "Adds an item to a Bloom filter. If the filter does not exist, it is implicitly created with default parameters.",
        usage: "BF.ADD <key> <item>",
        since: "1.0.0",
        group: "bloom",
    });
    m.insert(
        "bf.card",
        CommandHelp {
            summary: "Returns the number of items in a Bloom filter.",
            usage: "BF.CARD <key>",
            since: "1.0.0",
            group: "bloom",
        },
    );
    m.insert(
        "bf.exists",
        CommandHelp {
            summary: "Checks if an item might exist in a Bloom filter.",
            usage: "BF.EXISTS <key> <item>",
            since: "1.0.0",
            group: "bloom",
        },
    );
    m.insert(
        "bf.info",
        CommandHelp {
            summary: "Returns information about a Bloom filter.",
            usage: "BF.INFO <key>",
            since: "1.0.0",
            group: "bloom",
        },
    );
    m.insert("bf.insert", CommandHelp {
        summary: "Adds one or more items to a Bloom filter, creating it if it does not exist with specified capacity and error rate.",
        usage: "BF.INSERT <key> [CAPACITY <capacity>] [ERROR <error_rate>] ITEMS <item> [item ...]",
        since: "1.0.0",
        group: "bloom",
    });
    m.insert("bf.madd", CommandHelp {
        summary: "Adds one or more items to a Bloom filter. If the filter does not exist, it is implicitly created with default parameters.",
        usage: "BF.MADD <key> <item> [item ...]",
        since: "1.0.0",
        group: "bloom",
    });
    m.insert(
        "bf.mexists",
        CommandHelp {
            summary: "Checks if multiple items might exist in a Bloom filter.",
            usage: "BF.MEXISTS <key> <item> [item ...]",
            since: "1.0.0",
            group: "bloom",
        },
    );
    m.insert("bf.reserve", CommandHelp {
        summary: "Creates a new Bloom filter with a specified initial capacity and desired error rate.",
        usage: "BF.RESERVE <key> <error_rate> <capacity>",
        since: "1.0.0",
        group: "bloom",
    });
    m
}
