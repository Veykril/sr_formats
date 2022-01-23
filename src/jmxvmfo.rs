use nom::bytes::complete::{tag, take};
use nom::combinator::map;
use nom::number::complete::le_u8;
use nom::sequence::preceded;

#[cfg(feature = "serde")]
use serde_derive::Serialize;

use crate::parser_ext::multi::count;

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct JmxMapInfo {
    //BitArray: 256 * 256 = 65536 bits / 8 = 8192 bytes
    pub region_data: Box<[u8]>,
}

impl JmxMapInfo {
    pub fn parse<'i>(i: &'i [u8]) -> nom::IResult<&'i [u8], Self> {
        map(
            preceded(
                tag("JMXVMFO 1000"),
                preceded(take(12usize), count(le_u8, 256 * 256 / 8)),
            ),
            |region_data| JmxMapInfo { region_data },
        )(i)
    }
}
