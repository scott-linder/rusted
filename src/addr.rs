//! Line addressing

use std::str::FromStr;
use error::{Error, Result};

/// An address into a list of lines.
///
/// A simple address identifies only a single line, but an address can also
/// refer to a range of lines.
#[derive(Copy, Debug, Eq, PartialEq)]
pub enum Addr {
    /// An address of a single line.
    Line(Line),
    /// An address of a range between two lines.
    Range(Line, Line),
}

/// Characters used to split a range up into its component line addresses.
static RANGE_SPLITTERS: [char; 2] = [',', ';'];

impl FromStr for Addr {
    type Err = Error;

    fn from_str(s: &str) -> Result<Addr> {
        // special cases
        match s {
            "," => return Ok(Addr::Range(Line::Idx(1), Line::Last)),
            ";" => return Ok(Addr::Range(Line::Current, Line::Last)),
            _ => {},
        }
        // normal cases
        let mut splits = s.split(&RANGE_SPLITTERS[..]);
        let l1: Line = match splits.next() {
            Some("") | None => return Ok(Addr::Line(Line::Current)),
            Some(l) => try!(l.parse()),
        };
        let l2: Line = match splits.next() {
            None => return Ok(Addr::Line(l1)),
            Some(l) => try!(l.parse()),
        };
        match splits.next() {
            None => Ok(Addr::Range(l1, l2)),
            Some(..) => return Err(Error::InvalidAddress),
        }
    }
}

/// A subset of possible addresses which refer to a single line.
#[derive(Copy, Debug, Eq, PartialEq)]
pub enum Line {
    /// The index of a line.
    Idx(usize),
    /// The current line.
    Current,
    /// The last line.
    Last,
}

impl FromStr for Line {
    type Err = Error;

    fn from_str(s: &str) -> Result<Line> {
        // special cases
        match s {
            "" => return Ok(Line::Current),
            "." => return Ok(Line::Current),
            "$" => return Ok(Line::Last),
            _ => {},
        }
        // normal cases
        match s.parse::<usize>() {
            Ok(i) => return Ok(Line::Idx(i)),
            Err(..) => return Err(Error::InvalidAddress),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {
        assert_eq!("".parse(), Ok(Addr::Line(Line::Current)));
        assert_eq!("0".parse(), Ok(Addr::Line(Line::Idx(0))));
        assert_eq!("0,0".parse(), Ok(Addr::Range(Line::Idx(0),Line::Idx(0))));
        assert_eq!("0;0".parse(), Ok(Addr::Range(Line::Idx(0),Line::Idx(0))));
        assert!("0,0,".parse::<Addr>().is_err())
    }

    #[test]
    fn line() {
        assert_eq!("10".parse(), Ok(Addr::Line(Line::Idx(10))));
    }

    #[test]
    fn range() {
        assert_eq!(",".parse(), Ok(Addr::Range(Line::Idx(1), Line::Last)));
        assert_eq!("0,1".parse(), Ok(Addr::Range(Line::Idx(0), Line::Idx(1))));
    }
}
