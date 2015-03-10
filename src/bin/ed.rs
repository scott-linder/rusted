#![feature(io)]

extern crate rusted;

use std::io::{stdin, BufReadExt};
use std::fs::File;
use rusted::ed::Ed;

fn main() {
    let mut ed = Ed::new(|s| {
        println!("{}", s);
        Ok(())
    }, |s| {
        let file = try!(File::create(s));
        Ok(Box::new(file))
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
