use mint::{Vector3, Vector4};
use nom::bytes::complete::tag;
use nom::error::ParseError;
use nom::number::complete::{le_u32, le_u8};
use nom::sequence::preceded;
use nom::IResult;
use struple::Struple;

#[cfg(feature = "serde")]
use serde::Serialize;

use crate::{parse_objects_u32, sized_string, struple, vector3_f32, vector4_f32};

#[derive(Debug, Struple)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct Bone {
    pub unk: u8,
    pub name: String,
    pub parent_name: String,
    pub rotation_to_parent: Vector4<f32>,
    pub translation_to_parent: Vector3<f32>,
    pub rotation_to_origin: Vector4<f32>,
    pub translation_to_origin: Vector3<f32>,
    pub rotation_to_unknown: Vector4<f32>,
    pub translation_to_unknown: Vector3<f32>,
    pub children: Vec<String>,
}

impl Bone {
    pub fn parse<'a, E: ParseError<&'a [u8]>>(i: &'a [u8]) -> IResult<&'a [u8], Self, E> {
        struple((
            le_u8,
            sized_string,
            sized_string,
            vector4_f32,
            vector3_f32,
            vector4_f32,
            vector3_f32,
            vector4_f32,
            vector3_f32,
            parse_objects_u32(sized_string),
        ))(i)
    }
}

#[derive(Debug, Struple)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct JmxSkeleton {
    pub bones: Vec<Bone>,
    pub unk0: u32,
    pub unk1: u32,
}

impl JmxSkeleton {
    pub fn parse<'a, E: ParseError<&'a [u8]>>(i: &'a [u8]) -> Result<Self, nom::Err<E>> {
        preceded(
            tag("JMXVBSK 0101"),
            struple((parse_objects_u32(Bone::parse), le_u32, le_u32)),
        )(i)
        .map(|(_, r)| r)
    }
}
