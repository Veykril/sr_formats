use mint::{Vector3, Vector4};
use nom::bytes::complete::tag;
use nom::combinator::map;
use nom::error::{make_error, ErrorKind};
use nom::number::complete::le_u32;
use nom::sequence::{pair, tuple};
use nom::IResult;

#[cfg(feature = "serde")]
use serde::Serialize;

use crate::parser_ext::complete::take_fixed;
use crate::parser_ext::multi::parse_objects_u32;
use crate::parser_ext::number::{vector3_f32, vector4_f32};
use crate::parser_ext::string::sized_string;
use crate::ttr_closure;

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct KeyFrame {
    pub rotation: Vector4<f32>,
    pub translation: Vector3<f32>,
}

impl KeyFrame {
    fn parse<'a>(i: &'a [u8]) -> IResult<&'a [u8], Self> {
        map(
            pair(vector4_f32, vector3_f32),
            ttr_closure! {
                KeyFrame {
                    rotation,
                    translation
                }
            },
        )(i)
    }
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct AnimatedBone {
    pub name: Box<str>,
    pub keyframes: Box<[KeyFrame]>,
}

impl AnimatedBone {
    fn parse<'a>(i: &'a [u8]) -> IResult<&'a [u8], Self> {
        map(
            pair(sized_string, parse_objects_u32(KeyFrame::parse)),
            ttr_closure! {
                AnimatedBone {
                    name,
                    keyframes
                }
            },
        )(i)
    }
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct JmxAnimation {
    pub unk0: u32,
    pub unk1: u32,
    pub name: Box<str>,
    pub duration: u32,
    pub frames_per_second: u32,
    pub is_continuous: bool,
    pub key_frame_times: Box<[u32]>,
    pub animated_bones: Box<[AnimatedBone]>,
}

impl JmxAnimation {
    pub fn parse<'i>(i: &'i [u8]) -> IResult<&'i [u8], Self> {
        let (i, (_, version)) = pair(tag(b"JMXVBAN "), take_fixed::<4>)(i)?;

        let (i, (unk0, unk1)) = match &version {
            // b"0101" => (i, (0, 0)), FIXME: something else has changed in this version
            b"0102" => pair(le_u32, le_u32)(i)?,
            _ => return Err(nom::Err::Failure(make_error(i, ErrorKind::Tag))),
        };
        map(
            tuple((
                sized_string,
                le_u32,
                le_u32,
                map(le_u32, |int| int != 0),
                parse_objects_u32(le_u32),
                parse_objects_u32(AnimatedBone::parse),
            )),
            ttr_closure! {
                unk0, unk1 -> JmxAnimation {
                    name,
                    duration,
                    frames_per_second,
                    is_continuous,
                    key_frame_times,
                    animated_bones
                }
            },
        )(i)
    }
}
