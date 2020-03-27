use nom::bytes::complete::tag;
use nom::character::complete::{char, line_ending, multispace1};
use nom::combinator::flat_map;
use nom::error::ParseError;
use nom::multi::{many0, many_m_n};
use nom::sequence::{delimited, pair, preceded, terminated};
use nom::IResult;
use struple::Struple;

#[cfg(feature = "serde")]
use serde::Serialize;

use std::path::PathBuf;

use crate::{
    parse_quoted_path_buf, parse_quoted_string, parse_u16_str, parse_u32_hex_str, struple,
};

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Debug, PartialEq, Struple)]
pub struct TileInfo2D {
    pub index: u16,
    // tile sound?
    pub flag: u32,
    pub category: String,
    pub file: PathBuf,
    // (model index into object.ifo, amount of objects placed)
    pub extra: Vec<(u16, u16)>,
}

impl TileInfo2D {
    pub fn parse<'a, E: ParseError<&'a str>>(i: &'a str) -> Result<Vec<Self>, nom::Err<E>> {
        preceded(
            tag("JMXV2DTI1001\n"),
            flat_map(terminated(parse_u16_str, line_ending), |count| {
                many_m_n(count as usize, count as usize, Self::parse_single)
            }),
        )(i)
        .map(|(_, r)| r)
    }

    fn parse_single<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, Self, E> {
        terminated(
            struple((
                parse_u16_str,
                preceded(multispace1, parse_u32_hex_str),
                preceded(multispace1, parse_quoted_string),
                preceded(multispace1, parse_quoted_path_buf),
                many0(preceded(
                    multispace1,
                    delimited(
                        char('{'),
                        pair(terminated(parse_u16_str, char(',')), parse_u16_str),
                        char('}'),
                    ),
                )),
            )),
            line_ending,
        )(i)
    }
}
