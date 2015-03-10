//! Addressing
//!
//! Parsing can result in a true error via `Err`, or can succeed, but indicate
//! that no address was specified via `Ok(None)`, in which case the calling
//! code can make a decision about what the "default" address should be.

use std::str::FromStr;
use error::{Error, Result};

/// An address spanning between two lines.
#[derive(Copy, Debug, Eq, PartialEq)]
pub struct Range(pub Line, pub Line);

impl Range {
    pub fn repeat(line: Line) -> Range {
        Range(line, line)
    }
}

/// Characters used to split a range up into its component line addresses.
static RANGE_SPLITTERS: [char; 2] = [',', ';'];

impl FromStr for Option<Range> {
    type Err = Error;

    fn from_str(s: &str) -> Result<Option<Range>> {
        // special cases
        match s {
            "," => return Ok(Some(Range(Line::Idx(1), Line::Last))),
            ";" => return Ok(Some(Range(Line::Current, Line::Last))),
            _ => {},
        }
        // normal cases
        let mut l1 = None;
        let mut l2 = None;
        for addr in s.split(&RANGE_SPLITTERS[..]) {
            l1 = l2;
            l2 = try!(addr.parse());
        }
        let (l1, l2) = match (l1, l2) {
            (None, None) => return Ok(None),
            (Some(l1), None) => (l1, l1),
            (None, Some(l2)) => (l2, l2),
            (Some(l1), Some(l2)) => (l1, l2),
        };
        Ok(Some(Range(l1, l2)))
    }
}

/// An address specifying a single line.
#[derive(Copy, Debug, Eq, PartialEq)]
pub enum Line {
    /// The index of a line.
    Idx(usize),
    /// The current line.
    Current,
    /// The last line.
    Last,
}

impl FromStr for Option<Line> {
    type Err = Error;

    fn from_str(s: &str) -> Result<Option<Line>> {
        // special cases
        match s {
            "" => return Ok(None),
            "." => return Ok(Some(Line::Current)),
            "$" => return Ok(Some(Line::Last)),
            _ => {},
        }
        // normal cases
        match s.parse::<usize>() {
            Ok(i) => return Ok(Some(Line::Idx(i))),
            Err(..) => return Err(Error::InvalidAddress),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn none() {
        assert_eq!("".parse::<Option<Range>>(), Ok(None));
    }

    #[test]
    fn one() {
        let one = Ok(Some(Range(Line::Idx(1), Line::Idx(1))));
        assert_eq!("1".parse(), one);
        assert_eq!("1,".parse(), one);
        assert_eq!("1,1".parse(), one);
        assert_eq!("1,1,".parse(), one);
        assert_eq!("1,1,1".parse(), one);
    }

    #[test]
    fn two() {
        let two = Ok(Some(Range(Line::Idx(1), Line::Idx(2))));
        assert_eq!("1,2".parse(), two);
    }

    #[test]
    fn special() {
        assert_eq!(".".parse(), Ok(Some(Line::Current)));
        assert_eq!("$".parse(), Ok(Some(Line::Last)));
        assert_eq!(",".parse(), Ok(Some(Range(Line::Idx(1), Line::Last))));
        assert_eq!(";".parse(), Ok(Some(Range(Line::Current, Line::Last))));
    }
}
