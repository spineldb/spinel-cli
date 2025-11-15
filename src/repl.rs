use crate::commands::COMMANDS;
use crate::display::print_resp_frame_to_stdout;
use anyhow::Result;
use futures::{SinkExt, stream::StreamExt};
use rustyline::{Editor, error::ReadlineError, history::DefaultHistory};
use spineldb::core::protocol::{RespFrame, RespFrameCodec};
use std::io;
use std::net::SocketAddr;
use std::time::Duration;
use tokio::net::TcpStream;
use tokio::signal;
use tokio::time::timeout;
use tokio_util::codec::Framed;

pub type Connection = Framed<TcpStream, RespFrameCodec>;

pub(crate) async fn try_connect(host: &str, port: u16) -> Result<Connection> {
    let addr = format!("{}:{}", host, port).parse::<SocketAddr>()?;
    let stream = TcpStream::connect(addr).await?;
    Ok(Framed::new(stream, RespFrameCodec))
}

pub async fn run(initial_connection: Option<Connection>, initial_addr: SocketAddr) -> Result<()> {
    let mut connection = initial_connection;
    let mut addr = initial_addr;
    let mut rl = Editor::<(), DefaultHistory>::new()?;

    'main_loop: loop {
        let prompt = if connection.is_some() {
            format!("{}> ", addr)
        } else {
            "not connected> ".to_string()
        };

        match rl.readline(&prompt) {
            Ok(line) => {
                let line = line.trim();
                if line.is_empty() {
                    continue;
                }
                let _ = rl.add_history_entry(line);

                let parts = match shlex::split(line) {
                    Some(parts) => parts,
                    None => {
                        eprintln!("Error parsing command: Unbalanced quotes or invalid input.");
                        continue;
                    }
                };
                let cmd = parts
                    .first()
                    .map(|s| s.to_ascii_lowercase())
                    .unwrap_or_default();

                match cmd.as_str() {
                    "quit" | "exit" => break 'main_loop,
                    "connect" => {
                        if connection.is_some() {
                            println!("Already connected. Use 'quit' to disconnect first.");
                            continue;
                        }
                        let host = parts.get(1).map(|s| s.as_str()).unwrap_or("127.0.0.1");
                        let port = parts
                            .get(2)
                            .and_then(|s| s.parse::<u16>().ok())
                            .unwrap_or(7878);
                        match try_connect(host, port).await {
                            Ok(conn) => {
                                connection = Some(conn);
                                addr = format!("{}:{}", host, port).parse()?;
                                println!("Connected to {}:{}", host, port);
                            }
                            Err(e) => {
                                let error_message =
                                    if let Some(io_err) = e.downcast_ref::<io::Error>() {
                                        if io_err.kind() == io::ErrorKind::ConnectionRefused {
                                            "Connection refused".to_string()
                                        } else {
                                            format!("{}", e)
                                        }
                                    } else {
                                        format!("{}", e)
                                    };
                                eprintln!("Failed to connect: {}", error_message);
                            }
                        }
                    }
                    "help" => {
                        if let Some(subcommand) = parts.get(1) {
                            if let Some(help) =
                                COMMANDS.get(subcommand.to_ascii_lowercase().as_str())
                            {
                                println!();
                                println!("  {}", help.usage);
                                println!("  summary: {}", help.summary);
                                println!("  since: {}", help.since);
                                println!("  group: {}", help.group);
                                println!();
                            } else {
                                println!("Unknown command for 'help': '{}'", subcommand);
                            }
                        } else {
                            println!("To get help on a specific command, type HELP <command>");
                            println!("To get a list of all commands, type COMMAND");
                        }
                    }
                    "auth" => {
                        if let Some(framed) = connection.as_mut() {
                            let password = match rpassword::prompt_password("Password: ") {
                                Ok(p) => p,
                                Err(e) => {
                                    eprintln!("Failed to read password: {}", e);
                                    continue;
                                }
                            };
                            let auth_cmd = RespFrame::Array(vec![
                                RespFrame::BulkString("AUTH".into()),
                                RespFrame::BulkString(password.into()),
                            ]);
                            if let Err(e) = framed.send(auth_cmd).await {
                                eprintln!("Failed to send AUTH command: {}", e);
                                connection = None;
                                continue;
                            }
                            if let Some(Ok(frame)) = framed.next().await {
                                print_resp_frame_to_stdout(&frame);
                            } else {
                                eprintln!("Error or server closed connection.");
                                connection = None;
                            }
                        } else {
                            eprintln!("Not connected. Please use 'connect'.");
                        }
                    }
                    _ => {
                        // Default command handling
                        if let Some(framed) = connection.as_mut() {
                            let is_pubsub = is_pubsub_command(&parts);
                            let command = RespFrame::Array(
                                parts
                                    .into_iter()
                                    .map(|s| RespFrame::BulkString(s.into()))
                                    .collect(),
                            );

                            if let Err(e) = framed.send(command).await {
                                eprintln!("Failed to send command: {}", e);
                                connection = None;
                                continue;
                            }

                            // Enter pub/sub mode if the command was a subscription
                            if is_pubsub {
                                println!("Entered Pub/Sub mode. Press CTRL-C to exit.");
                                'pubsub_loop: loop {
                                    tokio::select! {
                                        _ = signal::ctrl_c() => {
                                            println!("\nExiting Pub/Sub mode... Sending UNSUBSCRIBE.");
                                            let unsubscribe_cmd = RespFrame::Array(vec![RespFrame::BulkString("UNSUBSCRIBE".into())]);
                                            if let Err(e) = framed.send(unsubscribe_cmd).await {
                                                eprintln!("Failed to send UNSUBSCRIBE command: {}. Connection may be in an inconsistent state.", e);
                                                connection = None;
                                                break 'pubsub_loop;
                                            }
                                            // Wait for all unsubscribe confirmations
                                            'unsubscribe_loop: loop {
                                                match timeout(Duration::from_secs(5), framed.next()).await {
                                                    Ok(Some(Ok(frame))) => {
                                                        let is_final = if let RespFrame::Array(ps) = &frame {
                                                            if ps.len() == 3 {
                                                                if let (Some(RespFrame::BulkString(c)), Some(RespFrame::Integer(count))) = (ps.first(), ps.get(2)) {
                                                                    let c_str = String::from_utf8_lossy(c);
                                                                    (c_str.eq_ignore_ascii_case("unsubscribe") || c_str.eq_ignore_ascii_case("punsubscribe")) && *count == 0
                                                                } else { false }
                                                            } else { false }
                                                        } else { false };
                                                        print_resp_frame_to_stdout(&frame);
                                                        if is_final { break 'unsubscribe_loop; }
                                                    }
                                                    Ok(Some(Err(e))) => { eprintln!("Error waiting for unsubscribe confirmation: {}", e); connection = None; break 'unsubscribe_loop; }
                                                    Ok(None) => { eprintln!("Connection closed while waiting for unsubscribe confirmation."); connection = None; break 'unsubscribe_loop; }
                                                    Err(_) => { eprintln!("Timeout waiting for unsubscribe confirmation."); connection = None; break 'unsubscribe_loop; }
                                                }
                                            }
                                            println!("Returned to normal mode.");
                                            break 'pubsub_loop;
                                        }
                                        result = framed.next() => {
                                            match result {
                                                Some(Ok(frame)) => print_resp_frame_to_stdout(&frame),
                                                Some(Err(e)) => { eprintln!("Error in Pub/Sub mode: {}", e); connection = None; break 'pubsub_loop; }
                                                None => { println!("Server closed connection in Pub/Sub mode."); connection = None; break 'pubsub_loop; }
                                            }
                                        }
                                    }
                                }
                            } else {
                                // Normal command
                                if let Some(Ok(frame)) = framed.next().await {
                                    print_resp_frame_to_stdout(&frame);
                                } else {
                                    eprintln!("Error or server closed connection.");
                                    connection = None;
                                }
                            }
                        } else {
                            eprintln!("Not connected. Please use 'connect <host> <port>'.");
                        }
                    }
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break 'main_loop;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break 'main_loop;
            }
            Err(err) => {
                eprintln!("Error: {:?}", err);
                break 'main_loop;
            }
        }
    }
    Ok(())
}

fn is_pubsub_command(parts: &[String]) -> bool {
    if let Some(cmd) = parts.first() {
        let lower_cmd = cmd.to_ascii_lowercase();
        lower_cmd == "subscribe" || lower_cmd == "psubscribe"
    } else {
        false
    }
}
