use nom::bytes::complete::tag;
use nom::combinator::map;
use nom::error::ParseError;
use nom::multi::count;
use nom::number::complete::{le_f32, le_u16, le_u32};
use nom::sequence::{preceded, tuple};
use nom::IResult;

use crate::{parse_objects_u16, vector3_f32, Vector3};

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
        )(i)
    }
}

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
