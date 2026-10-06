//! utility fns for genereated bits

use logos::Lexer;

use crate::parser::HeaderValueToken;

#[inline]
pub(crate) fn header_value_usize<'raw>(
    lex: &mut Lexer<'raw, HeaderValueToken<'raw>>,
) -> Option<usize> {
    let slice = lex.slice();
    let mut idx = 0;
    for i in 0..slice.len() {
        if slice[i] == 58 {
            idx = i + 1;
            break;
        }
    }
    let (_, val) = if idx < slice.len() {
        slice.split_at(idx)
    } else {
        return None;
    };

    let s = match core::str::from_utf8(val.trim_ascii()) {
        Ok(s) => s,
        Err(_) => return None,
    };

    match s.parse::<usize>() {
        Ok(num) => Some(num),
        Err(_) => None,
    }
}
