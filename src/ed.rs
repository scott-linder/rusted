//! The editor

use std::collections::LinkedList;
use std::io::Write;
use cmd::Cmd;
use error::{Error, Result};
use addr::{Range, Line};

/// An instance of the editor.
///
/// Manages a buffer, parsing and executing commands.
/// Delegates implementation of some functionality to callbacks.
#[derive(Debug)]
pub struct Ed<D, F, W>
    where D: FnMut(&str) -> Result<()>,
          F: FnMut(&str) -> Result<W>,
          W: Write {
    display: D,
    get_file: F,
    lines: LinkedList<String>,
    appending: bool,
    current: usize,
    filename: Option<String>,
}

impl<D, F, W> Ed<D, F, W>
    where D: FnMut(&str) -> Result<()>,
          F: FnMut(&str) -> Result<W>,
          W: Write {

    pub fn new(display: D, get_file: F) -> Ed<D, F, W> {
        Ed {
            display: display,
            get_file: get_file,
            lines: LinkedList::new(),
            appending: false,
            current: 0,
            filename: None,
        }
    }

    pub fn run_line(&mut self, s: &str) -> Result<()> {
        if self.appending {
            if s == "." {
                self.appending = false;
            } else {
                if self.current > self.lines.len() {
                    return Err(Error::InvalidAddress);
                }
                let mut tail = self.lines.split_off(self.current);
                self.lines.push_back(s.to_string());
                self.lines.append(&mut tail);
                self.current += 1;
            }
        } else {
            match s.parse()? {
                Cmd::Append(line) => {
                    let line = line.unwrap_or(Line::Current);
                    match line {
                        Line::Idx(i) => if i <= self.lines.len() {
                            self.current = i;
                        } else {
                            return Err(Error::InvalidAddress);
                        },
                        Line::Current => {},
                        Line::Last => self.current = self.lines.len(),
                    }
                    self.appending = true;
                }
                Cmd::Print(range) => {
                    let Range(from, to) = range.unwrap_or(Range::repeat(Line::Current));
                    let from = self.line_number(from)?;
                    let to = self.line_number(to)?;
                    for (i, line) in self.lines.iter().enumerate() {
                        let i = i + 1;
                        if i >= from && i <= to {
                            (self.display)(&line[..])?;
                        }
                    }
                },
                Cmd::Write(range, filename) => {
                    if let Some(filename) = filename {
                        self.filename = Some(filename.to_string());
                    }
                    let mut write = (self.get_file)(match self.filename {
                        Some(ref s) => &*s,
                        None => return Err(Error::NoFilename),
                    })?;
                    let Range(from, to) = range.unwrap_or(Range(Line::Idx(1), Line::Last));
                    let from = self.line_number(from)?;
                    let to = self.line_number(to)?;
                    for (i, line) in self.lines.iter().enumerate() {
                        let i = i + 1;
                        if i >= from && i <= to {
                            writeln!(&mut write, "{}", line)?;
                        }
                    }
                    write.flush()?;
                },
                Cmd::Quit => {},
            }
        }
        Ok(())
    }

    /// Convert a Line into a concrete line number.
    fn line_number(&self, line: Line) -> Result<usize> {
        Ok(match line {
            Line::Idx(i) => if i > 0 && i <= self.lines.len() {
                i
            } else {
                return Err(Error::InvalidAddress);
            },
            Line::Current => self.current,
            Line::Last => self.lines.len(),
        })
    }
}
