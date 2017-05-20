// Copyright 2017 Dmytro Milinevskyi <dmilinevskyi@gmail.com>

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at

// http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![warn(missing_docs)]

//! # Unbytify is a library to parse and represent digital units
//!
//! # Installation
//! To start using `unbytify` it's enough to just enable it in your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! unbytify = "0.1"
//! ```
//!
//! A simple example should be self-explanatory:
//!
//! ```rust
//! extern crate unbytify;
//! use unbytify::*;
//!
//! fn main() {
//!     assert_eq!(unbytify("1.5K"), Ok(1024 + 512));
//!     assert_eq!(bytify(1024 + 512), (1.5, "KiB"));
//! }
//! ```

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref SUFFIXES: Vec<&'static str> = vec![
        "B",
        "KiB",
        "MiB",
        "GiB",
        "TiB",
        "PiB",
        "EiB",
    ];

    static ref B_POWERS: Vec<u64> = (0..SUFFIXES.len()).map(|x| 1024u64.pow(x as u32)).collect();
    static ref B_POWERS_F: Vec<f64> = B_POWERS.iter().map(|x| *x as f64).collect();
}

/// Represents parse failure
#[derive(PartialEq, PartialOrd, Clone, Copy, Debug)]
pub enum ParseError {
    /// Invalid input value
    Invalid,
    /// An overflow occurred
    Overflow,
}

/// Converts human readable number of bytes to a byte count
///
/// # Example
/// ```rust
/// extern crate unbytify;
/// use unbytify::*;
///
/// fn main() {
///     assert_eq!(unbytify("1.5K"), Ok(1024 + 512));
///     assert_eq!(unbytify("1O"), Err(ParseError::Invalid));
///     assert_eq!(unbytify("42E"), Err(ParseError::Overflow));
/// }
/// ```
pub fn unbytify(value: &str) -> Result<u64, ParseError> {
    lazy_static! {
        static ref SUFFIXES_LOWER: Vec<String> = SUFFIXES.iter().map(|x| x[..1].to_lowercase()).collect();
    }

    let value = value.trim().to_lowercase();

    if &value[..1] == "-" {
        return Err(ParseError::Invalid);
    }

    if let Ok(x) = value.parse::<u64>() {
        return Ok(x);
    }

    for (idx, suffix) in SUFFIXES_LOWER.iter().enumerate() {
        let mut res = value.split(suffix);

        let val: f64 = match res.next() {
            Some(x) => {
                match x.trim().parse() {
                    Ok(x) => x,
                    _ => continue,
                }
            },
            _ => continue,
        };

        match res.next() {
            Some(rest) => {
                if !rest.is_empty() && rest != "b" && rest != "ib" {
                    return Err(ParseError::Invalid);
                }
            },
            _ => {
                if val.ceil() as u64 != val as u64 {
                    return Err(ParseError::Invalid);
                }
            },
        };

        if val as u64 == 0 {
            return Ok(0);
        }

        if val.ceil() as u64 == val as u64 {
            return (val as u64)
                .checked_mul(B_POWERS[idx])
                .ok_or(ParseError::Overflow);
        }

        let val = val * B_POWERS_F[idx];

        if val as u64 == 0 {
            return Err(ParseError::Overflow);
        }

        return Ok(val.floor() as u64);
    }

    Err(ParseError::Invalid)
}

/// Converts a byte count to a human readable value
///
/// Returns a tuple of converted value with a suffix.
///
/// # Example
/// ```rust
/// extern crate unbytify;
/// use unbytify::*;
///
/// fn main() {
///     assert_eq!(bytify(512), (512.0, "B"));
///     assert_eq!(bytify(1024 + 512), (1.5, "KiB"));
/// }
/// ```
pub fn bytify(value: u64) -> (f64, &'static str) {
    let (mut val, mut idx) = (0.0, 0);

    if value > 0 {
        idx = (value as f64).log(1024.0).floor() as usize;
        val = value as f64 / B_POWERS_F[idx];
    }

    ((val * 1000.0).round() / 1000.0, SUFFIXES[idx])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unbytify() {
        assert_eq!(unbytify("-1"), Err(ParseError::Invalid));
        assert_eq!(unbytify("- 1"), Err(ParseError::Invalid));
        assert_eq!(unbytify("-1.0"), Err(ParseError::Invalid));
        assert_eq!(unbytify("- 1.0"), Err(ParseError::Invalid));
        assert_eq!(unbytify("0"), Ok(0));
        assert_eq!(unbytify("0K"), Ok(0));
        assert_eq!(unbytify("16E"), Err(ParseError::Overflow));
        assert_eq!(unbytify("15.75E"), Ok(15 * 1024u64.pow(6) + 3 * (1024u64.pow(6) / 4)));

        assert_eq!(unbytify("1"), Ok(1));
        assert_eq!(unbytify("1.0"), Ok(1));
        assert_eq!(unbytify("1K"), Ok(1024));
        assert_eq!(unbytify(" 1 K "), Ok(1024));
        assert_eq!(unbytify("1.0K"), Ok(1024));
        assert_eq!(unbytify("1.5K"), Ok(1024 + 512));
        assert_eq!(unbytify("1.25K"), Ok(1024 + 256));
        assert_eq!(unbytify("1.2K"), Ok((1.2 * 1024.0) as u64));

        assert_eq!(unbytify("16.000001E"), Err(ParseError::Overflow));
        assert_eq!(unbytify("42E"), Err(ParseError::Overflow));
        assert_eq!(unbytify("1.5"), Err(ParseError::Invalid));
        assert_eq!(unbytify("1O"), Err(ParseError::Invalid));

        for (idx, suffix) in SUFFIXES.iter().enumerate() {
            let val = 4;
            let expected = Ok(val * 2u64.pow(10 * idx as u32));

            assert_eq!(unbytify(&(val.to_string() + suffix)), expected);
            assert_eq!(unbytify(&(val.to_string() + &suffix[..1])), expected);
            assert_eq!(unbytify(&(val.to_string() + &suffix.to_lowercase())), expected);
            assert_eq!(unbytify(&(val.to_string() + &suffix[..1].to_lowercase())), expected);
        }
    }

    #[test]
    fn test_bytify() {
        assert_eq!(bytify(0), (0.0, "B"));
        assert_eq!(bytify(1024), (1.0, "KiB"));
        assert_eq!(bytify(1025), (1.001, "KiB"));
        assert_eq!(bytify(1024u64.pow(2)), (1.0, "MiB"));
        assert_eq!(bytify(std::u64::MAX), (16.0, "EiB"));

        for (idx, suffix) in SUFFIXES.iter().enumerate() {
            let val = 4;
            let expected = (val as f64, *suffix);

            assert_eq!(bytify(val * B_POWERS[idx]), expected);
        }
    }

    #[test]
    fn test_rebytify() {
        for idx in 0..SUFFIXES.len() {
            let val = 4;
            let formatted = (|(a, b)| format!("{} {}", a, b))(bytify(val * B_POWERS[idx]));

            assert_eq!(unbytify(&formatted).unwrap(), val * B_POWERS[idx]);
        }
    }
}
