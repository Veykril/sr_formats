use nom::{
    bytes::complete::tag,
    combinator::{flat_map, map},
    error::ParseError,
    multi::count,
    number::complete::{le_u32, le_u8},
    sequence::{pair, preceded},
};
#[cfg(feature = "serde")]
use serde::Serialize;

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct JmxMapTexture {
    pub shadow_map_tiles: Vec<u8>, //9216
    pub header_len: u32,
    pub data: Vec<u8>,
}

impl JmxMapTexture {
    pub fn parse<'a, E: ParseError<&'a [u8]>>(i: &'a [u8]) -> Result<Self, nom::Err<E>> {
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
        .map(|r| r.1)
    }
}
