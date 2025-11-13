use spineldb::core::protocol::RespFrame;
use std::io;

fn _print_resp_frame(
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
                _print_resp_frame(frame, writer, level + 1)?;
            }
            Ok(())
        }
    }
}

pub fn print_resp_frame(frame: &RespFrame, writer: &mut dyn io::Write) -> io::Result<()> {
    _print_resp_frame(frame, writer, 0)?;
    writeln!(writer)
}

pub fn print_resp_frame_to_stdout(frame: &RespFrame) {
    let mut stdout = io::stdout();
    let mut stderr = io::stderr();
    let writer: &mut dyn io::Write = if matches!(frame, RespFrame::Error(_)) {
        &mut stderr
    } else {
        &mut stdout
    };
    if let Err(e) = print_resp_frame(frame, writer) {
        eprintln!("Error writing to output: {}", e);
    }
}
