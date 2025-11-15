use crate::commands::CommandHelp;
use std::collections::HashMap;

pub fn get_commands() -> HashMap<&'static str, CommandHelp> {
    let mut m = HashMap::new();
    m.insert("cache.bypass", CommandHelp {
        summary: "Fetches content directly from an origin without reading from or writing to the cache.",
        usage: "CACHE.BYPASS <key> <url>",
        since: "1.0.0",
        group: "cache",
    });
    m.insert("cache.fetch", CommandHelp {
        summary: "Fetches cacheable content from an origin server, with support for streaming large bodies and stampede protection.",
        usage: "CACHE.FETCH <key> <url> [TTL <seconds>] [SWR <seconds>] [GRACE <seconds>] [VARY <header>] [TAGS <tag> ...] [HEADERS <header> <value> ...]",
        since: "1.0.0",
        group: "cache",
    });
    m.insert("cache.get", CommandHelp {
        summary: "Retrieves a cached object, supporting content variants and stale-while-revalidate.",
        usage: "CACHE.GET <key> [REVALIDATE <url>] [IF-NONE-MATCH <etag>] [IF-MODIFIED-SINCE <date>] [FORCE-REVALIDATE] [HEADERS <header> <value> ...]",
        since: "1.0.0",
        group: "cache",
    });
    m.insert(
        "cache.info",
        CommandHelp {
            summary: "Returns detailed information about a cache entry.",
            usage: "CACHE.INFO <key>",
            since: "1.0.0",
            group: "cache",
        },
    );
    m.insert("cache.lock", CommandHelp {
        summary: "Manually locks a cache key for a specified duration, preventing it from being evicted.",
        usage: "CACHE.LOCK <key> <ttl_seconds>",
        since: "1.0.0",
        group: "cache",
    });
    m.insert("cache.policy", CommandHelp {
        summary: "Manages declarative caching rules.",
        usage: "CACHE.POLICY SET <name> <key_pattern> <url_template> [TTL <seconds>] [SWR <seconds>] [GRACE <seconds>] [NEGATIVE_TTL <seconds>] [PRIORITY <p>] [COMPRESSION] [FORCE-DISK] [PREWARM] [RESPECT_ORIGIN_HEADERS] [TAGS <tag> ...] [VARY_ON <header> ...]\nCACHE.POLICY DEL <name>\nCACHE.POLICY GET <name>\nCACHE.POLICY LIST",
        since: "1.0.0",
        group: "cache",
    });
    m.insert("cache.proxy", CommandHelp {
        summary: "Provides a get-or-fetch pattern. It attempts to retrieve a key, and if it's a cache miss, it automatically fetches from an origin and caches the result.",
        usage: "CACHE.PROXY <key> [url] [TTL <seconds>] [SWR <seconds>] [GRACE <seconds>] [TAGS <tag> ...] [VARY <header>] [HEADERS <header> <value> ...]",
        since: "1.0.0",
        group: "cache",
    });
    m.insert("cache.purge", CommandHelp {
        summary: "Adds one or more glob patterns to the lazy purge list. Keys matching these patterns will be evicted on their next access.",
        usage: "CACHE.PURGE <pattern> [pattern ...]",
        since: "1.0.0",
        group: "cache",
    });
    m.insert(
        "cache.purgetag",
        CommandHelp {
            summary: "Efficiently invalidates all cache entries associated with one or more tags.",
            usage: "CACHE.PURGETAG <tag> [tag ...]",
            since: "1.0.0",
            group: "cache",
        },
    );
    m.insert("cache.set", CommandHelp {
        summary: "Stores an object in the cache with advanced options for TTL, tagging, and content negotiation.",
        usage: "CACHE.SET <key> <value> [TTL <seconds>] [SWR <seconds>] [GRACE <seconds>] [ETAG <etag>] [LAST-MODIFIED <date>] [VARY <header>] [REVALIDATE-URL <url>] [COMPRESSION] [FORCE-DISK] [TAGS <tag> ...] [HEADERS <header> <value> ...]",
        since: "1.0.0",
        group: "cache",
    });
    m.insert("cache.softpurge", CommandHelp {
        summary: "Marks one or more cache entries as stale, allowing for background revalidation on the next request.",
        usage: "CACHE.SOFTPURGE <key> [key ...]",
        since: "1.0.0",
        group: "cache",
    });
    m.insert(
        "cache.softpurgetag",
        CommandHelp {
            summary: "Marks all cache entries associated with one or more tags as stale.",
            usage: "CACHE.SOFTPURGETAG <tag> [tag ...]",
            since: "1.0.0",
            group: "cache",
        },
    );
    m.insert(
        "cache.stats",
        CommandHelp {
            summary: "Returns cache performance metrics.",
            usage: "CACHE.STATS",
            since: "1.0.0",
            group: "cache",
        },
    );
    m.insert(
        "cache.unlock",
        CommandHelp {
            summary: "Manually unlocks a cache key, allowing it to be evicted again.",
            usage: "CACHE.UNLOCK <key>",
            since: "1.0.0",
            group: "cache",
        },
    );
    m
}
