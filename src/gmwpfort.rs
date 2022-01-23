use mint::Vector3;
use nom::combinator::map;
use nom::number::complete::{le_u16, le_u32};
use nom::sequence::tuple;

#[cfg(feature = "serde")]
use serde_derive::Serialize;

use crate::parser_ext::multi::parse_objects_u32;
use crate::parser_ext::number::vector3_f32;
use crate::parser_ext::string::sized_string;
use crate::ttr_closure;

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct Gmwpfort {
    pub name: Box<str>,
    pub region_id: u16,
    pub pad: u16,
    pub offset: Vector3<f32>,
    pub world_id: u32,
}

impl Gmwpfort {
    pub fn parse<'i>(i: &'i [u8]) -> nom::IResult<&'i [u8], Box<[Gmwpfort]>> {
        parse_objects_u32(map(
            tuple((sized_string, le_u16, le_u16, vector3_f32, le_u32)),
            ttr_closure! {
                Gmwpfort {
                    name, region_id, pad, offset, world_id
                }
            },
        ))(i)
    }
}
