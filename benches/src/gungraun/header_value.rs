use std::hint::black_box;

use bytes::Bytes;
use gungraun::{library_benchmark, library_benchmark_group, main};
use http::HeaderValue;

static SHORT: &'static [u8] = b"localhost";
static LONG: &'static [u8] = b"Mozilla/5.0 (X11; CrOS x86_64 9592.71.0) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/60.0.3112.80 Safari/537.36";

fn make_bytes(bytes: &'static [u8]) -> Bytes {
    Bytes::from_static(bytes)
}

fn make_value(bytes: &'static [u8]) -> HeaderValue {
    HeaderValue::from_bytes(bytes).unwrap()
}

#[library_benchmark]
#[bench::short(args = (SHORT,), setup = make_bytes)]
#[bench::long(args = (LONG,), setup = make_bytes)]
fn from_shared(bytes: Bytes) -> HeaderValue {
    black_box(HeaderValue::from_maybe_shared(black_box(bytes)).unwrap())
}

#[library_benchmark]
#[bench::short(args = (SHORT,), setup = make_bytes)]
#[bench::long(args = (LONG,), setup = make_bytes)]
fn from_shared_unchecked(bytes: Bytes) -> HeaderValue {
    black_box(unsafe { HeaderValue::from_maybe_shared_unchecked(black_box(bytes)) })
}

#[library_benchmark]
#[bench::short(args = (SHORT,), setup = make_value)]
#[bench::long(args = (LONG,), setup = make_value)]
fn to_str(value: HeaderValue) {
    black_box(black_box(&value).to_str().unwrap());
}

library_benchmark_group!(
    name = header_value,
    benchmarks = [from_shared, from_shared_unchecked, to_str]
);

main!(library_benchmark_groups = header_value);
