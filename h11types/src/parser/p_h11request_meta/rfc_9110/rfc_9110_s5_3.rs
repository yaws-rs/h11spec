//! RFC 9110 s. 5.3 Field order tests (parser)

use super::super::*;
use rstest::{rstest, Context};

//use crate::{MetaReceiver, HeaderReceiver};

// Guard against duplicate processing related headers
#[rstest]
#[case("Content-Length: 42\r\nContent-Length: 44\r\n")]
fn tc1_fieldorder_dup(#[context] ctx: Context, #[case] tc_input: &'static str) {
    do_header_test(ctx, tc_input);
}

// Ensure the field order remains the same for Set-Cookie
#[rstest]
#[case("Content-length: 200\r\nSet-Cookie: 3\r\nSet-Cookie: 2\r\nSet-Cookie: 1\r\n")]
#[case("Content-length: 20x\r\nSet-Cookie: 3\r\nSet-Cookie: 2\r\nSet-Cookie: 1\r\n")]
fn tc2_fieldorder_set_cookie_same(#[context] ctx: Context, #[case] tc_input: &'static str) {
    do_header_test(ctx, tc_input);
}
