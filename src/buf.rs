//! Text buffer

use std::collections::LinkedList;
use std::default::Default;
use std::fmt;
use cmd::Cmd;
use error::Result;

#[derive(Debug, Default)]
pub struct Buf {
    current: usize,
    lines: LinkedList<String>,
    appending: bool,
}

impl Buf {
    pub fn new() -> Buf {
        Default::default()
    }

    pub fn run(&mut self, s: &str) -> Result<()> {
        if self.appending {
            if s == "." {
                self.appending = false;
            } else {
                self.lines.push_back(s.to_string());
            }
        } else {
            let cmd: Cmd = try!(s.parse());
            match cmd {
                Cmd::Append(..) => self.appending = true,
                Cmd::Print(..) => print!("{}", self),
                Cmd::Quit => {},
            }
        }
        Ok(())
    }
}

impl fmt::Display for Buf {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in &self.lines {
            try!(writeln!(f, "{}", line));
        }
        Ok(())
    }
}
