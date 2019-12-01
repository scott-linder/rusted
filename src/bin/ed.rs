extern crate rusted;

use std::io::{stdin, stdout, Write};
use std::fs::File;
use rusted::ed::Ed;

fn main() {
    let mut write = stdout();
    let mut ed = Ed::new(|s| {
        writeln!(&mut write.lock(), "{}", s)?;
        write.flush()?;
        Ok(())
    }, |s| {
        let file = File::create(s)?;
        Ok(file)
    });
    let mut line = String::new();
    loop {
        stdin().read_line(&mut line).unwrap();
        match ed.run_line(line.trim_end()) {
            Ok(()) => {},
            Err(..) => println!("?"),
        }
    }
}
