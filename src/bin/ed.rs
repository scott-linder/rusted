#![feature(old_io)]

extern crate rusted;

use rusted::buf::Buf;
use std::old_io::stdio::stdin;

fn main() {
    let mut buf = Buf::new();
    let mut reader = stdin();
    for line in reader.lock().lines() {
        let line = line.unwrap();
        match buf.run(line.trim()) {
            Ok(()) => {},
            Err(..) => println!("?"),
        }
    }
}
