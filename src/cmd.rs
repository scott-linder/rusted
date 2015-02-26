//! Command

use std::str::FromStr;
use error::{Error, Result};
use addr::{Addr, Line};

/// An ed command.
#[derive(Copy, Debug, Eq, PartialEq)]
pub enum Cmd {
    Append(Line),
    Print(Addr),
    Quit,
}

impl FromStr for Cmd {
    type Err = Error;

    fn from_str(s: &str) -> Result<Cmd> {
        match s {
            "q" => return Ok(Cmd::Quit),
            _ => {},
        }
        return match s.find(|c: char| c.is_alphabetic()) {
            Some(i) => match &s[i..i+1] {
                "a" => Ok(Cmd::Append(try!(s[..i].parse()))),
                "p" => Ok(Cmd::Print(try!(s[..i].parse()))),
                _ => Err(Error::InvalidCommand),
            },
            None => return Err(Error::NoCommand),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use addr::{Addr, Line};

    #[test]
    fn simple() {
        assert_eq!("a".parse(), Ok(Cmd::Append(Line::Current)));
        assert_eq!("1a".parse(), Ok(Cmd::Append(Line::Idx(1))));
        assert!("1,2a".parse::<Cmd>().is_err());
        assert_eq!("p".parse(), Ok(Cmd::Print(Addr::Line(Line::Current))));
        assert_eq!("1p".parse(), Ok(Cmd::Print(Addr::Line(Line::Idx(1)))));
        assert_eq!("1,2p".parse(), Ok(Cmd::Print(Addr::Range(Line::Idx(1), Line::Idx(2)))));
        assert_eq!("q".parse(), Ok(Cmd::Quit));
    }
}
