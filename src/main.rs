use std::env;
use std::fs;
use std::io::{self, Read};

const COLOR_NORMAL: &str = "\x1b[0m";
const COLOR_RED: &str = "\x1b[1;31m";
const COLOR_GREEN: &str = "\x1b[1;32m";
const COLOR_YELLOW: &str = "\x1b[1;33m";
const COLOR_BLUE: &str = "\x1b[1;34m";

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("no file specified, exiting.");
    }
    let file = fs::File::open(&args[1])?;
    let mut reader = io::BufReader::new(file);

    let mut buffer: Vec<u8> = Vec::new();

    reader.read_to_end(&mut buffer)?;

    let mut offset = 0;
    let length = buffer.len();
    let mut output = String::with_capacity(length * 3);

    while offset < length {
        output.push_str(format!("\n{}0x{:08x}{} ", COLOR_YELLOW, offset, COLOR_NORMAL).as_str());
        let mut ascii_data = String::with_capacity(16);
        for i in 0..16 {
            if offset + i >= length {
                for _ in 0..(16 - i) {
                    output.push_str("   ");
                }
                break;
            } else {
                let byte = buffer[offset + i];
                let c: char;
                if (byte < 32) || (byte > 127) {
                    c = '.';

                    output.push_str(
                        format!(" {}{:02x}{}", COLOR_RED, buffer[offset + i], COLOR_NORMAL)
                            .as_str(),
                    );
                } else {
                    c = byte as char;
                    if c == ' ' {
                        output.push_str(
                            format!(" {}{:02x}{}", COLOR_BLUE, buffer[offset + i], COLOR_NORMAL)
                                .as_str(),
                        );
                    } else {
                        output.push_str(
                            format!(" {}{:02x}{}", COLOR_GREEN, buffer[offset + i], COLOR_NORMAL)
                                .as_str(),
                        );
                    }
                }
                ascii_data.push_str(COLOR_YELLOW);
                ascii_data.push(c);
                ascii_data.push_str(COLOR_NORMAL);
            }
        }
        output.push_str(format!("  {}", ascii_data).as_str());
        offset += 16;
    }
    println!("{}\n", output);

    Ok(())
}
