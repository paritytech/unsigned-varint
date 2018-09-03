// Copyright 2018 Parity Technologies (UK) Ltd.
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of
// this software and associated documentation files (the "Software"), to deal in
// the Software without restriction, including without limitation the rights to
// use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
// the Software, and to permit persons to whom the Software is furnished to do so,
// subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
// FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS
// OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY,
// WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

use std::{self, fmt};

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    /// Not enough input bytes.
    Insufficient,
    /// Input bytes exceed maximum.
    Overflow,

    #[doc(hidden)]
    __Nonexhaustive
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Insufficient => f.write_str("not enough input bytes"),
            Error::Overflow => f.write_str("input bytes exceed maximum"),
            _ => f.write_str("__Nonexhaustive")
        }
    }
}

impl std::error::Error for Error {}

macro_rules! decode {
    ($buf:expr, $max_bytes:expr, $typ:ident) => {{
        let mut n = 0;
        for (i, b) in $buf.iter().cloned().enumerate() {
            let k = $typ::from(b & 0x7F);
            n |= k << (i * 7);
            if b & 0x80 == 0 {
                return Ok((n, &$buf[i+1..]))
            }
            if i == $max_bytes {
                return Err(Error::Overflow)
            }
        }
        Err(Error::Insufficient)
    }}
}

/// Decode the given slice as `u8`.
///
/// Returns the value and the remaining slice.
#[inline]
pub fn u8(buf: &[u8]) -> Result<(u8, &[u8]), Error> {
    decode!(buf, 1, u8)
}

/// Decode the given slice as `u16`.
///
/// Returns the value and the remaining slice.
#[inline]
pub fn u16(buf: &[u8]) -> Result<(u16, &[u8]), Error> {
    decode!(buf, 2, u16)
}

/// Decode the given slice as `u32`.
///
/// Returns the value and the remaining slice.
#[inline]
pub fn u32(buf: &[u8]) -> Result<(u32, &[u8]), Error> {
    decode!(buf, 4, u32)
}

/// Decode the given slice as `u64`.
///
/// Returns the value and the remaining slice.
#[inline]
pub fn u64(buf: &[u8]) -> Result<(u64, &[u8]), Error> {
    decode!(buf, 9, u64)
}

/// Decode the given slice as `u128`.
///
/// Returns the value and the remaining slice.
#[inline]
pub fn u128(buf: &[u8]) -> Result<(u128, &[u8]), Error> {
    decode!(buf, 18, u128)
}

/// Decode the given slice as `usize`.
///
/// Returns the value and the remaining slice.
#[inline]
#[cfg(target_pointer_width = "64")]
pub fn usize(buf: &[u8]) -> Result<(usize, &[u8]), Error> {
    u64(buf).map(|(n, i)| (n as usize, i))
}

/// Decode the given slice as `usize`.
///
/// Returns the value and the remaining slice.
#[inline]
#[cfg(target_pointer_width = "32")]
pub fn usize(buf: &[u8]) -> Result<(usize, &[u8]), Error> {
    u32(buf).map(|(n, i)| (n as usize, i))
}

