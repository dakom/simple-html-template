use std::error::Error as ErrorTrait;
use std::fmt;

/// Errors that occured while finding and replacing
///
/// This implements the [`Display`](std::fmt::Display) trait and can be printed
/// nicely that way. There is also [`Self::into_inner`](Errors::into_inner) if
/// you need more control.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Errors {
    pub(crate) inner: Vec<Error>,
}

impl Errors {
    /// Get error list
    pub fn into_inner(self) -> Vec<Error> {
        self.inner
    }
}

impl ErrorTrait for Errors {}

// This is awful but the results are pretty
impl fmt::Display for Errors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let uncloseds = self.inner.iter().filter_map(|e| {
            if let Error::Unclosed(i) = e {
                Some(i)
            } else {
                None
            }
        });

        let missings = self.inner.iter().filter_map(|e| {
            if let Error::Missing(k) = e {
                Some(k)
            } else {
                None
            }
        });

        let extras = self.inner.iter().filter_map(|e| {
            if let Error::Extra(k) = e {
                Some(k)
            } else {
                None
            }
        });

        let uncloseds_count = uncloseds.clone().count();
        let missings_count = missings.clone().count();
        let extras_count = extras.clone().count();

        format_error(
            f,
            "unclosed delimiter opened at byte",
            uncloseds,
            uncloseds_count,
            false,
        )?;

        if uncloseds_count > 0 && missings_count > 0 {
            write!(f, "; ")?;
        }

        format_error(f, "missing key", missings, missings_count, true)?;

        if (uncloseds_count > 0 || missings_count > 0) && extras_count > 0 {
            write!(f, "; ")?;
        }

        format_error(f, "extraneous key", extras, extras_count, true)?;

        Ok(())
    }
}

// This is also awful but I don't think it can really be better
fn format_error(
    f: &mut fmt::Formatter<'_>,
    problem: &str,
    iter: impl Iterator<Item = impl fmt::Display>,
    count: usize,
    quotes: bool,
) -> fmt::Result {
    if count > 0 {
        let s = if count != 1 {
            "s"
        } else {
            ""
        };

        write!(f, "{}{}: ", problem, s)?;
        for (i, error) in iter.enumerate() {
            let sep = if i + 1 == count {
                ""
            } else if i + 2 == count {
                if count == 2 {
                    " and "
                } else {
                    ", and "
                }
            } else {
                ", "
            };

            if quotes {
                write!(f, "\"{}\"{}", error, sep)?;
            } else {
                write!(f, "{}{}", error, sep)?;
            }
        }
    }

    Ok(())
}

/// A single specific error
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Error {
    /// A key was found in the template but no value was provided
    ///
    /// Holds the name of the offending key
    Missing(String),

    /// A key-value pair was given but never used in the template
    ///
    /// Holds the name of the offending key
    Extra(String),

    /// A key-begin delimeter was found but there was no matching key-close
    /// delimiter
    ///
    /// Holds the zero-indexed byte position of the beginning of the opening
    /// delimiter
    Unclosed(usize),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Missing(k) => write!(f, "missing key \"{}\"", k),
            Self::Extra(k) => write!(f, "extraneous key \"{}\"", k),
            Self::Unclosed(i) => {
                write!(f, "unclosed delimitor opened at byte {}", i)
            }
        }
    }
}
