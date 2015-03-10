#![feature(io)]

extern crate rusted;

use std::io::{stdin, stdout, Write, BufReadExt};
use std::fs::File;
use rusted::ed::Ed;

fn main() {
    let write = stdout();
    let mut ed = Ed::new(|s| {
        try!(write!(&mut write.lock(), "{}", s));
        Ok(())
    }, |s| {
        let file = try!(File::create(s));
        Ok(file)
    });
    let read = stdin();
    for line in read.lock().lines() {
        let line = line.unwrap();
        match ed.run_line(line.trim_right()) {
            Ok(()) => {},
            Err(..) => println!("?"),
        }
    }
}
