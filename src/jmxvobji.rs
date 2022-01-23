use nom::bytes::complete::tag;
use nom::character::complete::{line_ending, multispace1};
use nom::combinator::{flat_map, map};
use nom::multi::many_m_n;
use nom::sequence::{preceded, terminated, tuple};
use nom::IResult;

#[cfg(feature = "serde")]
use serde_derive::Serialize;

use std::path::Path;

use crate::parser_ext::text::{
    parse_quoted_path_buf, parse_quoted_string, parse_u16_str, parse_u32_hex_str, parse_u8_str,
};
use crate::ttr_closure;

fn parse_f32_hex_dumped_str<'i>(input: &'i str) -> IResult<&'i str, f32> {
    map(parse_u32_hex_str, |num| {
        f32::from_le_bytes(num.to_le_bytes())
    })(input)
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Debug, PartialEq)]
pub struct ObjectStringIfo {
    pub index: u32,
    pub flag: u32,
    pub x_sec: u8,
    pub y_sec: u8,
    pub x_offset: f32,
    pub y_offset: f32,
    pub z_offset: f32,
    pub yaw: f32,
    pub string: Box<str>,
}

impl ObjectStringIfo {
    pub fn parse<'i>(i: &'i str) -> IResult<&'i str, Vec<ObjectStringIfo>> {
        preceded(
            tag("JMXVOBJI1000\n"),
            flat_map(terminated(parse_u16_str, line_ending), |count| {
                many_m_n(count as usize, count as usize, Self::parse_single)
            }),
        )(i)
    }
}

impl ObjectStringIfo {
    fn parse_single<'i>(i: &'i str) -> IResult<&'i str, Self> {
        terminated(
            map(
                tuple((
                    parse_u32_hex_str,
                    preceded(multispace1, parse_u32_hex_str),
                    preceded(multispace1, parse_u8_str),
                    preceded(multispace1, parse_u8_str),
                    preceded(multispace1, parse_f32_hex_dumped_str),
                    preceded(multispace1, parse_f32_hex_dumped_str),
                    preceded(multispace1, parse_f32_hex_dumped_str),
                    preceded(multispace1, parse_f32_hex_dumped_str),
                    preceded(multispace1, parse_quoted_string),
                )),
                ttr_closure! {
                    ObjectStringIfo {
                        index,
                        flag,
                        x_sec,
                        y_sec,
                        x_offset,
                        y_offset,
                        z_offset,
                        yaw,
                        string,
                    }
                },
            ),
            line_ending,
        )(i)
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Debug, PartialEq)]
pub struct ObjectIfo {
    pub index: u16,
    pub flag: u32,
    pub path: Box<Path>,
}

impl ObjectIfo {
    pub fn parse<'i>(i: &'i str) -> IResult<&'i str, Vec<ObjectIfo>> {
        preceded(
            tag("JMXVOBJI1000\n"),
            flat_map(terminated(parse_u16_str, line_ending), |count| {
                many_m_n(count as usize, count as usize, Self::parse_single)
            }),
        )(i)
    }
}

impl ObjectIfo {
    fn parse_single<'i>(i: &'i str) -> IResult<&'i str, Self> {
        terminated(
            map(
                tuple((
                    parse_u16_str,
                    preceded(multispace1, parse_u32_hex_str),
                    preceded(multispace1, parse_quoted_path_buf),
                )),
                ttr_closure! {
                    ObjectIfo {
                        index, flag, path
                    }
                },
            ),
            line_ending,
        )(i)
    }
}

#[test]
fn objifo_single() {
    assert_eq!(
        ObjectIfo::parse_single(
            "01057 0x00000000 \"res\\bldg\\oasis\\karakorm\\kara-obj-new\\oas_kara_obj02.bsr\"\r\n",
        ),
        Ok((
            "",
            ObjectIfo {
                index: 1057,
                flag: 0x0,
                path: std::path::PathBuf::from(
                    "res\\bldg\\oasis\\karakorm\\kara-obj-new\\oas_kara_obj02.bsr"
                )
                .into()
            }
        ))
    );
}
