use nom::bytes::complete::{tag, take_till};
use nom::character::complete::{char, digit1, hex_digit1, line_ending, multispace1};
use nom::combinator::{flat_map, map, map_res};
use nom::error::ParseError;
use nom::multi::many_m_n;
use nom::sequence::{delimited, preceded, terminated};
use nom::IResult;
use struple::Struple;

use std::path::PathBuf;
use std::str::FromStr;

use crate::{parse_u16_str, parse_u32_hex_str, struple};

fn parse_f32_hex_dumped_str<'a, E: ParseError<&'a str>>(
    input: &'a str,
) -> IResult<&'a str, f32, E> {
    map(parse_u32_hex_str, |num| {
        f32::from_le_bytes(num.to_le_bytes())
    })(input)
}

#[derive(Debug, PartialEq, Struple)]
pub struct ObjectStringIfo {
    pub index: u32,
    pub flag: u32,
    pub x_sec: u16,
    pub y_sec: u16,
    pub x_offset: f32,
    pub y_offset: f32,
    pub z_offset: f32,
    pub yaw: f32,
    pub string: String,
}

impl ObjectStringIfo {
    pub fn parse<'a, E: ParseError<&'a str>>(i: &'a str) -> Result<Vec<Self>, nom::Err<E>> {
        preceded(
            tag("JMXVOBJI1000\n"),
            flat_map(terminated(parse_u16_str, line_ending), |count| {
                many_m_n(count as usize, count as usize, Self::parse_single)
            }),
        )(i)
        .map(|(_, r)| r)
    }

    fn parse_single<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, Self, E> {
        terminated(
            struple((
                parse_u32_hex_str,
                preceded(multispace1, parse_u32_hex_str),
                preceded(multispace1, parse_u16_str),
                preceded(multispace1, parse_u16_str),
                preceded(multispace1, parse_f32_hex_dumped_str),
                preceded(multispace1, parse_f32_hex_dumped_str),
                preceded(multispace1, parse_f32_hex_dumped_str),
                preceded(multispace1, parse_f32_hex_dumped_str),
                preceded(
                    multispace1,
                    map(
                        delimited(char('"'), take_till(|c| c == '"'), char('"')),
                        From::from,
                    ),
                ),
            )),
            line_ending,
        )(i)
    }
}

#[derive(Debug, PartialEq, Struple)]
pub struct ObjectIfo {
    pub index: u16,
    pub flag: u32,
    pub path: PathBuf,
}

impl ObjectIfo {
    pub fn parse<'a, E: ParseError<&'a str>>(i: &'a str) -> Result<Vec<Self>, nom::Err<E>> {
        preceded(
            tag("JMXVOBJI1000\n"),
            flat_map(terminated(parse_u16_str, line_ending), |count| {
                many_m_n(count as usize, count as usize, Self::parse_single)
            }),
        )(i)
        .map(|(_, r)| r)
    }

    fn parse_single<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, Self, E> {
        terminated(
            struple((
                map_res(digit1, u16::from_str),
                preceded(
                    multispace1,
                    preceded(
                        tag("0x"),
                        map_res(hex_digit1, |s| u32::from_str_radix(s, 16)),
                    ),
                ),
                preceded(
                    multispace1,
                    map(
                        delimited(char('"'), take_till(|c| c == '"'), char('"')),
                        From::from,
                    ),
                ),
            )),
            line_ending,
        )(i)
    }
}

#[test]
fn objifo_single() {
    assert_eq!(
        ObjectIfo::parse_single::<nom::error::VerboseError<&str>>(
            "01057 0x00000000 \"res\\bldg\\oasis\\karakorm\\kara-obj-new\\oas_kara_obj02.bsr\"\r\n",
        ),
        Ok((
            "",
            ObjectIfo {
                index: 1057,
                flag: 0x0,
                path: PathBuf::from("res\\bldg\\oasis\\karakorm\\kara-obj-new\\oas_kara_obj02.bsr")
            }
        ))
    );
}
