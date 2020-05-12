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

//! Basic unsigned-varint encoding.

macro_rules! encode {
    ($($type:ident, $name:expr, $buf_len:ident);*) => {
        $(
            #[doc = "Encode the given "]
            #[doc = $name]
            #[doc = " into the given byte array.\n\n"]
            #[doc = "Returns the slice of encoded bytes."]
            #[inline]
            pub fn $type(number: $type, buf: &mut [u8; $buf_len]) -> &[u8] {
                let mut n = number;
                let mut i = 0;
                for b in buf.iter_mut() {
                    *b = n as u8 | 0x80;
                    n >>= 7;
                    if n == 0 {
                        *b &= 0x7f;
                        break
                    }
                    i += 1
                }
                debug_assert_eq!(n, 0);
                &buf[0..=i]
            }
        )*
    }
}

encode! {
    u8,   "`u8`",   U8_LEN;
    u16,  "`u16`",  U16_LEN;
    u32,  "`u32`",  U32_LEN;
    u64,  "`u64`",  U64_LEN;
    u128, "`u128`", U128_LEN
}


/// Encode the given `usize` into the given byte array.
///
/// Returns the slice of encoded bytes.
#[inline]
#[cfg(target_pointer_width = "64")]
pub fn usize(number: usize, buf: &mut [u8; USIZE_LEN]) -> &[u8] {
    u64(number as u64, buf)
}

/// Encode the given `usize` into the given byte array.
///
/// Returns the slice of encoded bytes.
#[inline]
#[cfg(target_pointer_width = "32")]
pub fn usize(number: usize, buf: &mut [u8; USIZE_LEN]) -> &[u8] {
    u32(number as u32, buf)
}

macro_rules! buffer {
    ($($func_name:ident, $type:expr, $buf_len:ident);*) => {
        $(
            #[doc = "Create new array buffer for encoding of "]
            #[doc = $type]
            #[doc = " values."]
            #[inline]
            pub fn $func_name() -> [u8; $buf_len] {
                [0; $buf_len]
            }
        )*
    }
}

buffer! {
    u8_buffer,    "`u8`",    U8_LEN;
    u16_buffer,   "`u16`",   U16_LEN;
    u32_buffer,   "`u32`",   U32_LEN;
    u64_buffer,   "`u64`",   U64_LEN;
    u128_buffer,  "`u128`",  U128_LEN;
    usize_buffer, "`usize`", USIZE_LEN
}

// Required lengths of encoding buffers:

const U8_LEN: usize = 2;
const U16_LEN: usize = 3;
const U32_LEN: usize = 5;
const U64_LEN: usize = 10;
const U128_LEN: usize = 19;

#[cfg(target_pointer_width = "64")]
const USIZE_LEN: usize = U64_LEN;

#[cfg(target_pointer_width = "32")]
const USIZE_LEN: usize = U32_LEN;

