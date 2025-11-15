use clap::Parser;

/// A CLI for interacting with a SpinelDB server
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// The hostname of the SpinelDB server
    #[arg(short = 'H', long, default_value = "127.0.0.1")]
    pub host: String,

    /// The port of the SpinelDB server
    #[arg(short, long, default_value_t = 7878)]
    pub port: u16,

    #[arg(short, long)]
    pub user: Option<String>,

    #[arg(
        short = 'a',
        long,
        help = "Password to use when connecting to the server.",
        long_help = "Password to use when connecting to the server. WARNING: Specifying the password on the command line is insecure as it may be visible to other users via process lists and stored in shell history. Consider using the --user flag for a secure password prompt."
    )]
    pub password: Option<String>,

    /// Output raw RESP data.
    #[arg(long, group = "output_format")]
    pub raw: bool,

    /// Output in CSV format.
    #[arg(long, group = "output_format")]
    pub csv: bool,

    /// Output in JSON format.
    #[arg(long, group = "output_format")]
    pub json: bool,

    /// Read commands from stdin (pipe mode).
    #[arg(long, group = "mode")]
    pub pipe: bool,

    /// Run in latency monitoring mode.
    #[arg(long, group = "mode")]
    pub latency: bool,

    /// Run in scan mode (for iterating keys).
    #[arg(long, group = "mode")]
    pub scan: bool,

    /// Run in stat mode (for server statistics).
    #[arg(long, group = "mode")]
    pub stat: bool,

    /// Run in bigkeys mode (for finding large keys).
    #[arg(long, group = "mode")]
    pub bigkeys: bool,

    /// Run in memkeys mode (for memory usage of keys).
    #[arg(long, group = "mode")]
    pub memkeys: bool,

    /// Run in hotkeys mode (for finding hot keys).
    #[arg(long, group = "mode")]
    pub hotkeys: bool,

    /// Run in cluster management mode.
    #[arg(long, group = "mode")]
    pub cluster: bool,

    /// Command to execute (optional). If provided, CLI will execute and exit.
    /// Otherwise, it will enter interactive REPL mode.
    #[arg(trailing_var_arg = true, allow_hyphen_values = true, group = "mode")]
    pub command: Option<Vec<String>>,
}
