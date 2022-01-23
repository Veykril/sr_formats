use mint::Vector3;
use nom::bytes::complete::tag;
use nom::combinator::map;
use nom::number::complete::{le_f32, le_u16, le_u32};
use nom::sequence::{preceded, tuple};
use nom::IResult;

#[cfg(feature = "serde")]
use serde::Serialize;

use crate::parser_ext::multi::{count, parse_objects_u16};
use crate::parser_ext::number::vector3_f32;
use crate::ttr_closure;

#[derive(Debug)]
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
    fn parse<'a>(i: &'a [u8]) -> IResult<&'a [u8], Self> {
        map(
            tuple((le_u32, vector3_f32, le_u16, le_f32, le_u32, le_u16, le_u16)),
            ttr_closure! {
                MapObject {
                    id,
                    position,
                    visibility_flag,
                    theta,
                    unique_id,
                    scale,
                    region
                }
            },
        )(i)
    }
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct MapObjectGroup {
    pub entries: Box<[MapObject]>,
}

impl MapObjectGroup {
    fn parse<'a>(i: &'a [u8]) -> IResult<&'a [u8], Self> {
        map(parse_objects_u16(MapObject::parse), |entries| {
            MapObjectGroup { entries }
        })(i)
    }
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct JmxMapObject {
    pub objects: Box<[MapObjectGroup]>,
}

impl JmxMapObject {
    pub fn parse<'i>(i: &'i [u8]) -> IResult<&'i [u8], Self> {
        map(
            preceded(tag(b"JMXVMAPO1001"), count(MapObjectGroup::parse, 144)),
            |objects| JmxMapObject { objects },
        )(i)
    }
}
