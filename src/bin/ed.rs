#![feature(old_io)]

extern crate rusted;

use std::old_io::stdio::stdin;
use rusted::ed::Ed;

fn main() {
    let mut ed = Ed::new(|s| {
        println!("{}", s);
        Ok(())
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
