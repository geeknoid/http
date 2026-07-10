use std::hint::black_box;

use gungraun::{library_benchmark, library_benchmark_group, main};
use http::header::HeaderName;

// This is a list of some of the standard headers ordered by increasing size.
// It has exactly one standard header per size (some sizes don't have a standard
// header).
#[library_benchmark]
#[bench::size_2("te")]
#[bench::size_3("age")]
#[bench::size_4("date")]
#[bench::size_5("allow")]
#[bench::size_6("accept")]
#[bench::size_7("alt-svc")]
#[bench::size_8("if-match")]
#[bench::size_9("forwarded")]
#[bench::size_10("connection")]
#[bench::size_11("retry-after")]
#[bench::size_12("content-type")]
#[bench::size_13("accept-ranges")]
#[bench::size_14("accept-charset")]
#[bench::size_15("accept-encoding")]
#[bench::size_16("content-encoding")]
#[bench::size_17("if-modified-since")]
#[bench::size_18("proxy-authenticate")]
#[bench::size_19("content-disposition")]
#[bench::size_20("sec-websocket-accept")]
#[bench::size_21("sec-websocket-version")]
#[bench::size_22("access-control-max-age")]
#[bench::size_23("content-security-policy")]
#[bench::size_24("sec-websocket-extensions")]
#[bench::size_25("strict-transport-security")]
#[bench::size_27("access-control-allow-origin")]
#[bench::size_28("access-control-allow-headers")]
#[bench::size_29("access-control-expose-headers")]
#[bench::size_30("access-control-request-headers")]
#[bench::size_33("access-control-allow-credentials")]
#[bench::size_36("content-security-policy-report-only")]
fn header_name_by_size(name: &'static str) -> HeaderName {
    black_box(HeaderName::from_static(black_box(name)))
}

library_benchmark_group!(
    name = std_hdr,
    benchmarks = header_name_by_size
);

main!(library_benchmark_groups = std_hdr);
