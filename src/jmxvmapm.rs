use nom::bytes::complete::{tag, take};
use nom::combinator::map;
use nom::error::ParseError;
use nom::multi::count;
use nom::number::complete::{le_f32, le_u16, le_u32, le_u8};
use nom::sequence::{preceded, tuple};
use nom::IResult;

pub struct MapMeshCell {
    pub height: u32,
    pub texture: u16,
    pub brightness: u8,
}

impl MapMeshCell {
    pub fn parse<'a, E: ParseError<&'a [u8]>>(i: &'a [u8]) -> IResult<&'a [u8], Self, E> {
        map(tuple((le_u32, le_u16, le_u8)), |data| MapMeshCell {
            height: data.0,
            texture: data.1,
            brightness: data.2,
        })(i)
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
    pub fn parse<'a, E: ParseError<&'a [u8]>>(i: &'a [u8]) -> IResult<&'a [u8], Self, E> {
        map(
            tuple((
                string_6,
                count(MapMeshCell::parse, 16 * 16 + 1),
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
        )(i)
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

pub struct JmxMapMesh {
    pub blocks: Vec<MapBlock>,
}

impl JmxMapMesh {
    pub fn parse<'a, E: ParseError<&'a [u8]>>(i: &'a [u8]) -> Result<Self, nom::Err<E>> {
        map(
            preceded(tag(b"JMXVMAPM1000"), count(MapBlock::parse, 6 * 6)),
            |blocks| JmxMapMesh { blocks },
        )(i)
        .map(|(_, this)| this)
    }
}
