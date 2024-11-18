use std::io::{self, Read};

use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

fn main() {
    enable_raw_mode().unwrap();
    for byte in io::stdin().bytes() {
        match byte {
            Ok(byte) => {
                let c = byte as char;
                // control character: ASCII 0-31 and 127, like \n, \r
                if c.is_control() {
                    // '0:08b': the first 0 means to use the first param, namely ['byte']
                    // 08b: means to show in binary format, the min length of which is 8 bits
                    println!("Binary: {0:8b} ASCII: {0:03} \r", byte)
                } else {
                    println!("Binary: {0:08b} ASCII: {0:03} Character: {1:#?}\r", byte, c);
                }

                if c == 'q' {                  
                    break;
                }
            },
            Err(e) => {
                println!("Error while reading from stdin: {e}");
            },
        }
        disable_raw_mode().unwrap();
    }
}
