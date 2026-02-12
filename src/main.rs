use std::fs::File;
use std::io::{BufReader, Read};
use std::process::ExitCode;

trait IsAsciiPrintable {
    fn is_ascii_printable(&self) -> bool;
}

impl IsAsciiPrintable for u8 {
    fn is_ascii_printable(&self) -> bool {
        matches!(self, 32..=126)
    }
}


fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("give file name...");
        return ExitCode::FAILURE;
    }
    let mut fileslop = BufReader::new(File::open(&args[1]).unwrap());
    let mut buffer = [0u8; 10240];
    while let Ok(bytes_read) = fileslop.read(&mut buffer) {
        if bytes_read == 0 {
            break;
        }
        for chunk in buffer[..bytes_read].chunks(16) {
             for byte in chunk {
                 print!("{:02x} ", byte);
             }
             print!("| ");
             for byte in chunk {
                 if byte.is_ascii_printable() {
                     print!("{}", *byte as char);
                 } else {
                     print!(".");
                 }
             }
             println!()
        }
    }

    ExitCode::SUCCESS
}
