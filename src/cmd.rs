//! Commands
//!
//! All command are invoked with a single alphabetic character, preceeded by
//! an optional address and trailed by an optional suffix.

use std::str::FromStr;
use error::{Error, Result};
use addr::{Range, OptionalRange, Line, OptionalLine};

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
        let mut ci = s.char_indices();
        loop {
            let (i, ch) = match ci.next() {
                Some(x) => x,
                None => break,
            };
            if ch.is_alphabetic() {
                let address = &s[..i];
                let command = ch;
                let suffix = ci.as_str();
                return match command {
                    'a' => Ok(Cmd::Append(address.parse::<OptionalLine>()?.0)),
                    'p' => Ok(Cmd::Print(address.parse::<OptionalRange>()?.0)),
                    'q' => Ok(Cmd::Quit),
                    'w' => Ok(Cmd::Write(address.parse::<OptionalRange>()?.0, match suffix {
                        "" => None,
                        s => Some(s.to_string()),
                    })),
                    _ => Err(Error::InvalidCommand),
                }
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
        assert_eq!("w".parse(), Ok(Cmd::Write(None, None)));
        assert_eq!("wfoo".parse(), Ok(Cmd::Write(None, Some("foo".to_string()))));
        assert_eq!("1,2wbar".parse(), Ok(Cmd::Write(Some(Range(Line::Idx(1), Line::Idx(2))), Some("bar".to_string()))));
    }
}
