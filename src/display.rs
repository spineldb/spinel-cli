use spineldb::core::protocol::RespFrame;
use std::io;

pub fn print_resp_frame(frame: &RespFrame, writer: &mut dyn io::Write) -> io::Result<()> {
    match frame {
        RespFrame::SimpleString(s) => writeln!(writer, "{}", s),
        RespFrame::Error(e) => writeln!(writer, "(error) {}", e),
        RespFrame::Integer(i) => writeln!(writer, "(integer) {}", i),
        RespFrame::BulkString(bytes) => {
            if let Ok(s) = String::from_utf8(bytes.to_vec()) {
                writeln!(writer, "\"{}\"", s)
            } else {
                writeln!(writer, "{:?}", bytes)
            }
        }
        RespFrame::Null => writeln!(writer, "(nil)"),
        RespFrame::NullArray => writeln!(writer, "(empty array)"),
        RespFrame::Array(frames) => {
            for (i, frame) in frames.iter().enumerate() {
                write!(writer, "{}. ", i + 1)?;
                print_resp_frame(frame, writer)?;
            }
            Ok(())
        }
    }
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
