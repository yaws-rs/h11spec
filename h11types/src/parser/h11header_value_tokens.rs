//! Header value initial parsing

use logos::Logos;

#[derive(Debug, Logos)]
#[allow(missing_docs)]
#[logos(utf8 = false)]
pub(crate) enum HeaderValueToken<'raw> {
    #[regex(r"\s*(\d+)\s*\r\n", callback = super::p_generated_util::header_value_usize, priority = 100)]
    Integer(usize),
    #[regex(r##"\s*"([^\r\x00]+)"\s*\r{1}\n{1}"##, |lex| lex.slice().strip_suffix(&[13, 10]).unwrap().trim_ascii_end().trim_ascii_start(), allow_greedy = true, priority = 2)]
    MaybeQuotedValue(&'raw [u8]),
    #[regex(r"\s*([^\r\x00\x34]+)\r{1}\n{1}", |lex| lex.slice().trim_ascii_end().trim_ascii_start(), allow_greedy = false, priority = 1)]
    MaybeValue(&'raw [u8]),
}
