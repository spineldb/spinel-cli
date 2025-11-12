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

    #[arg(short = 'a', long)]
    pub password: Option<String>,

    /// Read commands from stdin (pipe mode).
    #[arg(long)]
    pub pipe: bool,

    /// Command to execute (optional). If provided, CLI will execute and exit.
    /// Otherwise, it will enter interactive REPL mode.
    #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
    pub command: Option<Vec<String>>,
}
