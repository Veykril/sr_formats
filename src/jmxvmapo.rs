use nalgebra::Vector3;
use nom::{
    bytes::complete::tag,
    combinator::map,
    multi::count,
    number::complete::{le_f32, le_u16, le_u32},
    sequence::{preceded, tuple},
    IResult,
};

use crate::{parse_objects_u16, vector3_f32};

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
    pub fn parser<'a>() -> impl Fn(&'a [u8]) -> IResult<&'a [u8], Self> {
        map(
            tuple((le_u32, vector3_f32, le_u16, le_f32, le_u32, le_u16, le_u16)),
            |data| MapObject {
                id: data.0,
                position: data.1,
                visibility_flag: data.2,
                theta: data.3,
                unique_id: data.4,
                scale: data.5,
                region: data.6,
            },
        )
    }
}

pub struct MapObjectGroup {
    pub entries: Vec<MapObject>,
}

impl MapObjectGroup {
    pub fn parser<'a>() -> impl Fn(&'a [u8]) -> IResult<&'a [u8], Self> {
        map(parse_objects_u16(MapObject::parser()), |entries| {
            MapObjectGroup { entries }
        })
    }
}

pub struct JmxMapObject {
    pub objects: Vec<MapObjectGroup>,
}

impl JmxMapObject {
    pub fn parse(i: &[u8]) -> Result<Self, nom::Err<(&[u8], nom::error::ErrorKind)>> {
        map(
            preceded(tag(b"JMXVMAPO1001"), count(MapObjectGroup::parser(), 144)),
            |objects| JmxMapObject { objects },
        )(i)
        .map(|(_, this)| this)
    }
}
