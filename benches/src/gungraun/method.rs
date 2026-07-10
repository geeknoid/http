use std::hint::black_box;

use gungraun::{library_benchmark, library_benchmark_group, main};
use http::method::Method;

fn make_all_methods() -> Vec<Vec<u8>> {
    vec![
        b"OPTIONS".to_vec(),
        b"GET".to_vec(),
        b"POST".to_vec(),
        b"PUT".to_vec(),
        b"DELETE".to_vec(),
        b"HEAD".to_vec(),
        b"TRACE".to_vec(),
        b"CONNECT".to_vec(),
        b"PATCH".to_vec(),
        b"CUSTOM_SHORT".to_vec(),
        b"CUSTOM_LONG_METHOD".to_vec(),
    ]
}

#[library_benchmark]
fn method_easy() -> Method {
    black_box(Method::from_bytes(black_box(&b"GET"[..])).unwrap())
}

#[library_benchmark]
#[bench::various(args = (), setup = make_all_methods)]
fn method_various(all_methods: Vec<Vec<u8>>) {
    for name in &all_methods {
        black_box(Method::from_bytes(black_box(name.as_slice())).unwrap());
    }
}

library_benchmark_group!(
    name = method,
    benchmarks = [method_easy, method_various]
);

main!(library_benchmark_groups = method);
