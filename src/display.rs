use crate::cli::Args;
use spineldb::core::protocol::RespFrame;
use std::io; // Import Args

pub fn _print_resp_frame_formatted(
    frame: &RespFrame,
    writer: &mut dyn io::Write,
    level: usize,
) -> io::Result<()> {
    let indent = "   ".repeat(level);
    match frame {
        RespFrame::SimpleString(s) => write!(writer, "{}", s),
        RespFrame::Error(e) => write!(writer, "(error) {}", e),
        RespFrame::Integer(i) => write!(writer, "(integer) {}", i),
        RespFrame::BulkString(bytes) => {
            if let Ok(s) = String::from_utf8(bytes.to_vec()) {
                write!(writer, "\"{}\"", s)
            } else {
                write!(writer, "{:?}", bytes)
            }
        }
        RespFrame::Null => write!(writer, "(nil)"),
        RespFrame::NullArray => write!(writer, "(empty array)"),
        RespFrame::Array(frames) => {
            for (i, frame) in frames.iter().enumerate() {
                if i > 0 {
                    write!(writer, "\n{}", indent)?;
                }
                write!(writer, "{}) ", i + 1)?;
                _print_resp_frame_formatted(frame, writer, level + 1)?;
            }
            Ok(())
        }
    }
}

fn _print_raw_resp_frame(frame: &RespFrame, writer: &mut dyn io::Write) -> io::Result<()> {
    // This is a simplified raw output. A true raw output would be the exact bytes.
    // For now, we'll just print the debug representation.
    write!(writer, "{:?}", frame)
}

fn _print_csv_resp_frame(frame: &RespFrame, writer: &mut dyn io::Write) -> io::Result<()> {
    match frame {
        RespFrame::Array(frames) => {
            let mut row_values = Vec::new();
            for frame in frames {
                match frame {
                    RespFrame::BulkString(bytes) => {
                        row_values.push(String::from_utf8_lossy(bytes).to_string());
                    }
                    RespFrame::SimpleString(s) => {
                        row_values.push(s.clone());
                    }
                    RespFrame::Integer(i) => {
                        row_values.push(i.to_string());
                    }
                    RespFrame::Error(e) => {
                        row_values.push(format!("ERROR: {}", e));
                    }
                    RespFrame::Null | RespFrame::NullArray => {
                        row_values.push("".to_string());
                    }
                    _ => row_values.push("UNSUPPORTED".to_string()),
                }
            }
            write!(writer, "{}", row_values.join(","))
        }
        RespFrame::BulkString(bytes) => {
            write!(writer, "{}", String::from_utf8_lossy(bytes))
        }
        RespFrame::SimpleString(s) => {
            write!(writer, "{}", s)
        }
        RespFrame::Integer(i) => {
            write!(writer, "{}", i)
        }
        RespFrame::Error(e) => {
            write!(writer, "ERROR: {}", e)
        }
        RespFrame::Null | RespFrame::NullArray => {
            write!(writer, "")
        }
    }
}

fn _print_json_resp_frame(frame: &RespFrame, writer: &mut dyn io::Write) -> io::Result<()> {
    let json_value = resp_frame_to_json_value(frame);
    serde_json::to_writer_pretty(writer, &json_value).map_err(|e| io::Error::other(e.to_string()))
}

fn resp_frame_to_json_value(frame: &RespFrame) -> serde_json::Value {
    match frame {
        RespFrame::SimpleString(s) => serde_json::Value::String(s.clone()),
        RespFrame::Error(e) => serde_json::Value::String(format!("ERROR: {}", e)),
        RespFrame::Integer(i) => serde_json::Value::Number(serde_json::Number::from(*i)),
        RespFrame::BulkString(bytes) => {
            if let Ok(s) = String::from_utf8(bytes.to_vec()) {
                serde_json::Value::String(s)
            } else {
                serde_json::Value::Array(
                    bytes
                        .iter()
                        .map(|&b| serde_json::Value::Number(serde_json::Number::from(b)))
                        .collect(),
                )
            }
        }
        RespFrame::Null => serde_json::Value::Null,
        RespFrame::NullArray => serde_json::Value::Array(vec![]),
        RespFrame::Array(frames) => {
            serde_json::Value::Array(frames.iter().map(resp_frame_to_json_value).collect())
        }
    }
}

pub fn print_resp_frame_to_stdout(frame: &RespFrame, args: &Args) {
    let mut stdout = io::stdout();
    let mut stderr = io::stderr();
    let writer: &mut dyn io::Write = if matches!(frame, RespFrame::Error(_)) {
        &mut stderr
    } else {
        &mut stdout
    };

    let result = if args.raw {
        _print_raw_resp_frame(frame, writer)
    } else if args.csv {
        _print_csv_resp_frame(frame, writer)
    } else if args.json {
        _print_json_resp_frame(frame, writer)
    } else {
        _print_resp_frame_formatted(frame, writer, 0)
    };

    if let Err(e) = result {
        eprintln!("Error writing to output: {}", e);
    } else {
        // Add a newline for non-JSON output, as serde_json::to_writer_pretty adds its own.
        if !args.json
            && let Err(e) = writeln!(writer)
        {
            eprintln!("Error writing newline: {}", e);
        }
    }
}
