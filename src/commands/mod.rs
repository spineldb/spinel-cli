use lazy_static::lazy_static;
use std::collections::HashMap;

pub struct CommandHelp {
    pub summary: &'static str,
    pub usage: &'static str,
    pub since: &'static str,
    pub group: &'static str,
}

pub mod bloom;
pub mod cache;
pub mod generic;
pub mod geospatial;
pub mod hash;
pub mod hyperloglog;
pub mod json;
pub mod list;
pub mod scan;
pub mod set;
pub mod streams;
pub mod string;
pub mod zset;

lazy_static! {
    pub static ref COMMANDS: HashMap<&'static str, CommandHelp> = {
        let mut m = HashMap::new();
        m.extend(generic::get_commands());
        m.extend(string::get_commands());
        m.extend(list::get_commands());
        m.extend(hash::get_commands());
        m.extend(set::get_commands());
        m.extend(zset::get_commands());
        m.extend(geospatial::get_commands());
        m.extend(streams::get_commands());
        m.extend(scan::get_commands());
        m.extend(hyperloglog::get_commands());
        m.extend(json::get_commands());
        m.extend(bloom::get_commands());
        m.extend(cache::get_commands());

        // Add CLI-specific commands
        m.insert("connect", CommandHelp {
            summary: "Connects to a SpinelDB server.",
            usage: "connect <host> <port>",
            since: "1.0.0",
            group: "connection",
        });
        m.insert("exit", CommandHelp {
            summary: "Closes the connection.",
            usage: "EXIT",
            since: "1.0.0",
            group: "connection",
        });
        m
    };
}
