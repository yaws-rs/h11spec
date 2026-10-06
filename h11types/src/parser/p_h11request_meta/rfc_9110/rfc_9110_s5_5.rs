//! RFC 9110 s. 5.5 Field value tests (parser)

use super::super::*;
use rstest::{rstest, Context};

//use crate::{MetaReceiver, HeaderReceiver};

// Trim preceding / trailing WS from header values
#[rstest]
#[case("Content-Length:  42\r\n")]
#[case("Content-Length: 42 \r\n")]
#[case("Host: foo.bar \r\n")]
#[case("Host:  foo.bar\r\n")]
#[case("Host:  foo.bar \r\n")]
#[case("Content-Length:  42 \r\nHost:  foo.bar  \r\n")]
#[case("Set-Cookie:   \"foo bar\"   \r\n")]
fn tc1_trim_ws(#[context] ctx: Context, #[case] tc_input: &'static str) {
    do_header_test(ctx, tc_input);
}

// Reject singleton headers with CR, LF or NUL
#[rstest]
#[case("Host:  foo\r.bar\r\n")]
#[case("Host:  foo\n.bar\r\n")]
#[case("Host:  foo\0.bar\r\n")]
#[case("Host\r:  foo.bar\r\n")]
#[case("\rHost:  foo.bar\r\n")]
#[case("Ho\rst:  foo.bar\r\n")]
#[case("Host\n:  foo.bar\r\n")]
#[case("Ho\nst:  foo.bar\r\n")]
#[case("Host\0:  foo.bar\r\n")]
#[case("Host\0:  foo\0bar\r\n")]
fn tc2_reject_singleton_cr_lf_nul(#[context] ctx: Context, #[case] tc_input: &'static str) {
    do_header_test(ctx, tc_input);
}
