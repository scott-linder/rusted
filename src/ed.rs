//! The editor

use std::collections::LinkedList;
use std::io::{self, Write};
use cmd::Cmd;
use error::Result;

#[derive(Debug)]
pub struct Ed<D, F, W>
    where D: FnMut(&str) -> Result<()>,
          F: FnMut(&str) -> Result<W>,
          W: Write {
    display: D,
    write: F,
    lines: LinkedList<String>,
    appending: bool,
}

impl<D, F, W> Ed<D, F, W>
    where D: FnMut(&str) -> Result<()>,
          F: FnMut(&str) -> Result<W>,
          W: Write {
    pub fn new(display: D, write: F) -> Ed<D, F, W> {
        Ed {
            display: display,
            write: write,
            lines: LinkedList::new(),
            appending: false,
        }
    }

    pub fn run_line(&mut self, s: &str) -> Result<()> {
        if self.appending {
            if s == "." {
                self.appending = false;
            } else {
                self.lines.push_back(s.to_string());
            }
        } else {
            match try!(s.parse()) {
                Cmd::Append(..) => self.appending = true,
                Cmd::Print(..) => {
                    for line in &self.lines {
                        try!(self.display.call_mut((&line[..],)));
                    }
                },
                Cmd::Write(..) => {
                    let mut writer = try!(self.write.call_mut(("test.txt",)));
                    for line in &self.lines {
                        try!(writeln!(&mut writer, "{}", line));
                    }
                    try!(writer.flush());
                },
                Cmd::Quit => {},
            }
        }
        Ok(())
    }
}
