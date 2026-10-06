//! RFC 9110 s. 5.4 Field limit tests (parser)

use super::super::*;
use rstest::{rstest, Context};

use crate::{MetaReceiver, HeaderReceiver};

/* TODO
// Reject oversize headers with  4xx (Client Error)
#[rstest]
#[case("..", HeaderLimits {})]
fn tc1_reject_oversize_headers(#[context] ctx: Context, #[case] tc_input: &'static str, limits: HeaderLimits) {
    do_header_test_with_limits(ctx, tc_input, limits);
}
*/
