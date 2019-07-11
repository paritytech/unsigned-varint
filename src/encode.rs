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

macro_rules! encode {
    ($number:expr, $buf:expr) => {{
        let mut n = $number;
        let mut i = 0;
        for b in $buf.iter_mut() {
            *b = n as u8 | 0x80;
            n >>= 7;
            if n == 0 {
                *b &= 0x7f;
                break
            }
            i += 1
        }
        debug_assert_eq!(n, 0);
        &$buf[0..=i]
    }}
}


/// Encode the given `u8` into the given byte array.
///
/// Returns the slice of encoded bytes.
#[inline]
pub fn u8(number: u8, buf: &mut [u8; U8_BUF_LEN]) -> &[u8] {
    encode!(number, buf)
}

/// Encode the given `u16` into the given byte array.
///
/// Returns the slice of encoded bytes.
#[inline]
pub fn u16(number: u16, buf: &mut [u8; U16_BUF_LEN]) -> &[u8] {
    encode!(number, buf)
}

/// Encode the given `u32` into the given byte array.
///
/// Returns the slice of encoded bytes.
#[inline]
pub fn u32(number: u32, buf: &mut [u8; U32_BUF_LEN]) -> &[u8] {
    encode!(number, buf)
}

/// Encode the given `u64` into the given byte array.
///
/// Returns the slice of encoded bytes.
#[inline]
pub fn u64(number: u64, buf: &mut [u8; U64_BUF_LEN]) -> &[u8] {
    encode!(number, buf)
}

/// Encode the given `u128` into the given byte array.
///
/// Returns the slice of encoded bytes.
#[inline]
pub fn u128(number: u128, buf: &mut [u8; U128_BUF_LEN]) -> &[u8] {
    encode!(number, buf)
}

/// Encode the given `usize` into the given byte array.
///
/// Returns the slice of encoded bytes.
#[inline]
#[cfg(target_pointer_width = "64")]
pub fn usize(number: usize, buf: &mut [u8; USIZE_BUF_LEN]) -> &[u8] {
    u64(number as u64, buf)
}

/// Encode the given `usize` into the given byte array.
///
/// Returns the slice of encoded bytes.
#[inline]
#[cfg(target_pointer_width = "32")]
pub fn usize(number: usize, buf: &mut [u8; USIZE_BUF_LEN]) -> &[u8] {
    u32(number as u32, buf)
}

/// Create new array buffer for encoding of `u8` values.
#[inline]
pub fn u8_buffer() -> [u8; U8_BUF_LEN] {
    [0; U8_BUF_LEN]
}

/// Create new array buffer for encoding of `u16` values.
#[inline]
pub fn u16_buffer() -> [u8; U16_BUF_LEN] {
    [0; U16_BUF_LEN]
}

/// Create new array buffer for encoding of `u32` values.
#[inline]
pub fn u32_buffer() -> [u8; U32_BUF_LEN] {
    [0; U32_BUF_LEN]
}

/// Create new array buffer for encoding of `u64` values.
#[inline]
pub fn u64_buffer() -> [u8; U64_BUF_LEN] {
    [0; U64_BUF_LEN]
}

/// Create new array buffer for encoding of `u128` values.
#[inline]
pub fn u128_buffer() -> [u8; U128_BUF_LEN] {
    [0; U128_BUF_LEN]
}

/// Create new array buffer for encoding of `usize` values.
#[inline]
pub fn usize_buffer() -> [u8; USIZE_BUF_LEN] {
    [0; USIZE_BUF_LEN]
}

/// Required buffer length to encode a `u8`.
pub const U8_BUF_LEN: usize = 2;

/// Required buffer length to encode a `u16`.
pub const U16_BUF_LEN: usize = 3;

/// Required buffer length to encode a `u32`.
pub const U32_BUF_LEN: usize = 5;

/// Required buffer length to encode a `u64`.
pub const U64_BUF_LEN: usize = 10;

/// Required buffer length to encode a `u128`.
pub const U128_BUF_LEN: usize = 19;

/// Required buffer length to encode a `usize`.
#[cfg(target_pointer_width = "64")]
pub const USIZE_BUF_LEN: usize = U64_BUF_LEN;

/// Required buffer length to encode a `usize`.
#[cfg(target_pointer_width = "32")]
pub const USIZE_BUF_LEN: usize = U32_BUF_LEN;

