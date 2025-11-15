use crate::commands::CommandHelp;
use std::collections::HashMap;

pub fn get_commands() -> HashMap<&'static str, CommandHelp> {
    let mut m = HashMap::new();
    m.insert("geoadd", CommandHelp {
        summary: "Adds one or more geospatial items to the sorted set stored at key.",
        usage: "GEOADD <key> <longitude> <latitude> <member> [<longitude> <latitude> <member> ...]",
        since: "1.0.0",
        group: "geospatial",
    });
    m.insert(
        "geodist",
        CommandHelp {
            summary: "Returns the distance between two members in a geospatial index.",
            usage: "GEODIST <key> <member1> <member2> [unit]",
            since: "1.0.0",
            group: "geospatial",
        },
    );
    m.insert(
        "geohash",
        CommandHelp {
            summary: "Returns the Geohash string for one or more members in a geospatial index.",
            usage: "GEOHASH <key> <member> [member ...]",
            since: "1.0.0",
            group: "geospatial",
        },
    );
    m.insert("geopos", CommandHelp {
        summary: "Returns the longitude and latitude of one or more members in a geospatial index.",
        usage: "GEOPOS <key> <member> [member ...]",
        since: "1.0.0",
        group: "geospatial",
    });
    m.insert("georadius", CommandHelp {
        summary: "Finds members within a given radius from a specified longitude and latitude.",
        usage: "GEORADIUS <key> <longitude> <latitude> <radius> <unit> [WITHCOORD] [WITHDIST] [WITHHASH] [COUNT <count>] [ASC|DESC] [STORE <key>] [STOREDIST <key>]",
        since: "1.0.0",
        group: "geospatial",
    });
    m.insert("georadiusbymember", CommandHelp {
        summary: "Finds members within a given radius from a specified member.",
        usage: "GEORADIUSBYMEMBER <key> <member> <radius> <unit> [WITHCOORD] [WITHDIST] [WITHHASH] [COUNT <count>] [ASC|DESC] [STORE <key>] [STOREDIST <key>]",
        since: "1.0.0",
        group: "geospatial",
    });
    m
}
