use nom::bytes::complete::tag;
use nom::character::complete::{char, line_ending, multispace1};
use nom::combinator::{flat_map, map};
use nom::multi::{many0, many_m_n};
use nom::sequence::{delimited, pair, preceded, terminated, tuple};
use nom::IResult;

#[cfg(feature = "serde")]
use serde::Serialize;

use std::path::Path;

use crate::parser_ext::text::{
    parse_quoted_path_buf, parse_quoted_string, parse_u16_str, parse_u32_hex_str,
};
use crate::ttr_closure;

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Debug, PartialEq)]
pub struct TileInfo2D {
    pub index: u16,
    // tile sound?
    pub flag: u32,
    pub category: Box<str>,
    pub file: Box<Path>,
    // (model index into object.ifo, amount of objects placed)
    pub extra: Vec<(u16, u16)>,
}

impl TileInfo2D {
    pub fn parse<'i>(i: &'i str) -> IResult<&'i str, Vec<TileInfo2D>> {
        preceded(
            tag("JMXV2DTI1001\n"),
            flat_map(terminated(parse_u16_str, line_ending), |count| {
                many_m_n(count as usize, count as usize, Self::parse_single)
            }),
        )(i)
    }
}

impl TileInfo2D {
    fn parse_single<'a>(i: &'a str) -> IResult<&'a str, Self> {
        terminated(
            map(
                tuple((
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
                ttr_closure! {
                    TileInfo2D {
                        index, flag, category, file, extra
                    }
                },
            ),
            line_ending,
        )(i)
    }
}
