use nom::{
    bytes::complete::tag,
    combinator::{flat_map, map},
    multi::count,
    number::complete::{le_u32, le_u8},
    sequence::{pair, preceded},
};
#[cfg(feature = "serde")]
use serde::Serialize;

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct JmxTexture {
    pub header_len: u32,
    pub data: Vec<u8>,
}

impl JmxTexture {
    pub fn parse(i: &[u8]) -> Result<Self, nom::Err<(&[u8], nom::error::ErrorKind)>> {
        map(
            preceded(
                tag(b"JMXVDDJ 1000"),
                flat_map(le_u32, |c| pair(le_u32, count(le_u8, c as usize))),
            ),
            |(header_len, data)| JmxTexture { header_len, data },
        )(i)
        .map(|r| r.1)
    }
}
