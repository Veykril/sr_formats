use nom::bytes::complete::tag;
use nom::combinator::map;
use nom::number::complete::{le_f32, le_u16, le_u32, le_u8};
use nom::sequence::{preceded, tuple};
use nom::IResult;

#[cfg(feature = "serde")]
use serde::Serialize;

use crate::parser_ext::multi::count;
use crate::parser_ext::string::fixed_string;
use crate::ttr_closure;

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct MapMeshCell {
    pub height: u32,
    pub texture: u16,
    pub brightness: u8,
}

impl MapMeshCell {
    fn parse<'a>(i: &'a [u8]) -> IResult<&'a [u8], Self> {
        map(
            tuple((le_u32, le_u16, le_u8)),
            ttr_closure! {
                MapMeshCell {
                    height,
                    texture,
                    brightness
                }
            },
        )(i)
    }
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct MapBlock {
    pub name: Box<str>,
    pub cells: Box<[MapMeshCell]>,
    pub density: u8,
    pub unk0: u8,
    pub sea_level: f32,
    pub extra_data: Box<[u8]>,
    pub height_min: f32,
    pub height_max: f32,
    pub unk0_buffer: Box<[u8]>,
}

impl MapBlock {
    fn parse<'a>(i: &'a [u8]) -> IResult<&'a [u8], Self> {
        map(
            tuple((
                fixed_string::<6>,
                count(MapMeshCell::parse, 16 * 16 + 1),
                le_u8,
                le_u8,
                le_f32,
                count(le_u8, 256),
                le_f32,
                le_f32,
                count(le_u8, 20),
            )),
            ttr_closure! {
                MapBlock {
                    name,
                    cells,
                    density,
                    unk0,
                    sea_level,
                    extra_data,
                    height_min,
                    height_max,
                    unk0_buffer
                }
            },
        )(i)
    }
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct JmxMapMesh {
    pub blocks: Box<[MapBlock]>,
}

impl JmxMapMesh {
    pub fn parse<'i>(i: &'i [u8]) -> IResult<&'i [u8], Self> {
        map(
            preceded(tag(b"JMXVMAPM1000"), count(MapBlock::parse, 6 * 6)),
            |blocks| JmxMapMesh { blocks },
        )(i)
    }
}
