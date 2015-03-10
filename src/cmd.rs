//! Commands
//!
//! All command are invoked with a single alphabetic character, preceeded by
//! an optional address and trailed by an optional suffix.

use std::str::{FromStr, CharRange};
use error::{Error, Result};
use addr::{Range, Line};

/// A command.
#[derive(Debug, Eq, PartialEq)]
pub enum Cmd {
    Append(Option<Line>),
    Print(Option<Range>),
    Write(Option<Range>, Option<String>),
    Quit,
}

impl FromStr for Cmd {
    type Err = Error;

    fn from_str(s: &str) -> Result<Cmd> {
        let mut i = 0;
        while i < s.len() {
            let CharRange {ch, next} = s.char_range_at(i);
            if ch.is_alphabetic() {
                let address = &s[..i];
                let command = ch;
                let suffix = &s[next..];
                return match command {
                    'a' => Ok(Cmd::Append(try!(address.parse()))),
                    'p' => Ok(Cmd::Print(try!(address.parse()))),
                    'q' => Ok(Cmd::Quit),
                    'w' => Ok(Cmd::Write(try!(address.parse()), match suffix {
                        "" => None,
                        s => Some(s.to_string()),
                    })),
                    _ => Err(Error::InvalidCommand),
                }
            } else {
                i = next;
            }
        }
        Err(Error::NoCommand)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use addr::{Range, Line};

    #[test]
    fn simple() {
        assert_eq!("a".parse(), Ok(Cmd::Append(None)));
        assert_eq!("1a".parse(), Ok(Cmd::Append(Some(Line::Idx(1)))));
        assert!("1,2a".parse::<Cmd>().is_err());
        assert_eq!("p".parse(), Ok(Cmd::Print(None)));
        assert_eq!("1p".parse(), Ok(Cmd::Print(Some(Range(Line::Idx(1), Line::Idx(1))))));
        assert_eq!("1,2p".parse(), Ok(Cmd::Print(Some(Range(Line::Idx(1), Line::Idx(2))))));
        assert_eq!("q".parse(), Ok(Cmd::Quit));
    }
}
