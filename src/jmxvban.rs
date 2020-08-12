use mint::{Vector3, Vector4};
use nom::bytes::complete::tag;
use nom::combinator::map;
use nom::error::ParseError;
use nom::number::complete::le_u32;
use nom::sequence::{pair, preceded};
use nom::IResult;
use struple::Struple;

#[cfg(feature = "serde")]
use serde::Serialize;

use crate::parser_ext::combinator::{struple, struple_map};
use crate::parser_ext::multi::parse_objects_u32;
use crate::parser_ext::number::{vector3_f32, vector4_f32};
use crate::parser_ext::string::sized_string;
use crate::SrFile;

#[derive(Debug, Struple)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct KeyFrame {
    pub rotation: Vector4<f32>,
    pub translation: Vector3<f32>,
}

impl KeyFrame {
    fn parse<'a, E: ParseError<&'a [u8]>>(i: &'a [u8]) -> IResult<&'a [u8], Self, E> {
        struple_map(pair(vector4_f32, vector3_f32))(i)
    }
}

#[derive(Debug, Struple)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct AnimatedBone {
    pub name: String,
    pub keyframes: Vec<KeyFrame>,
}

impl AnimatedBone {
    fn parse<'a, E: ParseError<&'a [u8]>>(i: &'a [u8]) -> IResult<&'a [u8], Self, E> {
        struple_map(pair(sized_string, parse_objects_u32(KeyFrame::parse)))(i)
    }
}

#[derive(Debug, Struple)]
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

impl SrFile for JmxAnimation {
    type Input = [u8];
    type Output = Self;
    fn nom_parse<'i, E: ParseError<&'i Self::Input>>(
        i: &'i Self::Input,
    ) -> IResult<&'i Self::Input, Self::Output, E> {
        preceded(
            tag(b"MXVBAN 0102"),
            struple((
                le_u32,
                le_u32,
                sized_string,
                le_u32,
                le_u32,
                map(le_u32, |int| int != 0),
                parse_objects_u32(le_u32),
                parse_objects_u32(AnimatedBone::parse),
            )),
        )(i)
    }
}
