use nom::bytes::complete::tag;
use nom::combinator::{flat_map, map};
use nom::multi::count;
use nom::number::complete::{le_u32, le_u8};
use nom::sequence::{pair, preceded};

#[cfg(feature = "serde")]
use serde_derive::Serialize;

use crate::ttr_closure;

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct JmxTexture {
    pub header_len: u32,
    pub data: Vec<u8>,
}

impl JmxTexture {
    pub fn parse<'i>(i: &'i [u8]) -> nom::IResult<&'i [u8], Self> {
        map(
            preceded(
                tag(b"JMXVDDJ 1000"),
                flat_map(le_u32, |texture_size| {
                    pair(le_u32, count(le_u8, texture_size as usize - 8))
                }),
            ),
            ttr_closure! {
                JmxTexture {
                    header_len,
                    data
                }
            },
        )(i)
    }
}
