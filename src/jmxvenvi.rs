use mint::Vector3;
use nom::bytes::complete::tag;
use nom::combinator::{flat_map, map};
use nom::error::ParseError;
use nom::multi::count;
use nom::number::complete::{le_f32, le_u16, le_u32};
use nom::sequence::{pair, preceded};
use nom::IResult;
use struple::Struple;

#[cfg(feature = "serde")]
use serde::Serialize;

use crate::{count_indexed, parse_objects_u32, sized_string, struple, vector3_f32};

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum GraphPoint {
    Float {
        value: f32,
        pos_on_graph: f32,
    },
    Vector {
        value: Vector3<f32>,
        pos_on_graph: f32,
    },
}

impl GraphPoint {
    pub fn parser<'a, E: ParseError<&'a [u8]>>(
        idx: usize,
    ) -> impl Fn(&'a [u8]) -> IResult<&'a [u8], Self, E> {
        move |i| {
            if idx == 7 || idx == 8 || idx == 10 || idx == 11 || idx == 12 || idx == 15 {
                map(pair(le_f32, le_f32), |(value, pos_on_graph)| {
                    GraphPoint::Float {
                        value,
                        pos_on_graph,
                    }
                })(i)
            } else {
                map(pair(vector3_f32, le_f32), |(value, pos_on_graph)| {
                    GraphPoint::Vector {
                        value,
                        pos_on_graph,
                    }
                })(i)
            }
        }
    }
}

#[derive(Debug, Struple)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct EnvironmentGroup {
    pub name: String,
    pub unk0: u16,
    pub unk1: u16,
    pub unk2: u16,
    pub unk3: u16,
    pub unk4: u16,
    pub unk5: u16,
    pub entries: Vec<EnvironmentGroupEntry>,
}

impl EnvironmentGroup {
    pub fn parse<'a, E: ParseError<&'a [u8]>>(i: &'a [u8]) -> IResult<&'a [u8], Self, E> {
        struple((
            sized_string,
            le_u16,
            le_u16,
            le_u16,
            le_u16,
            le_u16,
            le_u16,
            parse_objects_u32(EnvironmentGroupEntry::parse),
        ))(i)
    }
}

#[derive(Debug, Struple)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct EnvironmentGroupEntry {
    pub name: String,
    pub unk0: u16,
    pub unk1: u16,
    pub unk2: u16,
    pub unk3: u16,
    pub unk4: u16,
    pub unk5: u16,
    pub unk6: u16,
    pub unk7: u16,
}

impl EnvironmentGroupEntry {
    pub fn parse<'a, E: ParseError<&'a [u8]>>(i: &'a [u8]) -> IResult<&'a [u8], Self, E> {
        struple((
            sized_string,
            le_u16,
            le_u16,
            le_u16,
            le_u16,
            le_u16,
            le_u16,
            le_u16,
            le_u16,
        ))(i)
    }
}

#[derive(Debug, Struple)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct Environment {
    pub id: u16,
    pub name: String,
    pub unk0: u32,
    pub unk1: u32,
    pub fncs: Vec<Vec<GraphPoint>>, //16
}

impl Environment {
    pub fn parse<'a, E: ParseError<&'a [u8]>>(i: &'a [u8]) -> IResult<&'a [u8], Self, E> {
        struple((
            le_u16,
            sized_string,
            le_u32,
            le_u32,
            count_indexed(|i, idx| parse_objects_u32(GraphPoint::parser(idx))(i), 16),
        ))(i)
    }
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct JmxEnvironment {
    pub unk0: u16,
    pub environments: Vec<Environment>,
    pub environment_groups: Vec<EnvironmentGroup>,
}

impl JmxEnvironment {
    pub fn parse(i: &[u8]) -> Result<Self, nom::Err<(&[u8], nom::error::ErrorKind)>> {
        map(
            preceded(
                tag(b"JMXVENVI1003"),
                pair(
                    flat_map(le_u32, |c| {
                        pair(le_u16, count(Environment::parse, c as usize))
                    }),
                    parse_objects_u32(EnvironmentGroup::parse),
                ),
            ),
            |((unk0, environments), environment_groups)| JmxEnvironment {
                unk0,
                environments,
                environment_groups,
            },
        )(i)
        .map(|(_, this)| this)
    }
}
