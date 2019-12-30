use nom::bytes::complete::tag;
use nom::combinator::{flat_map, map};
use nom::error::ParseError;
use nom::multi::count;
use nom::number::complete::{le_u32, le_u8};
use nom::sequence::{pair, preceded};

#[cfg(feature = "serde")]
use serde::Serialize;

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct JmxTexture {
    pub header_len: u32,
    pub data: Vec<u8>,
}

impl JmxTexture {
    pub fn parse<'a, E: ParseError<&'a [u8]>>(i: &'a [u8]) -> Result<Self, nom::Err<E>> {
        map(
            preceded(
                tag(b"JMXVDDJ 1000"),
                flat_map(le_u32, |texture_size| {
                    pair(le_u32, count(le_u8, texture_size as usize - 8))
                }),
            ),
            |(header_len, data)| JmxTexture { header_len, data },
        )(i)
        .map(|r| r.1)
    }
}
