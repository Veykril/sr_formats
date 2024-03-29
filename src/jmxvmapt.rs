use nom::bytes::complete::tag;
use nom::combinator::{flat_map, map};
use nom::number::complete::{le_u32, le_u8};
use nom::sequence::{pair, preceded};

#[cfg(feature = "serde")]
use serde_derive::Serialize;

use crate::parser_ext::multi::count;

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct JmxMapTexture {
    pub shadow_map_tiles: Box<[u8]>, //9216
    pub header_len: u32,
    pub data: Box<[u8]>,
}

impl JmxMapTexture {
    pub fn parse<'i>(i: &'i [u8]) -> nom::IResult<&'i [u8], Self> {
        map(
            preceded(
                tag("JMXVMAPT 1001"),
                pair(
                    count(le_u8, 9216),
                    flat_map(le_u32, |c| pair(le_u32, count(le_u8, c as usize))),
                ),
            ),
            |(shadow_map_tiles, (header_len, data))| JmxMapTexture {
                shadow_map_tiles,
                header_len,
                data,
            },
        )(i)
    }
}
