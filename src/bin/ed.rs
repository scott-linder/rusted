#![feature(old_io, old_path)]

extern crate rusted;

use std::old_io::stdio::stdin;
use std::old_io::{File, Truncate, Write};
use rusted::ed::Ed;

fn main() {
    let mut ed = Ed::new(|s| {
        println!("{}", s);
        Ok(())
    }, |s| {
        let file = try!(File::open_mode(&Path::new(s), Truncate, Write));
        Ok(Box::new(file))
    });
    let mut reader = stdin();
    for line in reader.lock().lines() {
        let line = line.unwrap();
        match ed.run_line(line.trim_right()) {
            Ok(()) => {},
            Err(..) => println!("?"),
        }
    }
}
