// Copyright 2019 Parity Technologies (UK) Ltd.
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

use crate::{encode, decode};
use futures::prelude::*;
use std::{fmt, io};

macro_rules! encode_decode_impls {
    ($mod: ident, $typ: ty, $enc: expr, $dec: expr, $arr: expr) => {
        pub mod $mod {
            use super::*;

            /// Write an unsigned varint value to the given [`AsyncWrite`] impl.
            pub async fn write<W>(item: $typ, dest: &mut W) -> Result<(), io::Error>
            where
                W: AsyncWrite + Unpin
            {
                let mut buf = $arr;
                let len = $enc(item, &mut buf);
                dest.write_all(len).await?;
                Ok(())
            }

            /// Read an unsigned varint value from the given [`AsyncRead`] impl.
            pub async fn read<R>(src: &mut R) -> Result<$typ, Error>
            where
                R: AsyncRead + Unpin
            {
                let mut b = $arr;
                let mut i = 0;
                loop {
                    let n = src.read(&mut b[i .. i + 1]).await?;
                    if n == 0 {
                        return Err(Error::Io(io::ErrorKind::UnexpectedEof.into()))
                    }
                    if decode::is_last(b[i]) {
                        break
                    }
                    i += 1
                }
                Ok($dec(&b[..= i])?.0)
            }
        }
    }
}

encode_decode_impls!(u8, u8, encode::u8, decode::u8, encode::u8_buffer());
encode_decode_impls!(u16, u16, encode::u16, decode::u16, encode::u16_buffer());
encode_decode_impls!(u32, u32, encode::u32, decode::u32, encode::u32_buffer());
encode_decode_impls!(u64, u64, encode::u64, decode::u64, encode::u64_buffer());
encode_decode_impls!(u128, u128, encode::u128, decode::u128, encode::u128_buffer());
encode_decode_impls!(usize, usize, encode::usize, decode::usize, encode::usize_buffer());

/// Error that may occur during decoding of unsigned varint values.
#[derive(Debug)]
pub enum Error {
    /// The underlying I/O resource errored.
    Io(io::Error),
    /// Decoding the length failed.
    Decode(decode::Error)
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Io(e) => write!(f, "i/o error: {}", e),
            Error::Decode(e) => write!(f, "decode error: {}", e)
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Io(e) => Some(e),
            Error::Decode(e) => Some(e)
        }
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::Io(e)
    }
}

impl From<decode::Error> for Error {
    fn from(e: decode::Error) -> Self {
        Error::Decode(e)
    }
}

// Tests //////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use futures::{executor::block_on, prelude::*};
    use quickcheck::{QuickCheck, StdThreadGen};

    #[test]
    fn encode_decode_identity() {
        fn prop(xs: Vec<u8>) -> bool {
            let mut io = std::io::Cursor::new(vec![0; xs.len() + crate::encode::USIZE_BUF_LEN]);

            // encode
            block_on(super::usize::write(xs.len(), &mut io)).expect("encode len");
            block_on(io.write_all(&xs)).expect("encode bytes");

            io.set_position(0);

            // decode
            let n = block_on(super::usize::read(&mut io)).expect("decode len");
            assert_eq!(n, xs.len());
            let mut ys = vec![0; n];
            block_on(io.read_exact(&mut ys)).expect("decode bytes");

            xs == ys
        }

        QuickCheck::with_gen(StdThreadGen::new(512 * 1024)).quickcheck(prop as fn(Vec<u8>) -> bool)
    }
}

