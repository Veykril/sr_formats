use mint::Vector3;
use nom::bytes::complete::tag;
use nom::combinator::{flat_map, map};
use nom::number::complete::{le_f32, le_u16, le_u32};
use nom::sequence::{pair, preceded, tuple};
use nom::IResult;

#[cfg(feature = "serde")]
use serde_derive::Serialize;

use crate::parser_ext::multi::{count, count_indexed, parse_objects_u32};
use crate::parser_ext::{number::vector3_f32, string::sized_string};
use crate::ttr_closure;

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
    fn parser<'a>(idx: usize) -> impl Fn(&'a [u8]) -> IResult<&'a [u8], Self> {
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

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct EnvironmentGroup {
    pub name: Box<str>,
    pub unk0: u16,
    pub unk1: u16,
    pub unk2: u16,
    pub unk3: u16,
    pub unk4: u16,
    pub unk5: u16,
    pub entries: Box<[EnvironmentGroupEntry]>,
}

impl EnvironmentGroup {
    fn parse<'a>(i: &'a [u8]) -> IResult<&'a [u8], Self> {
        map(
            tuple((
                sized_string,
                le_u16,
                le_u16,
                le_u16,
                le_u16,
                le_u16,
                le_u16,
                parse_objects_u32(EnvironmentGroupEntry::parse),
            )),
            ttr_closure! {
                EnvironmentGroup {
                    name,
                    unk0,
                    unk1,
                    unk2,
                    unk3,
                    unk4,
                    unk5,
                    entries
                }
            },
        )(i)
    }
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct EnvironmentGroupEntry {
    pub name: Box<str>,
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
    fn parse<'a>(i: &'a [u8]) -> IResult<&'a [u8], Self> {
        map(
            tuple((
                sized_string,
                le_u16,
                le_u16,
                le_u16,
                le_u16,
                le_u16,
                le_u16,
                le_u16,
                le_u16,
            )),
            ttr_closure! {
                EnvironmentGroupEntry {
                    name,
                    unk0,
                    unk1,
                    unk2,
                    unk3,
                    unk4,
                    unk5,
                    unk6,
                    unk7
                }
            },
        )(i)
    }
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct Environment {
    pub id: u16,
    pub name: Box<str>,
    pub unk0: u32,
    pub unk1: u32,
    pub fncs: Box<[Box<[GraphPoint]>]>, //16
}

impl Environment {
    fn parse<'a>(i: &'a [u8]) -> IResult<&'a [u8], Self> {
        map(
            tuple((
                le_u16,
                sized_string,
                le_u32,
                le_u32,
                count_indexed(|i, idx| parse_objects_u32(GraphPoint::parser(idx))(i), 16),
            )),
            ttr_closure! {
                Environment {
                    id,
                    name,
                    unk0,
                    unk1,
                    fncs
                }
            },
        )(i)
    }
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct JmxEnvironment {
    pub unk0: u16,
    pub environments: Box<[Environment]>,
    pub environment_groups: Box<[EnvironmentGroup]>,
}

impl JmxEnvironment {
    pub fn parse<'i>(i: &'i [u8]) -> IResult<&'i [u8], Self> {
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
    }
}
