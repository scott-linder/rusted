//! The editor

use std::collections::LinkedList;
use std::old_io::IoResult;
use cmd::Cmd;
use error::Result;

#[derive(Debug)]
pub struct Ed<D, W>
    where D: FnMut(&str) -> IoResult<()>,
          W: FnMut(&str) -> IoResult<Box<Writer>> {
    display: D,
    write: W,
    lines: LinkedList<String>,
    appending: bool,
}

impl<D, W> Ed<D, W>
    where D: FnMut(&str) -> IoResult<()>,
          W: FnMut(&str) -> IoResult<Box<Writer>> {
    pub fn new(display: D, write: W) -> Ed<D, W> {
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
            let cmd: Cmd = try!(s.parse());
            match cmd {
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
