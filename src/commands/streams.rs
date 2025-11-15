use crate::commands::CommandHelp;
use std::collections::HashMap;

pub fn get_commands() -> HashMap<&'static str, CommandHelp> {
    let mut m = HashMap::new();
    m.insert(
        "xack",
        CommandHelp {
            summary: "Acknowledges pending messages for a consumer group.",
            usage: "XACK <key> <group> <id> [id ...]",
            since: "1.0.0",
            group: "streams",
        },
    );
    m.insert("xadd", CommandHelp {
        summary: "Appends a new entry to a stream, creating the stream if it does not exist.",
        usage: "XADD <key> [NOMKSTREAM] [MAXLEN [~] <count>] <ID> <field> <value> [field <value> ...]",
        since: "1.0.0",
        group: "streams",
    });
    m.insert("xautoclaim", CommandHelp {
        summary: "Automatically claims pending messages from a consumer group that have been idle for a minimum amount of time.",
        usage: "XAUTOCLAIM <key> <group> <consumer> <min-idle-time> <start-id> [COUNT <count>] [JUSTID]",
        since: "1.0.0",
        group: "streams",
    });
    m.insert("xclaim", CommandHelp {
        summary: "Changes the ownership of a pending message in a consumer group.",
        usage: "XCLAIM <key> <group> <consumer> <min-idle-time> <id> [id ...] [IDLE <ms>] [TIME <ms>] [RETRYCOUNT <count>] [FORCE] [JUSTID]",
        since: "1.0.0",
        group: "streams",
    });
    m.insert(
        "xdel",
        CommandHelp {
            summary: "Deletes one or more messages from a stream.",
            usage: "XDEL <key> <id> [id ...]",
            since: "1.0.0",
            group: "streams",
        },
    );
    m.insert("xgroup", CommandHelp {
        summary: "Manages consumer groups for streams.",
        usage: "XGROUP CREATE <key> <groupname> <id> [MKSTREAM] | XGROUP SETID <key> <groupname> <id> | XGROUP DESTROY <key> <groupname> | XGROUP DELCONSUMER <key> <groupname> <consumername>",
        since: "1.0.0",
        group: "streams",
    });
    m.insert("xinfo", CommandHelp {
        summary: "Returns information about a stream or its consumer groups.",
        usage: "XINFO STREAM <key> [FULL] | XINFO GROUPS <key> | XINFO CONSUMERS <key> <groupname>",
        since: "1.0.0",
        group: "streams",
    });
    m.insert(
        "xlen",
        CommandHelp {
            summary: "Returns the number of entries in a stream.",
            usage: "XLEN <key>",
            since: "1.0.0",
            group: "streams",
        },
    );
    m.insert("xpending", CommandHelp {
        summary: "Returns information about pending messages in a consumer group.",
        usage: "XPENDING <key> <group> | XPENDING <key> <group> [IDLE <min-idle-time>] <start> <end> <count> [consumer]",
        since: "1.0.0",
        group: "streams",
    });
    m.insert(
        "xrange",
        CommandHelp {
            summary: "Returns a range of entries from a stream.",
            usage: "XRANGE <key> <start> <end> [COUNT <count>]",
            since: "1.0.0",
            group: "streams",
        },
    );
    m.insert("xread", CommandHelp {
        summary: "Reads entries from one or more streams, with optional blocking.",
        usage: "XREAD [COUNT <count>] [BLOCK <milliseconds>] STREAMS <key> [key ...] <ID> [ID ...]",
        since: "1.0.0",
        group: "streams",
    });
    m.insert("xreadgroup", CommandHelp {
        summary: "Reads entries from one or more streams as part of a consumer group, with optional blocking.",
        usage: "XREADGROUP GROUP <groupname> <consumername> [COUNT <count>] [BLOCK <milliseconds>] [NOACK] STREAMS <key> [key ...] <ID> [ID ...]",
        since: "1.0.0",
        group: "streams",
    });
    m.insert(
        "xtrim",
        CommandHelp {
            summary: "Trims a stream to a given maximum length or minimum ID.",
            usage: "XTRIM <key> [LIMIT <count>] MAXLEN [~] <count> | MINID [~] <ID>",
            since: "1.0.0",
            group: "streams",
        },
    );
    m
}
