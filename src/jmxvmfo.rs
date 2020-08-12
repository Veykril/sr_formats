use nom::bytes::complete::{tag, take};
use nom::combinator::map;
use nom::error::ParseError;
use nom::multi::count;
use nom::number::complete::le_u8;
use nom::sequence::preceded;

#[cfg(feature = "serde")]
use serde::Serialize;

use crate::SrFile;

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct JmxMapInfo {
    //BitArray: 256 * 256 = 65536 bits / 8 = 8192 bytes
    pub region_data: Box<[u8]>,
}

impl SrFile for JmxMapInfo {
    type Input = [u8];
    type Output = Self;

    fn nom_parse<'i, E: ParseError<&'i Self::Input>>(
        i: &'i Self::Input,
    ) -> nom::IResult<&'i Self::Input, Self::Output, E> {
        map(
            preceded(
                tag("JMXVMFO 1000"),
                preceded(take(12usize), count(le_u8, 256 * 256)),
            ),
            |region_data| JmxMapInfo {
                region_data: region_data.into_boxed_slice(),
            },
        )(i)
    }
}
