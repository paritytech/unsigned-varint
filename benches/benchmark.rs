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

#![feature(async_await)]

use criterion::{criterion_group, criterion_main, Criterion};
use std::u64;
use unsigned_varint::{decode, encode};

fn bench_decode(c: &mut Criterion) {
    let mut buf = [0; 10];
    let len = encode::u64(u64::MAX, &mut buf).len();
    c.bench_function("decode", move |b| b.iter(|| {
        assert_eq!(u64::MAX, decode::u64(&buf[.. len]).unwrap().0)
    }));
}

fn bench_encode(c: &mut Criterion) {
    let mut buf = [0; 10];
    let encoded = [0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 1];
    c.bench_function("encode", move |b| b.iter(|| {
        assert_eq!(&encoded, encode::u64(u64::MAX, &mut buf));
    }));
}

#[cfg(feature = "codec")]
fn bench_codec(c: &mut Criterion) {
    use futures::{executor::block_on, prelude::*};
    use unsigned_varint::codec;

    let data = vec![1; 8192];
    let mut io = std::io::Cursor::new(vec![0; 9000]);

    c.bench_function("codec", move |b| b.iter(|| {
        io.set_position(0);
        let result: Vec<u8> = block_on(async {
            codec::usize::write(data.len(), &mut io).await.expect("encode len");
            io.write_all(&data).await.expect("encode data");
            io.set_position(0);
            let n = codec::usize::read(&mut io).await.expect("decode len");
            let mut xs = vec![0; n];
            io.read_exact(&mut xs).await.expect("decode data");
            xs
        });
        assert_eq!(data, result)
    }));
}

#[cfg(feature = "codec")]
criterion_group!(benches, bench_encode, bench_decode, bench_codec);

#[cfg(not(feature = "codec"))]
criterion_group!(benches, bench_encode, bench_decode);

criterion_main!(benches);

