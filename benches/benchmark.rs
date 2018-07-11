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

#![feature(test)]

extern crate test;
extern crate unsigned_varint;

#[cfg(feature = "codec")]
extern crate bytes;
#[cfg(feature = "codec")]
extern crate tokio_codec;

use std::u64;
use test::Bencher;
use unsigned_varint::{decode, encode};

#[bench]
fn bench_decode(b: &mut Bencher) {
    let mut buf = [0; 10];
    let bytes = encode::u64(u64::MAX, &mut buf);
    b.iter(|| {
        assert_eq!(u64::MAX, decode::u64(bytes).unwrap().0)
    });
}

#[bench]
fn bench_encode(b: &mut Bencher) {
    let mut buf = [0; 10];
    let encoded = [0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 1];
    b.iter(|| {
        assert_eq!(&encoded, encode::u64(u64::MAX, &mut buf));
    });
}

#[cfg(feature = "codec")]
#[bench]
fn bench_codec(b: &mut Bencher) {
    use bytes::{Bytes, BytesMut};
    use tokio_codec::{Decoder, Encoder};
    use unsigned_varint::codec::UviBytes;

    let data = Bytes::from(vec![1; 8192]);
    let mut bytes = BytesMut::with_capacity(9000);
    let mut uvi_bytes = UviBytes::default();

    b.iter(move || {
        uvi_bytes.encode(data.clone(), &mut bytes).unwrap();
        assert_eq!(data, uvi_bytes.decode(&mut bytes.take()).unwrap().unwrap())
    });
}

