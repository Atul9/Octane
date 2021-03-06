extern crate octane;
use octane::request::{RequestLine, RequestMethod};

#[test]
fn success_standard() {
    // Parsing should work as expected.
    let req = RequestLine::parse("POST /abc/def HTTP/1.1").unwrap();
    assert_eq!(req.method, RequestMethod::Post);
    assert_eq!(req.path, "/abc/def");
    assert_eq!(req.version, "1.1");
}

#[test]
fn success_other_method() {
    // Non-documented methods should also work.
    let req = RequestLine::parse("PATCH /abc/def HTTP/1.1").unwrap();
    assert_eq!(req.method, RequestMethod::Other("PATCH"));
    assert_eq!(req.path, "/abc/def");
    assert_eq!(req.version, "1.1");
}

#[test]
#[should_panic]
#[cfg_attr(not(feature = "faithful"), ignore)]
fn fail_extra_1() {
    // Extra clauses should error.
    RequestLine::parse("POST /abc/def HTTP/1.1 x").unwrap();
}

#[test]
#[should_panic]
fn fail_extra_2() {
    // Extra clauses should error.
    RequestLine::parse("POST /a /b HTTP/1.1").unwrap();
}

#[test]
#[should_panic]
#[cfg_attr(not(feature = "faithful"), ignore)]
fn fail_malformed_version() {
    // Malformed versions should error.
    RequestLine::parse("POST /abc/def HTDP/1.1").unwrap();
}

#[test]
#[should_panic]
fn fail_missing_clause() {
    // Missing clauses should error.
    RequestLine::parse("POST /abc/def").unwrap();
}
