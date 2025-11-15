use anyhow::{Result, anyhow};
use clap::Parser;
use futures::{SinkExt, stream::StreamExt};
use spineldb::core::protocol::RespFrame;
use std::net::SocketAddr;
use tokio::io::{self, AsyncBufReadExt};

mod cli;
mod commands;
mod display;
mod repl;

use cli::Args;
use display::{_print_resp_frame_formatted, print_resp_frame_to_stdout};
use repl::Connection;
use repl::run as run_repl; // Import Connection type

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    let addr: SocketAddr = format!("{}:{}", args.host, args.port).parse()?;

    // If any specialized mode is requested, or a command is given, or pipe mode is on,
    // we run in non-interactive mode.
    if args.pipe
        || args.command.is_some()
        || args.latency
        || args.scan
        || args.stat
        || args.bigkeys
        || args.memkeys
        || args.hotkeys
        || args.cluster
    {
        run_non_interactive_mode(args).await?;
    } else {
        // Otherwise, run in interactive REPL mode.
        let initial_connection = match repl::try_connect(&args.host, args.port).await {
            Ok(conn) => {
                println!("Connected to {}:{}", args.host, args.port);
                Some(conn)
            }
            Err(e) => {
                let error_message = if let Some(io_err) = e.downcast_ref::<io::Error>() {
                    if io_err.kind() == io::ErrorKind::ConnectionRefused {
                        "Connection refused".to_string()
                    } else {
                        format!("{}", e)
                    }
                } else {
                    format!("{}", e)
                };
                eprintln!(
                    "Could not connect to SpinelDB at {}:{}: {}",
                    args.host, args.port, error_message
                );
                None
            }
        };

        run_repl(initial_connection, addr, &args).await?;
    }

    Ok(())
}

async fn get_auth_credentials(args: &Args) -> Result<Option<(Option<String>, String)>> {
    // 1. Check --password flag
    if let Some(password) = args.password.clone() {
        return Ok(Some((args.user.clone(), password)));
    }

    // 2. Check REDISCLI_AUTH environment variable
    if let Ok(auth_str) = std::env::var("REDISCLI_AUTH") {
        if auth_str.contains(':') {
            let parts: Vec<&str> = auth_str.splitn(2, ':').collect();
            if parts.len() == 2 {
                return Ok(Some((Some(parts[0].to_string()), parts[1].to_string())));
            }
        }
        // If it's just a password or malformed username:password, treat as password only
        return Ok(Some((args.user.clone(), auth_str)));
    }

    // 3. If --user is present, interactively prompt for password
    if let Some(user) = args.user.clone() {
        let password = rpassword::prompt_password("Password: ")?;
        return Ok(Some((Some(user), password)));
    }

    Ok(None)
}

