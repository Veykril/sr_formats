use mint::{Vector3, Vector4};
use nom::bytes::complete::tag;
use nom::combinator::map;
use nom::number::complete::{le_u32, le_u8};
use nom::sequence::{preceded, tuple};
use nom::IResult;

#[cfg(feature = "serde")]
use serde::Serialize;

use crate::parser_ext::multi::parse_objects_u32;
use crate::parser_ext::number::{vector3_f32, vector4_f32};
use crate::parser_ext::string::sized_string;
use crate::ttr_closure;

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct Bone {
    pub unk: u8,
    pub name: Box<str>,
    pub parent_name: Box<str>,
    pub rotation_to_parent: Vector4<f32>,
    pub translation_to_parent: Vector3<f32>,
    pub rotation_to_origin: Vector4<f32>,
    pub translation_to_origin: Vector3<f32>,
    pub rotation_to_unknown: Vector4<f32>,
    pub translation_to_unknown: Vector3<f32>,
    pub children: Box<[Box<str>]>,
}

impl Bone {
    fn parse<'a>(i: &'a [u8]) -> IResult<&'a [u8], Self> {
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
            ttr_closure! {
                Bone {
                    unk,
                    name,
                    parent_name,
                    rotation_to_parent,
                    translation_to_parent,
                    rotation_to_origin,
                    translation_to_origin,
                    rotation_to_unknown,
                    translation_to_unknown,
                    children
                }
            },
        )(i)
    }
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct JmxSkeleton {
    pub bones: Box<[Bone]>,
    pub unk0: u32,
    pub unk1: u32,
}

impl JmxSkeleton {
    pub fn parse<'i>(i: &'i [u8]) -> IResult<&'i [u8], Self> {
        preceded(
            // what about the mysterious tag(b"BSK e\0\0\0\0\x03\0\0\0")
            tag("JMXVBSK 0101"),
            map(
                tuple((parse_objects_u32(Bone::parse), le_u32, le_u32)),
                ttr_closure! {
                    JmxSkeleton {
                        bones,
                        unk0,
                        unk1
                    }
                },
            ),
        )(i)
    }
}
