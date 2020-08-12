use nom::bytes::complete::tag;
use nom::combinator::{map, map_res};
use nom::error::ParseError;
use nom::number::complete::le_u8;
use nom::sequence::terminated;
use nom::IResult;
use struple::Struple;

#[cfg(feature = "serde")]
use serde::Serialize;

use std::net::Ipv4Addr;

use crate::parser_ext::combinator::struple;
use crate::parser_ext::multi::parse_objects_u8;
use crate::parser_ext::string::sized_string;
use crate::SrFile;

#[derive(Debug, Struple)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct DivisionInfo {
    pub content_id: u8,
    pub divisions: Vec<Division>,
}

impl SrFile for DivisionInfo {
    type Input = [u8];
    type Output = Self;
    fn nom_parse<'i, E: ParseError<&'i Self::Input>>(
        i: &'i Self::Input,
    ) -> IResult<&'i Self::Input, Self::Output, E> {
        struple((le_u8, parse_objects_u8(Division::parse)))(i)
    }
}

#[derive(Debug, Struple)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct Division {
    pub name: String,
    pub gateways: Vec<Gateway>,
}

impl Division {
    fn parse<'a, E: ParseError<&'a [u8]>>(i: &'a [u8]) -> IResult<&'a [u8], Self, E> {
        struple((
            terminated(sized_string, tag(b"\00")),
            parse_objects_u8(Gateway::parse),
        ))(i)
    }
}

#[derive(Debug, Struple)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct Gateway {
    ip: Ipv4Addr,
}

impl Gateway {
    fn parse<'a, E: ParseError<&'a [u8]>>(i: &'a [u8]) -> IResult<&'a [u8], Self, E> {
        map(
            terminated(map_res(sized_string, |addr| addr.parse()), tag(b"\00")),
            |ip| Gateway { ip },
        )(i)
    }
}
