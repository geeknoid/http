use std::hint::black_box;

use gungraun::{library_benchmark, library_benchmark_group, main};
use http::header::*;

#[library_benchmark]
fn new_insert_get_host() {
    let mut h = HeaderMap::default();
    h.insert(HOST, "hyper.rs");
    black_box(h.get(&HOST));
}

#[library_benchmark]
fn insert_4_std_get_30() {
    let mut h = HeaderMap::default();

    for i in 0..4 {
        h.insert(STD[i].clone(), "foo");
    }

    for i in 0..30 {
        black_box(h.get(&STD[i % 4]));
    }
}

#[library_benchmark]
fn insert_6_std_get_6() {
    let mut h = HeaderMap::default();

    for i in 0..6 {
        h.insert(STD[i].clone(), "foo");
    }

    for i in 0..6 {
        black_box(h.get(&STD[i % 4]));
    }
}

#[library_benchmark]
#[bench::n8(8)]
#[bench::n10(10)]
#[bench::n20(20)]
fn set_n_get_1_std(n: usize) {
    let mut h = HeaderMap::default();

    for hdr in &STD[0..n] {
        h.insert(hdr.clone(), "foo");
    }

    black_box(h.get(&STD[0]));
}

#[library_benchmark]
#[bench::short(args = (10,), setup = custom_hdr)]
#[bench::med(args = (10,), setup = med_custom_hdr)]
#[bench::long(args = (10,), setup = long_custom_hdr)]
#[bench::very_long(args = (10,), setup = very_long_custom_hdr)]
fn set_10_get_1_custom(hdrs: Vec<HeaderName>) {
    let mut h = HeaderMap::default();

    for hdr in &hdrs {
        h.insert(hdr.clone(), "foo");
    }

    black_box(h.get(&hdrs[0]));
}

#[library_benchmark]
#[bench::short(args = (20,), setup = custom_hdr)]
#[bench::med(args = (20,), setup = med_custom_hdr)]
#[bench::long(args = (20,), setup = long_custom_hdr)]
#[bench::very_long(args = (20,), setup = very_long_custom_hdr)]
fn set_20_get_1_custom(hdrs: Vec<HeaderName>) {
    let mut h = HeaderMap::default();

    for hdr in &hdrs {
        h.insert(hdr.clone(), "foo");
    }

    black_box(h.get(&hdrs[0]));
}

#[library_benchmark]
fn insert_all_std_headers() {
    let mut h = HeaderMap::default();

    for hdr in STD {
        black_box(h.insert(hdr.clone(), "foo"));
    }
}

#[library_benchmark]
#[bench::custom_std_79(args = (79,), setup = custom_std)]
fn insert_custom_std_headers(hdrs: Vec<HeaderName>) {
    let mut h = HeaderMap::default();

    for hdr in &hdrs {
        h.insert(hdr.clone(), "foo");
    }

    black_box(h);
}

#[library_benchmark]
#[bench::n100(args = (100,), setup = custom_hdr)]
#[bench::n500(args = (500,), setup = custom_hdr)]
fn insert_custom_headers(hdrs: Vec<HeaderName>) {
    let mut h = HeaderMap::default();

    for hdr in &hdrs {
        black_box(h.insert(hdr.clone(), "foo"));
    }
}

#[library_benchmark]
#[bench::c15(args = ("abcd-abcd-abcde",), setup = parse_name)]
#[bench::c25(args = ("abcd-abcd-abcd-abcd-abcde",), setup = parse_name)]
#[bench::c50(args = ("abcd-abcd-abcd-abcd-abcd-abcd-abcd-abcd-abcd-abcde",), setup = parse_name)]
#[bench::c100(
    args = ("abcd-abcd-abcd-abcd-abcd-abcd-abcd-abcd-abcd-abcdeabcd-abcd-abcd-abcd-abcd-abcd-abcd-abcd-abcd-abcde",),
    setup = parse_name
)]
fn insert_one_char_header(hdr: HeaderName) {
    let mut h = HeaderMap::default();
    h.insert(hdr.clone(), "hello");
    black_box(h);
}

#[library_benchmark]
#[bench::run(args = (), setup = std_map_10_30)]
fn get_10_of_20_std(h: HeaderMap<String>) {
    for hdr in &STD[10..20] {
        black_box(h.get(hdr));
    }
}

#[library_benchmark]
#[bench::run(args = (), setup = std_map_all)]
fn get_100_std(h: HeaderMap<String>) {
    for i in 0..100 {
        black_box(h.get(&STD[i % STD.len()]));
    }
}

