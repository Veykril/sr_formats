use nom::combinator::map;
use nom::error::ParseError;
use nom::number::complete::{le_u16, le_u32};
use nom::sequence::tuple;

#[cfg(feature = "serde")]
use serde::Serialize;

use crate::{parse_objects_u32, sized_string, vector3_f32, Vector3};

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct Gmwpfort {
    pub name: String,
    pub region_id: u16,
    pub pad: u16,
    pub offset: Vector3<f32>,
    pub world_id: u32,
}

impl Gmwpfort {
    pub fn parse<'a, E: ParseError<&'a [u8]>>(i: &'a [u8]) -> Result<Vec<Self>, nom::Err<E>> {
        parse_objects_u32(map(
            tuple((sized_string, le_u16, le_u16, vector3_f32, le_u32)),
            |(name, region_id, pad, offset, world_id)| Gmwpfort {
                name,
                region_id,
                pad,
                offset,
                world_id,
            },
        ))(i)
        .map(|(_, r)| r)
    }
}
