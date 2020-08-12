use nom::bytes::complete::tag;
use nom::combinator::flat_map;
use nom::error::ParseError;
use nom::multi::count;
use nom::number::complete::{le_u32, le_u8};
use nom::sequence::{pair, preceded};
use struple::Struple;

#[cfg(feature = "serde")]
use serde::Serialize;

use crate::parser_ext::combinator::struple_map;
use crate::SrFile;

#[derive(Debug, Struple)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct JmxTexture {
    pub header_len: u32,
    pub data: Vec<u8>,
}

impl SrFile for JmxTexture {
    type Input = [u8];
    type Output = Self;
    fn nom_parse<'i, E: ParseError<&'i Self::Input>>(
        i: &'i Self::Input,
    ) -> nom::IResult<&'i Self::Input, Self::Output, E> {
        struple_map(preceded(
            tag(b"JMXVDDJ 1000"),
            flat_map(le_u32, |texture_size| {
                pair(le_u32, count(le_u8, texture_size as usize - 8))
            }),
        ))(i)
    }
}
