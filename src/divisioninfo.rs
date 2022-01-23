use nom::bytes::complete::tag;
use nom::combinator::{map, map_res};
use nom::number::complete::le_u8;
use nom::sequence::{pair, terminated};
use nom::IResult;

#[cfg(feature = "serde")]
use serde_derive::Serialize;

use std::net::Ipv4Addr;

use crate::parser_ext::multi::parse_objects_u8;
use crate::parser_ext::string::sized_string;
use crate::ttr_closure;

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct DivisionInfo {
    pub content_id: u8,
    pub divisions: Box<[Division]>,
}

impl DivisionInfo {
    pub fn parse<'i>(i: &'i [u8]) -> IResult<&'i [u8], Self> {
        map(
            pair(le_u8, parse_objects_u8(Division::parse)),
            ttr_closure! {
                DivisionInfo {
                    content_id, divisions
                }
            },
        )(i)
    }
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct Division {
    pub name: Box<str>,
    pub gateways: Box<[Gateway]>,
}

impl Division {
    fn parse<'a>(i: &'a [u8]) -> IResult<&'a [u8], Self> {
        map(
            pair(
                terminated(sized_string, tag(b"\x00")),
                parse_objects_u8(Gateway::parse),
            ),
            ttr_closure! {
                Division {
                    name, gateways
                }
            },
        )(i)
    }
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct Gateway {
    ip: Ipv4Addr,
}

impl Gateway {
    fn parse<'a>(i: &'a [u8]) -> IResult<&'a [u8], Self> {
        map(
            terminated(map_res(sized_string, |addr| addr.parse()), tag(b"\x00")),
            ttr_closure! { Gateway { ip } },
        )(i)
    }
}
