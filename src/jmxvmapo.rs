use mint::Vector3;
use nom::bytes::complete::tag;
use nom::combinator::map;
use nom::error::ParseError;
use nom::multi::count;
use nom::number::complete::{le_f32, le_u16, le_u32};
use nom::sequence::preceded;
use nom::IResult;
use struple::Struple;

#[cfg(feature = "serde")]
use serde::Serialize;

use crate::{parse_objects_u16, struple, vector3_f32};

#[derive(Debug, Struple)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct MapObject {
    pub id: u32,
    pub position: Vector3<f32>,
    pub visibility_flag: u16,
    pub theta: f32,
    pub unique_id: u32,
    pub scale: u16,
    pub region: u16,
}

impl MapObject {
    pub fn parse<'a, E: ParseError<&'a [u8]>>(i: &'a [u8]) -> IResult<&'a [u8], Self, E> {
        struple((le_u32, vector3_f32, le_u16, le_f32, le_u32, le_u16, le_u16))(i)
    }
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct MapObjectGroup {
    pub entries: Vec<MapObject>,
}

impl MapObjectGroup {
    pub fn parse<'a, E: ParseError<&'a [u8]>>(i: &'a [u8]) -> IResult<&'a [u8], Self, E> {
        map(parse_objects_u16(MapObject::parse), |entries| {
            MapObjectGroup { entries }
        })(i)
    }
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct JmxMapObject {
    pub objects: Vec<MapObjectGroup>,
}

impl JmxMapObject {
    pub fn parse<'a, E: ParseError<&'a [u8]>>(i: &'a [u8]) -> Result<Self, nom::Err<E>> {
        map(
            preceded(tag(b"JMXVMAPO1001"), count(MapObjectGroup::parse, 144)),
            |objects| JmxMapObject { objects },
        )(i)
        .map(|(_, this)| this)
    }
}
