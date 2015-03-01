//! The editor

use std::collections::LinkedList;
use std::old_io::IoResult;
use cmd::Cmd;
use error::Result;

#[derive(Debug)]
pub struct Ed<D: FnMut(&str) -> IoResult<()>> {
    display: D,
    lines: LinkedList<String>,
    appending: bool,
}

impl<D: FnMut(&str) -> IoResult<()>> Ed<D> {
    pub fn new(display: D) -> Ed<D> {
        Ed {
            display: display,
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
            let cmd: Cmd = try!(s.parse());
            match cmd {
                Cmd::Append(..) => self.appending = true,
                Cmd::Print(..) => {
                    for line in &self.lines {
                        try!(self.display.call_mut((&line[..],)));
                    }
                },
                Cmd::Quit => {},
            }
        }
        Ok(())
    }
}
