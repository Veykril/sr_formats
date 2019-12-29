use nom::{
    bytes::complete::tag,
    combinator::map,
    number::complete::{le_u32, le_u8},
    sequence::{preceded, tuple},
    IResult,
};
#[cfg(feature = "serde")]
use serde::Serialize;

use crate::{parse_objects_u32, sized_string, vector3_f32, vector4_f32, Vector3, Vector4};

#[derive(Debug)]
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
    pub fn parser<'a>() -> impl Fn(&'a [u8]) -> IResult<&'a [u8], Self> {
        map(
            tuple((
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
            )),
            |data| Bone {
                unk: data.0,
                name: data.1,
                parent_name: data.2,
                rotation_to_parent: data.3,
                translation_to_parent: data.4,
                rotation_to_origin: data.5,
                translation_to_origin: data.6,
                rotation_to_unknown: data.7,
                translation_to_unknown: data.8,
                children: data.9,
            },
        )
    }
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct JmxSkeleton {
    pub bones: Vec<Bone>,
    pub unk0: u32,
    pub unk1: u32,
}

impl JmxSkeleton {
    pub fn parse(i: &[u8]) -> Result<Self, nom::Err<(&[u8], nom::error::ErrorKind)>> {
        map(
            preceded(
                tag("JMXVBSK 0101"),
                tuple((parse_objects_u32(Bone::parser()), le_u32, le_u32)),
            ),
            |data| JmxSkeleton {
                bones: data.0,
                unk0: data.1,
                unk1: data.2,
            },
        )(i)
        .map(|r| r.1)
    }
}
