use crate::commands::CommandHelp;
use std::collections::HashMap;

pub fn get_commands() -> HashMap<&'static str, CommandHelp> {
    let mut m = HashMap::new();
    m.insert("acl", CommandHelp {
        summary: "Manages Access Control List (ACL) users and rules.",
        usage: "ACL SETUSER <username> [rules ...] | ACL GETUSER <username> | ACL DELUSER <username> | ACL LIST | ACL SAVE",
        since: "1.0.0",
        group: "generic",
    });
    m.insert(
        "asking",
        CommandHelp {
            summary: "Used in Redis Cluster to redirect clients to the correct node.",
            usage: "ASKING",
            since: "1.0.0",
            group: "generic",
        },
    );
    m.insert(
        "auth",
        CommandHelp {
            summary: "Authenticates the current client to the server.",
            usage: "AUTH <password>",
            since: "1.0.0",
            group: "generic",
        },
    );
    m.insert(
        "backup",
        CommandHelp {
            summary: "Creates a backup of the database to the specified path.",
            usage: "BACKUP <path>",
            since: "1.0.0",
            group: "generic",
        },
    );
    m.insert(
        "bgrewriteaof",
        CommandHelp {
            summary: "Rewrites the Append Only File (AOF) in the background.",
            usage: "BGREWRITEAOF",
            since: "1.0.0",
            group: "generic",
        },
    );
    m.insert(
        "bgsave",
        CommandHelp {
            summary: "Saves the database in the background.",
            usage: "BGSAVE",
            since: "1.0.0",
            group: "generic",
        },
    );
    m.insert("client", CommandHelp {
        summary: "Manages client connections, including listing, setting names, and killing clients.",
        usage: "CLIENT LIST | CLIENT SETNAME <name> | CLIENT GETNAME | CLIENT KILL <id> | CLIENT SETINFO [LIB-NAME <name>] [LIB-VER <version>]",
        since: "1.0.0",
        group: "generic",
    });
    m.insert(
        "command",
        CommandHelp {
            summary: "Returns array reply of details about all SpinelDB commands.",
            usage: "COMMAND",
            since: "1.0.0",
            group: "generic",
        },
    );
    m.insert(
        "config",
        CommandHelp {
            summary: "Manages the runtime configuration of the SpinelDB server.",
            usage: "CONFIG GET <parameter> | CONFIG SET <parameter> <value> | CONFIG REWRITE",
            since: "1.0.0",
            group: "generic",
        },
    );
    m.insert(
        "dbsize",
        CommandHelp {
            summary: "Returns the number of keys in the currently selected database.",
            usage: "DBSIZE",
            since: "1.0.0",
            group: "generic",
        },
    );
    m.insert(
        "del",
        CommandHelp {
            summary: "Deletes one or more keys.",
            usage: "DEL key [key ...]",
            since: "1.0.0",
            group: "generic",
        },
    );
    m.insert(
        "echo",
        CommandHelp {
            summary: "Echoes the given string.",
            usage: "ECHO <message>",
            since: "1.0.0",
            group: "generic",
        },
    );
    m.insert(
        "eval",
        CommandHelp {
            summary: "Executes a server-side Lua script.",
            usage: "EVAL <script> <numkeys> <key> [key ...] <arg> [arg ...]",
            since: "1.0.0",
            group: "generic",
        },
    );
    m.insert(
        "evalsha",
        CommandHelp {
            summary: "Executes a server-side Lua script by its SHA1 digest.",
            usage: "EVALSHA <sha1> <numkeys> <key> [key ...] <arg> [arg ...]",
            since: "1.0.0",
            group: "generic",
        },
    );
    m.insert(
        "exists",
        CommandHelp {
            summary: "Checks if one or more keys exist.",
            usage: "EXISTS key [key ...]",
            since: "1.0.0",
            group: "generic",
        },
    );
    m.insert(
        "expire",
        CommandHelp {
            summary: "Sets a key's time to live in seconds.",
            usage: "EXPIRE <key> <seconds>",
            since: "1.0.0",
            group: "generic",
        },
    );
    m.insert(
        "expireat",
        CommandHelp {
            summary: "Sets a key's expiration time to a Unix timestamp in seconds.",
            usage: "EXPIREAT <key> <unix-time-seconds>",
            since: "1.0.0",
            group: "generic",
        },
    );
    m.insert(
        "failover",
        CommandHelp {
            summary: "Manages failover procedures, primarily for replication.",
            usage: "FAILOVER POISON <run_id> <ttl_secs>",
            since: "1.0.0",
            group: "generic",
        },
    );
    m.insert("flushall", CommandHelp {
        summary: "Deletes all the keys of all the existing databases, not just the currently selected one.",
        usage: "FLUSHALL",
        since: "1.0.0",
        group: "generic",
    });
    m.insert(
        "flushdb",
        CommandHelp {
            summary: "Deletes all the keys of the currently selected database.",
            usage: "FLUSHDB",
            since: "1.0.0",
            group: "generic",
        },
    );
    m.insert(
        "info",
        CommandHelp {
            summary: "Returns information and statistics about the server.",
            usage: "INFO [section]",
            since: "1.0.0",
            group: "generic",
        },
    );
    m.insert(
        "keys",
        CommandHelp {
            summary: "Finds all keys matching the given pattern.",
            usage: "KEYS <pattern>",
            since: "1.0.0",
            group: "generic",
        },
    );
    m.insert(
        "lastsave",
        CommandHelp {
            summary: "Returns the Unix timestamp of the last successful save to disk.",
            usage: "LASTSAVE",
            since: "1.0.0",
            group: "generic",
        },
    );
    m.insert(
        "latency",
        CommandHelp {
            summary: "Provides information about latency events and helps diagnose latency issues.",
            usage: "LATENCY DOCTOR | LATENCY HISTORY <event>",
            since: "1.0.0",
            group: "generic",
        },
    );
    m.insert(
        "memory",
        CommandHelp {
            summary: "Provides information about memory usage of the server or a specific key.",
            usage: "MEMORY USAGE <key>",
            since: "1.0.0",
            group: "generic",
        },
    );
    m.insert(
        "migrate",
        CommandHelp {
            summary: "Atomically transfer a key from the current instance to a different instance.",
            usage: "MIGRATE <host> <port> <key> <destination-db> <timeout> [COPY] [REPLACE]",
            since: "1.0.0",
            group: "generic",
        },
    );
    m.insert(
        "pexpire",
        CommandHelp {
            summary: "Sets a key's time to live in milliseconds.",
            usage: "PEXPIRE <key> <milliseconds>",
            since: "1.0.0",
            group: "generic",
        },
    );
    m.insert(
        "pexpireat",
        CommandHelp {
            summary: "Sets a key's expiration time to a Unix timestamp in milliseconds.",
            usage: "PEXPIREAT <key> <unix-time-milliseconds>",
            since: "1.0.0",
            group: "generic",
        },
    );
    m.insert(
        "persist",
        CommandHelp {
            summary: "Removes the expiration from a key.",
            usage: "PERSIST <key>",
            since: "1.0.0",
            group: "generic",
        },
    );
    m.insert(
        "ping",
        CommandHelp {
            summary: "Pings the server. If a message is provided, it returns that message.",
            usage: "PING [message]",
            since: "1.0.0",
            group: "generic",
        },
    );
    m.insert(
        "psubscribe",
        CommandHelp {
            summary: "Subscribes the client to the given patterns.",
            usage: "PSUBSCRIBE <pattern> [pattern ...]",
            since: "1.0.0",
            group: "generic",
        },
    );
    m.insert(
        "psync",
        CommandHelp {
            summary: "Performs a partial or full resynchronization with a master.",
            usage: "PSYNC <replication_id> <offset>",
            since: "1.0.0",
            group: "generic",
        },
    );
    m.insert(
        "pubsub",
        CommandHelp {
            summary: "Provides introspection capabilities for the Pub/Sub system.",
            usage: "PUBSUB CHANNELS [pattern] | PUBSUB NUMSUB [channel ...] | PUBSUB NUMPAT",
            since: "1.0.0",
            group: "generic",
        },
    );
    m.insert(
        "publish",
        CommandHelp {
            summary: "Posts a message to the given channel.",
            usage: "PUBLISH <channel> <message>",
            since: "1.0.0",
            group: "generic",
        },
    );
    m.insert(
        "punsubscribe",
        CommandHelp {
            summary: "Unsubscribes the client from the given patterns.",
            usage: "PUNSUBSCRIBE [pattern ...]",
            since: "1.0.0",
            group: "generic",
        },
    );
    m.insert(
        "quit",
        CommandHelp {
            summary: "Closes the client connection.",
            usage: "QUIT",
            since: "1.0.0",
            group: "generic",
        },
    );
    m.insert(
        "rename",
        CommandHelp {
            summary: "Renames a key.",
            usage: "RENAME <key> <newkey>",
            since: "1.0.0",
            group: "generic",
        },
    );
    m.insert(
        "renamenx",
        CommandHelp {
            summary: "Renames a key, only if the new key does not exist.",
            usage: "RENAMENX <key> <newkey>",
            since: "1.0.0",
            group: "generic",
        },
    );
    m.insert(
        "replconf",
        CommandHelp {
            summary: "Used by replicas to communicate with their master.",
            usage: "REPLCONF <argument> [argument ...]",
            since: "1.0.0",
            group: "generic",
        },
    );
    m.insert(
        "restore",
        CommandHelp {
            summary: "Creates a key using the provided serialized value, TTL, and options.",
            usage: "RESTORE <key> <ttl> <serialized-value> [REPLACE]",
            since: "1.0.0",
            group: "generic",
        },
    );
    m.insert(
        "role",
        CommandHelp {
            summary: "Returns the role of the instance in the context of replication.",
            usage: "ROLE",
            since: "1.0.0",
            group: "generic",
        },
    );
    m.insert(
        "save",
        CommandHelp {
            summary: "Synchronously saves the database to disk.",
            usage: "SAVE",
            since: "1.0.0",
            group: "generic",
        },
    );
    m.insert(
        "script",
        CommandHelp {
            summary: "Manages the server-side Lua script cache.",
            usage: "SCRIPT FLUSH | SCRIPT EXISTS <sha1> [sha1 ...] | SCRIPT LOAD <script>",
            since: "1.0.0",
            group: "generic",
        },
    );
    m.insert(
        "select",
        CommandHelp {
            summary: "Selects the database to be used by the current client.",
            usage: "SELECT <index>",
            since: "1.0.0",
            group: "generic",
        },
    );
    m.insert(
        "shutdown",
        CommandHelp {
            summary: "Synchronously saves the database and then shuts down the server.",
            usage: "SHUTDOWN [NOSAVE|SAVE]",
            since: "1.0.0",
            group: "generic",
        },
    );
    m.insert(
        "slowlog",
        CommandHelp {
            summary: "Manages the Slow Log of commands that exceeded a specified execution time.",
            usage: "SLOWLOG GET [count] | SLOWLOG LEN | SLOWLOG RESET",
            since: "1.0.0",
            group: "generic",
        },
    );
    m.insert("sort", CommandHelp {
        summary: "Sorts the elements in a list, set or sorted set, or the keys of a hash, by some criteria.",
        usage: "SORT <key> [BY <pattern>] [LIMIT <offset> <count>] [GET <pattern> [GET <pattern> ...]] [ASC|DESC] [ALPHA] [STORE <destination>]",
        since: "1.0.0",
        group: "generic",
    });
    m.insert(
        "subscribe",
        CommandHelp {
            summary: "Subscribes the client to the given channels.",
            usage: "SUBSCRIBE <channel> [channel ...]",
            since: "1.0.0",
            group: "generic",
        },
    );
    m.insert("time", CommandHelp {
        summary: "Returns the current server time as a two-item array: Unix timestamp in seconds, and microseconds.",
        usage: "TIME",
        since: "1.0.0",
        group: "generic",
    });
    m.insert(
        "ttl",
        CommandHelp {
            summary: "Returns the remaining time to live of a key that has a timeout, in seconds.",
            usage: "TTL <key>",
            since: "1.0.0",
            group: "generic",
        },
    );
    m.insert(
        "type",
        CommandHelp {
            summary: "Returns the string representation of the type of the value stored at key.",
            usage: "TYPE <key>",
            since: "1.0.0",
            group: "generic",
        },
    );
    m.insert(
        "unlink",
        CommandHelp {
            summary: "Deletes one or more keys asynchronously.",
            usage: "UNLINK <key> [key ...]",
            since: "1.0.0",
            group: "generic",
        },
    );
    m.insert(
        "unsubscribe",
        CommandHelp {
            summary: "Unsubscribes the client from the given channels.",
            usage: "UNSUBSCRIBE [channel ...]",
            since: "1.0.0",
            group: "generic",
        },
    );
    m.insert(
        "unwatch",
        CommandHelp {
            summary: "Flushes all previously WATCHed keys for the current transaction.",
            usage: "UNWATCH",
            since: "1.0.0",
            group: "generic",
        },
    );
    m.insert(
        "watch",
        CommandHelp {
            summary: "Marks the given keys to be watched for changes by the current transaction.",
            usage: "WATCH <key> [key ...]",
            since: "1.0.0",
            group: "generic",
        },
    );
    m
}
