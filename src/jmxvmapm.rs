use nom::bytes::complete::{tag, take};
use nom::combinator::map;
use nom::error::ParseError;
use nom::multi::count;
use nom::number::complete::{le_f32, le_u16, le_u32, le_u8};
use nom::sequence::preceded;
use nom::IResult;
use struple::Struple;

#[cfg(feature = "serde")]
use serde::Serialize;

use crate::parser_ext::combinator::struple;
use crate::SrFile;

#[derive(Debug, Struple)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct MapMeshCell {
    pub height: u32,
    pub texture: u16,
    pub brightness: u8,
}

impl MapMeshCell {
    fn parse<'a, E: ParseError<&'a [u8]>>(i: &'a [u8]) -> IResult<&'a [u8], Self, E> {
        struple((le_u32, le_u16, le_u8))(i)
    }
}

#[derive(Debug, Struple)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct MapBlock {
    pub name: String,
    pub cells: Vec<MapMeshCell>,
    pub density: u8,
    pub unk0: u8,
    pub sea_level: f32,
    pub extra_data: Vec<u8>,
    pub height_min: f32,
    pub height_max: f32,
    pub unk0_buffer: Vec<u8>,
}

impl MapBlock {
    fn parse<'a, E: ParseError<&'a [u8]>>(i: &'a [u8]) -> IResult<&'a [u8], Self, E> {
        struple((
            string_6,
            count(MapMeshCell::parse, 16 * 16 + 1),
            le_u8,
            le_u8,
            le_f32,
            count(le_u8, 256),
            le_f32,
            le_f32,
            count(le_u8, 20),
        ))(i)
    }
}

#[inline]
pub fn string_6<'a, E: ParseError<&'a [u8]>>(i: &'a [u8]) -> IResult<&'a [u8], String, E> {
    map(take(6usize), |s| {
        encoding_rs::EUC_KR
            .decode_without_bom_handling(s)
            .0
            .into_owned()
    })(i)
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct JmxMapMesh {
    pub blocks: Vec<MapBlock>,
}

impl SrFile for JmxMapMesh {
    type Input = [u8];
    type Output = Self;
    fn nom_parse<'i, E: ParseError<&'i Self::Input>>(
        i: &'i Self::Input,
    ) -> IResult<&'i Self::Input, Self::Output, E> {
        map(
            preceded(tag(b"JMXVMAPM1000"), count(MapBlock::parse, 6 * 6)),
            |blocks| JmxMapMesh { blocks },
        )(i)
    }
}