async fn run_non_interactive_mode(args: Args) -> Result<()> {
    let mut framed = match repl::try_connect(&args.host, args.port).await {
        Ok(conn) => conn,
        Err(e) => {
            let error_message = if let Some(io_err) = e.downcast_ref::<io::Error>() {
                if io_err.kind() == io::ErrorKind::ConnectionRefused {
                    "Connection refused".to_string()
                } else {
                    format!("{}", e)
                }
            } else {
                format!("{}", e)
            };
            eprintln!(
                "Could not connect to SpinelDB server at {}:{}: {}",
                args.host, args.port, error_message
            );
            return Err(e);
        }
    };

    // Handle automatic authentication
    if let Some((username, password)) = get_auth_credentials(&args).await? {
        let auth_cmd = if let Some(user) = username {
            RespFrame::Array(vec![
                RespFrame::BulkString("AUTH".into()),
                RespFrame::BulkString(user.into()),
                RespFrame::BulkString(password.into()),
            ])
        } else {
            RespFrame::Array(vec![
                RespFrame::BulkString("AUTH".into()),
                RespFrame::BulkString(password.into()),
            ])
        };

        if let Err(e) = framed.send(auth_cmd).await {
            eprintln!("Failed to send AUTH command: {}", e);
            return Err(e.into());
        }

        match framed.next().await {
            Some(Ok(RespFrame::SimpleString(ref s))) if s == "OK" => {}
            Some(Ok(RespFrame::Error(e))) => {
                return Err(anyhow!("Authentication failed: {}", e));
            }
            Some(Ok(frame)) => {
                let mut frame_str = Vec::new();
                _print_resp_frame_formatted(&frame, &mut frame_str, 0)?;
                return Err(anyhow!(
                    "Authentication failed: Unexpected response: {}",
                    String::from_utf8_lossy(&frame_str)
                ));
            }
            Some(Err(e)) => return Err(e.into()),
            None => return Err(anyhow!("Failed to get AUTH response: connection closed")),
        }
    }

    if args.pipe {
        let mut reader = io::BufReader::new(io::stdin());
        let mut line = String::new();
        while reader.read_line(&mut line).await? > 0 {
            let parts = match shlex::split(&line) {
                Some(parts) => parts,
                None => {
                    eprintln!("Error parsing command: Unbalanced quotes or invalid input.");
                    line.clear();
                    continue;
                }
            };
            if parts.is_empty() {
                line.clear();
                continue;
            }

            let command = RespFrame::Array(
                parts
                    .into_iter()
                    .map(|s| RespFrame::BulkString(s.clone().into()))
                    .collect(),
            );

            if let Err(e) = framed.send(command).await {
                eprintln!("Failed to send command in pipe mode: {}", e);
                return Err(e.into());
            }

            if let Some(Ok(frame)) = framed.next().await {
                print_resp_frame_to_stdout(&frame, &args);
            } else {
                return Err(anyhow!(
                    "Connection closed while waiting for response in pipe mode."
                ));
            }
            line.clear();
        }
    } else if let Some(ref cmd_parts) = args.command {
        let command = RespFrame::Array(
            cmd_parts
                .iter()
                .map(|s| RespFrame::BulkString(s.clone().into()))
                .collect(),
        );

        if let Err(e) = framed.send(command).await {
            eprintln!("Failed to send command: {}", e);
            return Err(e.into());
        }

        if let Some(Ok(frame)) = framed.next().await {
            print_resp_frame_to_stdout(&frame, &args);
        } else {
            return Err(anyhow!("Connection closed while waiting for response."));
        }
    } else if args.latency {
        run_latency_mode(&mut framed).await?;
    } else if args.scan {
        run_scan_mode(&mut framed).await?;
    } else if args.stat {
        run_stat_mode(&mut framed).await?;
    } else if args.bigkeys {
        run_bigkeys_mode(&mut framed).await?;
    } else if args.memkeys {
        run_memkeys_mode(&mut framed).await?;
    } else if args.hotkeys {
        run_hotkeys_mode(&mut framed).await?;
    } else if args.cluster {
        run_cluster_mode(&mut framed).await?;
    }

    Ok(())
}

// Placeholder functions for specialized modes
async fn run_latency_mode(_conn: &mut Connection) -> Result<()> {
    println!("Latency monitoring mode not yet fully implemented.");
    Ok(())
}

async fn run_scan_mode(_conn: &mut Connection) -> Result<()> {
    println!("Scan mode not yet fully implemented.");
    Ok(())
}

async fn run_stat_mode(_conn: &mut Connection) -> Result<()> {
    println!("Stat mode not yet fully implemented.");
    Ok(())
}

async fn run_bigkeys_mode(_conn: &mut Connection) -> Result<()> {
    println!("Bigkeys mode not yet fully implemented.");
    Ok(())
}

async fn run_memkeys_mode(_conn: &mut Connection) -> Result<()> {
    println!("Memkeys mode not yet fully implemented.");
    Ok(())
}

async fn run_hotkeys_mode(_conn: &mut Connection) -> Result<()> {
    println!("Hotkeys mode not yet fully implemented.");
    Ok(())
}

async fn run_cluster_mode(_conn: &mut Connection) -> Result<()> {
    println!("Cluster mode not yet fully implemented.");
    Ok(())
}
