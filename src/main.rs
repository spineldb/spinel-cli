use anyhow::{Result, anyhow};
use clap::Parser;
use futures::{SinkExt, stream::StreamExt};
use spineldb::core::protocol::RespFrame;
use std::net::SocketAddr;
use tokio::io::{self, AsyncReadExt};

mod cli;
mod commands;
mod display;
mod repl;

use cli::Args;
use display::{print_resp_frame, print_resp_frame_to_stdout};
use repl::run as run_repl;

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    let addr: SocketAddr = format!("{}:{}", args.host, args.port).parse()?;

    // --- Non-Interactive Mode (Pipe or Direct Command) ---
    if args.pipe || args.command.is_some() {
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
        if let Some(password) = args.password {
            let auth_cmd = RespFrame::Array(vec![
                RespFrame::BulkString("AUTH".into()),
                RespFrame::BulkString(password.into()),
            ]);

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
                    print_resp_frame(&frame, &mut frame_str)?;
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
            let mut buffer = String::new();
            io::stdin().read_to_string(&mut buffer).await?;

            let lines = buffer.lines().filter(|line| !line.is_empty());

            for line in lines {
                let parts: Vec<String> = shlex::split(line).unwrap_or_default();
                if parts.is_empty() {
                    continue;
                }

                let command = RespFrame::Array(
                    parts
                        .into_iter()
                        .map(|s| RespFrame::BulkString(s.into()))
                        .collect(),
                );

                if let Err(e) = framed.send(command).await {
                    eprintln!("Failed to send command in pipe mode: {}", e);
                    return Err(e.into());
                }

                if let Some(Ok(frame)) = framed.next().await {
                    print_resp_frame_to_stdout(&frame);
                } else {
                    return Err(anyhow!(
                        "Connection closed while waiting for response in pipe mode."
                    ));
                }
            }
        } else if let Some(cmd_parts) = args.command {
            let command = RespFrame::Array(
                cmd_parts
                    .into_iter()
                    .map(|s| RespFrame::BulkString(s.into()))
                    .collect(),
            );

            if let Err(e) = framed.send(command).await {
                eprintln!("Failed to send command: {}", e);
                return Err(e.into());
            }

            if let Some(Ok(frame)) = framed.next().await {
                print_resp_frame_to_stdout(&frame);
            } else {
                return Err(anyhow!("Connection closed while waiting for response."));
            }
        }

        return Ok(());
    }

    // --- Interactive Mode ---
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

    run_repl(initial_connection, addr).await?;

    Ok(())
}
