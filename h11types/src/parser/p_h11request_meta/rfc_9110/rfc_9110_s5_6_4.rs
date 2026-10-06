//! RFC 9110 s. 5.6.4 Quoted strings tests (parser)

use super::super::*;
use rstest::{rstest, Context};

//use crate::{MetaReceiver, HeaderReceiver};

// Quoted strings in singletons
#[rstest]
#[case("Host: \"foo.bar\"\r\n")]
fn tc1_quoted_strings(#[context] ctx: Context, #[case] tc_input: &'static str) {
    do_header_test(ctx, tc_input);
}


/* TODO: maciejhirsz/logos/issues/568
// Backslash quotes
#[rstest]
#[case("Set-Cookie: \"test\\\"bar\\\"foo\"\r\n")]
fn tc1_quoted_backslash(#[context] ctx: Context, #[case] tc_input: &'static str) {
    do_header_test(ctx, tc_input);
}
*/
