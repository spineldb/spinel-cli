use crate::display::print_resp_frame_to_stdout;
use anyhow::Result;
use futures::{SinkExt, stream::StreamExt};
use rustyline::{Editor, error::ReadlineError, history::DefaultHistory};
use spineldb::core::protocol::{RespFrame, RespFrameCodec};
use std::net::SocketAddr;
use tokio::net::TcpStream;
use tokio::signal;
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

    let mut in_pubsub_mode = false;

    loop {
        let prompt = if connection.is_some() {
            format!("{}> ", addr)
        } else {
            "not connected> ".to_string()
        };

        if in_pubsub_mode {
            if let Some(framed) = connection.as_mut() {
                tokio::select! {
                    _ = signal::ctrl_c() => {
                        println!("\nExiting Pub/Sub mode... Sending UNSUBSCRIBE.");

                        let unsubscribe_cmd = RespFrame::Array(vec![
                            RespFrame::BulkString("UNSUBSCRIBE".into()),
                        ]);

                        if let Err(e) = framed.send(unsubscribe_cmd).await {
                            eprintln!("Failed to send UNSUBSCRIBE command: {}. Connection may be in an inconsistent state.", e);
                            connection = None; // Assume connection is lost
                        } else {
                            // Wait for confirmation
                            if let Some(Ok(frame)) = framed.next().await {
                                print_resp_frame_to_stdout(&frame);
                            }
                        }

                        in_pubsub_mode = false;
                        println!("Returned to normal mode.");
                        continue;
                    }
                    result = framed.next() => {
                        match result {
                            Some(Ok(frame)) => print_resp_frame_to_stdout(&frame),
                            Some(Err(e)) => {
                                eprintln!("Error in Pub/Sub mode: {}", e);
                                connection = None;
                                in_pubsub_mode = false;
                            }
                            None => {
                                println!("Server closed connection in Pub/Sub mode.");
                                connection = None;
                                in_pubsub_mode = false;
                            }
                        }
                    }
                }
            } else {
                // This should ideally not be reached if in_pubsub_mode is true
                eprintln!("Error: In Pub/Sub mode but no active connection.");
                in_pubsub_mode = false;
            }
        } else {
            match rl.readline(&prompt) {
                Ok(line) => {
                    let line = line.trim();
                    if line.is_empty() {
                        continue;
                    }
                    let _ = rl.add_history_entry(line);

                    let parts = shlex::split(line).unwrap_or_default();
                    let cmd = parts
                        .first()
                        .map(|s| s.to_ascii_lowercase())
                        .unwrap_or_default();

                    if cmd == "quit" || cmd == "exit" {
                        break;
                    }

                    if let Some(framed) = connection.as_mut() {
                        if is_pubsub_command(&parts) {
                            in_pubsub_mode = true;
                        }

                        let command = RespFrame::Array(
                            parts
                                .into_iter()
                                .map(|s| RespFrame::BulkString(s.into()))
                                .collect(),
                        );

                        if let Err(e) = framed.send(command).await {
                            eprintln!("Failed to send command: {}", e);
                            connection = None; // Assume connection is lost
                            continue;
                        }

                        if let Some(Ok(frame)) = framed.next().await {
                            print_resp_frame_to_stdout(&frame);
                        } else {
                            eprintln!("Error or server closed connection.");
                            connection = None; // Assume connection is lost
                        }
                    } else if cmd == "connect" {
                        let host = parts.get(1).map(|s| s.as_str()).unwrap_or("127.0.0.1");
                        let port = parts
                            .get(2)
                            .and_then(|s| s.parse::<u16>().ok())
                            .unwrap_or(7878);
                        match try_connect(host, port).await {
                            Ok(conn) => {
                                connection = Some(conn);
                                addr = format!("{}:{}", host, port).parse()?;
                            }
                            Err(e) => eprintln!("Failed to connect: {}", e),
                        }
                    } else {
                        eprintln!("Not connected. Please use 'connect <host> <port>'.");
                    }
                }
                Err(ReadlineError::Interrupted) => {
                    println!("CTRL-C");
                    break;
                }
                Err(ReadlineError::Eof) => {
                    println!("CTRL-D");
                    break;
                }
                Err(err) => {
                    eprintln!("Error: {:?}", err);
                    break;
                }
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
