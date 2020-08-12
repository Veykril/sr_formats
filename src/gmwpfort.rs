use mint::Vector3;
use nom::error::ParseError;
use nom::number::complete::{le_u16, le_u32};
use struple::Struple;

#[cfg(feature = "serde")]
use serde::Serialize;

use crate::parser_ext::combinator::struple;
use crate::parser_ext::multi::parse_objects_u32;
use crate::parser_ext::number::vector3_f32;
use crate::parser_ext::string::sized_string;

#[derive(Debug, Struple)]
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
        parse_objects_u32(struple((sized_string, le_u16, le_u16, vector3_f32, le_u32)))(i)
            .map(|(_, r)| r)
    }
}
