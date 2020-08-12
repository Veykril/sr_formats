use nom::bytes::complete::tag;
use nom::combinator::{flat_map, map};
use nom::error::ParseError;
use nom::multi::count;
use nom::number::complete::{le_u32, le_u8};
use nom::sequence::{pair, preceded};

#[cfg(feature = "serde")]
use serde::Serialize;

use crate::SrFile;

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct JmxMapTexture {
    pub shadow_map_tiles: Vec<u8>, //9216
    pub header_len: u32,
    pub data: Vec<u8>,
}

impl SrFile for JmxMapTexture {
    type Input = [u8];
    type Output = Self;

    fn nom_parse<'i, E: ParseError<&'i Self::Input>>(
        i: &'i Self::Input,
    ) -> nom::IResult<&'i Self::Input, Self::Output, E> {
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
