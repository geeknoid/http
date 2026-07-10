use std::hint::black_box;

use gungraun::{library_benchmark, library_benchmark_group, main};
use http::Uri;

#[library_benchmark]
#[bench::slash("/")]
#[bench::relative_medium("/wp-content/uploads/2010/03/hello-kitty-darth-vader-pink.jpg")]
#[bench::relative_query(
    "/wp-content/uploads/2010/03/hello-kitty-darth-vader-pink.jpg?foo={bar}|baz%13%11quux"
)]
fn uri_parse(s: &str) -> Uri {
    black_box(black_box(s).parse::<Uri>().unwrap())
}

fn parse(s: &str) -> Uri {
    s.parse().unwrap()
}

#[library_benchmark]
#[bench::relative(args = ("/wp-content/uploads/2010/03/hello-kitty-darth-vader-pink.jpg",), setup = parse)]
#[bench::relative_query(args = ("/wp-content/uploads/2010/03/hello-kitty-darth-vader-pink.jpg?foo=bar&baz=quux",), setup = parse)]
#[bench::absolute(args = ("https://www.example.com/wp-content/uploads/hello.jpg?foo=bar",), setup = parse)]
fn uri_to_string(uri: Uri) -> String {
    black_box(black_box(&uri).to_string())
}

library_benchmark_group!(
    name = uri,
    benchmarks = [uri_parse, uri_to_string]
);

main!(library_benchmark_groups = uri);
