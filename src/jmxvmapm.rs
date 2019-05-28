
use nalgebra::Vector3;
use nom::{
    bytes::complete::{tag, take},
    combinator::map,
    multi::count,
    number::complete::{le_f32, le_u16, le_u32, le_u8},
    sequence::{preceded, tuple},
    IResult,
};


use crate::{parse_objects_u16, vector3_f32};

pub struct MapMeshCell {
    pub height: u32,
    pub texture: u16,
    pub brightness: u8,
}

impl MapMeshCell {
    pub fn parser<'a>() -> impl Fn(&'a [u8]) -> IResult<&'a [u8], Self> {
        map(tuple((le_u32, le_u16, le_u8)), |data| MapMeshCell {
            height: data.0,
            texture: data.1,
            brightness: data.2,
        })
    }
}

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
    pub fn parser<'a>() -> impl Fn(&'a [u8]) -> IResult<&'a [u8], Self> {
        map(
            tuple((
                string_6,
                count(MapMeshCell::parser(), 16 * 16 + 1),
                le_u8,
                le_u8,
                le_f32,
                count(le_u8, 256),
                le_f32,
                le_f32,
                count(le_u8, 20),
            )),
            |(
                name,
                cells,
                density,
                unk0,
                sea_level,
                extra_data,
                height_min,
                height_max,
                unk0_buffer,
            )| MapBlock {
                name,
                cells,
                density,
                unk0,
                sea_level,
                extra_data,
                height_min,
                height_max,
                unk0_buffer,
            },
        )
    }
}

#[inline]
pub fn string_6<'a, E: nom::error::ParseError<&'a [u8]>>(
    i: &'a [u8],
) -> IResult<&'a [u8], String, E> {
    use encoding::{all::WINDOWS_949, DecoderTrap, Encoding};
    map(take(6usize), |s| {
        WINDOWS_949.decode(s, DecoderTrap::Replace).unwrap()
    })(i)
}

pub struct JmxMapMesh {
    pub blocks: Vec<MapBlock>,
}


impl JmxMapMesh {
    pub fn parse(i: &[u8]) -> Result<Self, nom::Err<(&[u8], nom::error::ErrorKind)>> {
        map(
            preceded(tag(b"JMXVMAPM1000"), count(MapBlock::parser(), 6 * 6)),
            |blocks| JmxMapMesh { blocks },
        )(i)
        .map(|(_, this)| this)
    }
}