#[library_benchmark]
#[bench::run(args = (), setup = custom_short_map)]
fn get_10_custom_short((hdrs, h): (Vec<HeaderName>, HeaderMap<String>)) {
    for hdr in &hdrs[..10] {
        black_box(h.get(hdr));
    }
}

#[library_benchmark]
#[bench::run(args = (), setup = parse_hn_hdrs_8)]
fn hn_hdrs_set_8_get_many(hdrs: Vec<(HeaderName, &'static str)>) {
    let mut h = HeaderMap::default();

    for &(ref name, val) in hdrs.iter() {
        h.insert(name.clone(), val);
    }

    for _ in 0..15 {
        black_box(h.get(&CONTENT_LENGTH));
        black_box(h.get(&VARY));
    }
}

#[library_benchmark]
#[bench::run(args = (), setup = parse_hn_hdrs_8_miss)]
fn hn_hdrs_set_8_get_miss((hdrs, miss): (Vec<(HeaderName, &'static str)>, HeaderName)) {
    let mut h = HeaderMap::default();

    for &(ref name, val) in hdrs.iter() {
        h.insert(name.clone(), val);
    }

    black_box(h.get(&CONTENT_LENGTH));
    black_box(h.get(&miss));
}

#[library_benchmark]
#[bench::run(args = (), setup = parse_hn_hdrs_11_miss)]
fn hn_hdrs_set_11_get_with_miss((hdrs, miss): (Vec<(HeaderName, &'static str)>, HeaderName)) {
    let mut h = HeaderMap::default();

    for &(ref name, val) in hdrs.iter() {
        h.insert(name.clone(), val);
    }

    for _ in 0..10 {
        black_box(h.get(&CONTENT_LENGTH));
        black_box(h.get(&VARY));
        black_box(h.get(&miss));
    }
}

fn parse_name(s: &str) -> HeaderName {
    s.parse().unwrap()
}

fn custom_hdr(n: usize) -> Vec<HeaderName> {
    (0..n)
        .map(|i| {
            let s = format!("x-custom-{}", i);
            s.parse().unwrap()
        })
        .collect()
}

fn med_custom_hdr(n: usize) -> Vec<HeaderName> {
    (0..n)
        .map(|i| {
            let s = format!("content-length-{}", i);
            s.parse().unwrap()
        })
        .collect()
}

fn long_custom_hdr(n: usize) -> Vec<HeaderName> {
    (0..n)
        .map(|i| {
            let s = format!("access-control-allow-headers-{}", i);
            s.parse().unwrap()
        })
        .collect()
}

fn very_long_custom_hdr(n: usize) -> Vec<HeaderName> {
    (0..n)
        .map(|i| {
            let s = format!("access-control-allow-access-control-allow-headers-{}", i);
            s.parse().unwrap()
        })
        .collect()
}

fn custom_std(n: usize) -> Vec<HeaderName> {
    (0..n)
        .map(|i| {
            let s = format!("{}-{}", STD[i % STD.len()].as_str(), i);
            s.parse().unwrap()
        })
        .collect()
}

fn std_map_10_30() -> HeaderMap<String> {
    let mut h = HeaderMap::default();
    for hdr in STD[10..30].iter() {
        h.insert(hdr.clone(), hdr.as_str().to_string());
    }
    h
}

fn std_map_all() -> HeaderMap<String> {
    let mut h = HeaderMap::default();
    for hdr in STD.iter() {
        h.insert(hdr.clone(), hdr.as_str().to_string());
    }
    h
}

fn custom_short_map() -> (Vec<HeaderName>, HeaderMap<String>) {
    let hdrs = custom_hdr(20);
    let mut h = HeaderMap::default();
    for hdr in &hdrs {
        h.insert(hdr.clone(), hdr.as_str().to_string());
    }
    (hdrs, h)
}

fn parse_hn_hdrs_8() -> Vec<(HeaderName, &'static str)> {
    HN_HDRS[..8]
        .iter()
        .map(|&(name, val)| (name.parse().unwrap(), val))
        .collect()
}

fn parse_hn_hdrs_8_miss() -> (Vec<(HeaderName, &'static str)>, HeaderName) {
    (parse_hn_hdrs_8(), "x-wat".parse().unwrap())
}

fn parse_hn_hdrs_11_miss() -> (Vec<(HeaderName, &'static str)>, HeaderName) {
    let hdrs = HN_HDRS
        .iter()
        .map(|&(name, val)| (name.parse().unwrap(), val))
        .collect();
    (hdrs, "x-wat".parse().unwrap())
}

const HN_HDRS: [(&'static str, &'static str); 11] = [
    ("Date", "Fri, 27 Jan 2017 23:00:00 GMT"),
    ("Content-Type", "text/html; charset=utf-8"),
    ("Transfer-Encoding", "chunked"),
    ("Connection", "keep-alive"),
    ("Set-Cookie", "__cfduid=dbdfbbe3822b61cb8750ba37d894022151485558000; expires=Sat, 27-Jan-18 23:00:00 GMT; path=/; domain=.ycombinator.com; HttpOnly"),
    ("Vary", "Accept-Encoding"),
    ("Cache-Control", "private"),
    ("X-Frame-Options", "DENY"),
    ("Strict-Transport-Security", "max-age=31556900; includeSubDomains"),
    ("Server", "cloudflare-nginx"),
    ("CF-RAY", "327fd1809f3c1baf-SEA"),
];

const STD: &'static [HeaderName] = &[
    ACCEPT,
    ACCEPT_CHARSET,
    ACCEPT_ENCODING,
    ACCEPT_LANGUAGE,
    ACCEPT_RANGES,
    ACCESS_CONTROL_ALLOW_CREDENTIALS,
    ACCESS_CONTROL_ALLOW_HEADERS,
    ACCESS_CONTROL_ALLOW_METHODS,
    ACCESS_CONTROL_ALLOW_ORIGIN,
    ACCESS_CONTROL_EXPOSE_HEADERS,
    ACCESS_CONTROL_MAX_AGE,
    ACCESS_CONTROL_REQUEST_HEADERS,
    ACCESS_CONTROL_REQUEST_METHOD,
    AGE,
    ALLOW,
    ALT_SVC,
    AUTHORIZATION,
    CACHE_CONTROL,
    CACHE_STATUS,
    CDN_CACHE_CONTROL,
    CONNECTION,
    CONTENT_DISPOSITION,
    CONTENT_ENCODING,
    CONTENT_LANGUAGE,
    CONTENT_LENGTH,
    CONTENT_LOCATION,
    CONTENT_RANGE,
    CONTENT_SECURITY_POLICY,
    CONTENT_SECURITY_POLICY_REPORT_ONLY,
    CONTENT_TYPE,
    COOKIE,
    DNT,
    DATE,
    ETAG,
    EXPECT,
    EXPIRES,
    FORWARDED,
    FROM,
    HOST,
    IF_MATCH,
    IF_MODIFIED_SINCE,
    IF_NONE_MATCH,
    IF_RANGE,
    IF_UNMODIFIED_SINCE,
    LAST_MODIFIED,
    LINK,
    LOCATION,
    MAX_FORWARDS,
    ORIGIN,
    PRAGMA,
    PROXY_AUTHENTICATE,
    PROXY_AUTHORIZATION,
    PUBLIC_KEY_PINS,
    PUBLIC_KEY_PINS_REPORT_ONLY,
    RANGE,
    REFERER,
    REFERRER_POLICY,
    REFRESH,
    RETRY_AFTER,
    SERVER,
    SET_COOKIE,
    STRICT_TRANSPORT_SECURITY,
    TE,
    TRAILER,
    TRANSFER_ENCODING,
    USER_AGENT,
    UPGRADE,
    UPGRADE_INSECURE_REQUESTS,
    VARY,
    VIA,
    WARNING,
    WWW_AUTHENTICATE,
    X_CONTENT_TYPE_OPTIONS,
    X_DNS_PREFETCH_CONTROL,
    X_FRAME_OPTIONS,
    X_XSS_PROTECTION,
];

library_benchmark_group!(
    name = header_map,
    benchmarks = [
        new_insert_get_host,
        insert_4_std_get_30,
        insert_6_std_get_6,
        set_n_get_1_std,
        set_10_get_1_custom,
        set_20_get_1_custom,
        insert_all_std_headers,
        insert_custom_std_headers,
        insert_custom_headers,
        insert_one_char_header,
        get_10_of_20_std,
        get_100_std,
        get_10_custom_short,
        hn_hdrs_set_8_get_many,
        hn_hdrs_set_8_get_miss,
        hn_hdrs_set_11_get_with_miss
    ]
);

main!(library_benchmark_groups = header_map);
