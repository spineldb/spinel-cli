use crate::commands::CommandHelp;
use std::collections::HashMap;

pub fn get_commands() -> HashMap<&'static str, CommandHelp> {
    let mut m = HashMap::new();
    m.insert(
        "json.arrappend",
        CommandHelp {
            summary: "Appends JSON values to the end of an array at a specified path.",
            usage: "JSON.ARRAPPEND <key> <path> <value> [value ...]",
            since: "1.0.0",
            group: "json",
        },
    );
    m.insert("json.arrindex", CommandHelp {
        summary: "Returns the index of the first occurrence of a JSON value in an array at a specified path.",
        usage: "JSON.ARRINDEX <key> <path> <value> [start] [end]",
        since: "1.0.0",
        group: "json",
    });
    m.insert(
        "json.arrinsert",
        CommandHelp {
            summary: "Inserts JSON values into an array at a specified path and index.",
            usage: "JSON.ARRINSERT <key> <path> <index> <value> [value ...]",
            since: "1.0.0",
            group: "json",
        },
    );
    m.insert(
        "json.arrlen",
        CommandHelp {
            summary: "Returns the length of a JSON array at a specified path.",
            usage: "JSON.ARRLEN <key> [path]",
            since: "1.0.0",
            group: "json",
        },
    );
    m.insert(
        "json.arrpop",
        CommandHelp {
            summary:
                "Removes and returns an element from a JSON array at a specified path and index.",
            usage: "JSON.ARRPOP <key> [path] [index]",
            since: "1.0.0",
            group: "json",
        },
    );
    m.insert("json.arrtrim", CommandHelp {
        summary: "Trims a JSON array at a specified path to contain only the elements within the specified start and stop indices.",
        usage: "JSON.ARRTRIM <key> <path> <start> <stop>",
        since: "1.0.0",
        group: "json",
    });
    m.insert(
        "json.clear",
        CommandHelp {
            summary: "Clears the contents of an array, object, or string at a specified path.",
            usage: "JSON.CLEAR <key> <path>",
            since: "1.0.0",
            group: "json",
        },
    );
    m.insert(
        "json.del",
        CommandHelp {
            summary: "Deletes a JSON value at a specified path.",
            usage: "JSON.DEL <key> [path ...]",
            since: "1.0.0",
            group: "json",
        },
    );
    m.insert(
        "json.get",
        CommandHelp {
            summary: "Returns the value of a JSON document or a specific path within it.",
            usage: "JSON.GET <key> [path ...]",
            since: "1.0.0",
            group: "json",
        },
    );
    m.insert(
        "json.merge",
        CommandHelp {
            summary: "Merges a JSON value into an existing JSON value at a specified path.",
            usage: "JSON.MERGE <key> <path> <value>",
            since: "1.0.0",
            group: "json",
        },
    );
    m.insert(
        "json.mget",
        CommandHelp {
            summary: "Returns the values of a JSON path from multiple keys.",
            usage: "JSON.MGET <key> [key ...] <path>",
            since: "1.0.0",
            group: "json",
        },
    );
    m.insert(
        "json.numincrby",
        CommandHelp {
            summary:
                "Increments a number at a specified path in a JSON document by a given amount.",
            usage: "JSON.NUMINCRBY <key> <path> <number>",
            since: "1.0.0",
            group: "json",
        },
    );
    m.insert(
        "json.nummultby",
        CommandHelp {
            summary:
                "Multiplies a number at a specified path in a JSON document by a given amount.",
            usage: "JSON.NUMMULTBY <key> <path> <number>",
            since: "1.0.0",
            group: "json",
        },
    );
    m.insert(
        "json.objkeys",
        CommandHelp {
            summary: "Returns the keys of a JSON object at a specified path.",
            usage: "JSON.OBJKEYS <key> [path]",
            since: "1.0.0",
            group: "json",
        },
    );
    m.insert(
        "json.objlen",
        CommandHelp {
            summary: "Returns the number of keys in a JSON object at a specified path.",
            usage: "JSON.OBJLEN <key> [path]",
            since: "1.0.0",
            group: "json",
        },
    );
    m.insert(
        "json.set",
        CommandHelp {
            summary:
                "Sets the JSON value at a specified path. Creates the key if it doesn't exist.",
            usage: "JSON.SET <key> <path> <value> [NX|XX]",
            since: "1.0.0",
            group: "json",
        },
    );
    m.insert(
        "json.strappend",
        CommandHelp {
            summary: "Appends a string to a JSON string value at a specified path.",
            usage: "JSON.STRAPPEND <key> <path> <value>",
            since: "1.0.0",
            group: "json",
        },
    );
    m.insert(
        "json.strlen",
        CommandHelp {
            summary: "Returns the length of a JSON string value at a specified path.",
            usage: "JSON.STRLEN <key> [path]",
            since: "1.0.0",
            group: "json",
        },
    );
    m.insert(
        "json.toggle",
        CommandHelp {
            summary: "Toggles a boolean value at a specified path in a JSON document.",
            usage: "JSON.TOGGLE <key> <path>",
            since: "1.0.0",
            group: "json",
        },
    );
    m.insert(
        "json.type",
        CommandHelp {
            summary: "Returns the JSON type of the value at a specified path.",
            usage: "JSON.TYPE <key> [path]",
            since: "1.0.0",
            group: "json",
        },
    );
    m
}
