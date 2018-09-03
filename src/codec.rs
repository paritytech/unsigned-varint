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

use bytes::{Buf, BufMut, Bytes, BytesMut, IntoBuf};
use encode;
use decode::{self, Error};
use std::{io, marker::PhantomData, usize};
use tokio_codec::{Encoder, Decoder};


/// tokio-codec based encoder + decoder of unsigned-varint values
#[derive(Default)]
pub struct Uvi<T>(PhantomData<*const T>);

// Implement tokio-codec `Encoder` + `Decoder` traits for unsigned integers.
macro_rules! encoder_decoder_impls {
    ($typ:ty, $enc:expr, $dec:expr, $arr:expr) => {
        impl Encoder for Uvi<$typ> {
            type Item = $typ;
            type Error = io::Error;

            fn encode(&mut self, item: Self::Item, dst: &mut BytesMut) -> Result<(), Self::Error> {
                let mut buf = $arr;
                dst.extend_from_slice($enc(item, &mut buf));
                Ok(())
            }
        }

        impl Decoder for Uvi<$typ> {
            type Item = $typ;
            type Error = io::Error;

            fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
                let (number, consumed) =
                    match $dec(src.as_ref()) {
                        Ok((n, rem)) => (n, src.len() - rem.len()),
                        Err(Error::Insufficient) => return Ok(None),
                        Err(e) => return Err(io::Error::new(io::ErrorKind::Other, e))
                    };
                src.split_to(consumed);
                Ok(Some(number))
            }
        }
    }
}

encoder_decoder_impls!(u8, encode::u8, decode::u8, encode::u8_buffer());
encoder_decoder_impls!(u16, encode::u16, decode::u16, encode::u16_buffer());
encoder_decoder_impls!(u32, encode::u32, decode::u32, encode::u32_buffer());
encoder_decoder_impls!(u64, encode::u64, decode::u64, encode::u64_buffer());
encoder_decoder_impls!(u128, encode::u128, decode::u128, encode::u128_buffer());
encoder_decoder_impls!(usize, encode::usize, decode::usize, encode::usize_buffer());


/// tokio-codec based encoder + decoder of unsigned-varint, length-prefixed bytes
pub struct UviBytes<T = Bytes> {
    len: Option<usize>, // number of bytes (for decoding only)
    max: usize, // max. number of bytes (for decoding only)
    _ty: PhantomData<T>
}

impl<T> Default for UviBytes<T> {
    fn default() -> Self {
        Self { len: None, max: usize::MAX, _ty: PhantomData }
    }
}

impl<T> UviBytes<T> {
    /// Limit the maximum allowed length of bytes.
    pub fn set_max_len(&mut self, val: usize) {
        self.max = val
    }
}

impl<T: IntoBuf> Encoder for UviBytes<T> {
    type Item = T;
    type Error = io::Error;

    fn encode(&mut self, item: Self::Item, dst: &mut BytesMut) -> Result<(), Self::Error> {
        let bytes = item.into_buf();
        if bytes.len() > self.max {
            return Err(io::Error::new(io::ErrorKind::PermissionDenied, "len > max when encoding"));
        }
        Uvi::<usize>::default().encode(bytes.remaining(), dst)?;
        dst.reserve(bytes.remaining());
        dst.put(bytes);
        Ok(())
    }
}

impl<T> Decoder for UviBytes<T> {
    type Item = BytesMut;
    type Error = io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        loop {
            match self.len.take() {
                None => {
                    self.len = Uvi::<usize>::default().decode(src)?;
                    if self.len.is_none() {
                        return Ok(None)
                    }
                    continue
                }
                Some(n) if n > self.max => {
                    return Err(io::Error::new(io::ErrorKind::PermissionDenied, "len > max"))
                }
                Some(n) => {
                    if src.len() < n {
                        let add = n - src.len();
                        src.reserve(add);
                        self.len = Some(n);
                        return Ok(None)
                    } else {
                        return Ok(Some(src.split_to(n)))
                    }
                }
            }
        }
    }
}


