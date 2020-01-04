use nom::bytes::complete::tag;
use nom::combinator::map;
use nom::error::ParseError;
use nom::number::complete::le_u32;
use nom::sequence::{pair, preceded, tuple};
use nom::IResult;

#[cfg(feature = "serde")]
use serde::Serialize;

use crate::{parse_objects_u32, sized_string, vector3_f32, vector4_f32, Vector3, Vector4};

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct KeyFrame {
    pub rotation: Vector4<f32>,
    pub translation: Vector3<f32>,
}

impl KeyFrame {
    pub fn parse<'a, E: ParseError<&'a [u8]>>(i: &'a [u8]) -> IResult<&'a [u8], Self, E> {
        map(pair(vector4_f32, vector3_f32), |(rotation, translation)| {
            KeyFrame {
                rotation,
                translation,
            }
        })(i)
    }
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct AnimatedBone {
    pub name: String,
    pub keyframes: Vec<KeyFrame>,
}

impl AnimatedBone {
    pub fn parse<'a, E: ParseError<&'a [u8]>>(i: &'a [u8]) -> IResult<&'a [u8], Self, E> {
        map(
            pair(sized_string, parse_objects_u32(KeyFrame::parse)),
            |(name, keyframes)| AnimatedBone { name, keyframes },
        )(i)
    }
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct JmxAnimation {
    pub unk0: u32,
    pub unk1: u32,
    pub name: String,
    pub duration: u32,
    pub frames_per_second: u32,
    pub is_continuous: bool,
    pub key_frame_times: Vec<u32>,
    pub animated_bones: Vec<AnimatedBone>,
}

impl JmxAnimation {
    pub fn parse<'a, E: ParseError<&'a [u8]>>(i: &'a [u8]) -> Result<Self, nom::Err<E>> {
        map(
            preceded(
                tag(b"MXVBAN 0102"),
                tuple((
                    le_u32,
                    le_u32,
                    sized_string,
                    le_u32,
                    le_u32,
                    map(le_u32, |int| int != 0),
                    parse_objects_u32(le_u32),
                    parse_objects_u32(AnimatedBone::parse),
                )),
            ),
            |(
                unk0,
                unk1,
                name,
                duration,
                frames_per_second,
                is_continuous,
                key_frame_times,
                animated_bones,
            )| JmxAnimation {
                unk0,
                unk1,
                name,
                duration,
                frames_per_second,
                is_continuous,
                key_frame_times,
                animated_bones,
            },
        )(i)
        .map(|(_, r)| r)
    }
}
