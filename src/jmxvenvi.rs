use crate::{count_indexed, parse_objects_u32, sized_string, vector3_f32};
use nalgebra::Vector3;
use nom::{
    bytes::complete::{tag, take},
    combinator::{flat_map, map},
    multi::count,
    number::complete::{le_f32, le_u16, le_u32, le_u8},
    sequence::{pair, preceded, tuple},
    IResult,
};

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
    pub fn parser<'a>(idx: usize) -> impl Fn(&'a [u8]) -> IResult<&'a [u8], Self> {
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
    pub fn parser<'a>() -> impl Fn(&'a [u8]) -> IResult<&'a [u8], Self> {
        map(
            tuple((
                sized_string,
                le_u16,
                le_u16,
                le_u16,
                le_u16,
                le_u16,
                le_u16,
                parse_objects_u32(EnvironmentGroupEntry::parser()),
            )),
            |(name, unk0, unk1, unk2, unk3, unk4, unk5, entries)| EnvironmentGroup {
                name,
                unk0,
                unk1,
                unk2,
                unk3,
                unk4,
                unk5,
                entries,
            },
        )
    }
}

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
    pub fn parser<'a>() -> impl Fn(&'a [u8]) -> IResult<&'a [u8], Self> {
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
            |(name, unk0, unk1, unk2, unk3, unk4, unk5, unk6, unk7)| EnvironmentGroupEntry {
                name,
                unk0,
                unk1,
                unk2,
                unk3,
                unk4,
                unk5,
                unk6,
                unk7,
            },
        )
    }
}

pub struct Environment {
    pub id: u16,
    pub name: String,
    pub unk0: u32,
    pub unk1: u32,
    pub fncs: Vec<Vec<GraphPoint>>, //16
}

impl Environment {
    pub fn parser<'a>() -> impl Fn(&'a [u8]) -> IResult<&'a [u8], Self> {
        map(
            tuple((
                le_u16,
                sized_string,
                le_u32,
                le_u32,
                count_indexed(|i, idx| parse_objects_u32(GraphPoint::parser(idx))(i), 16),
            )),
            |(id, name, unk0, unk1, fncs)| Environment {
                id,
                name,
                unk0,
                unk1,
                fncs,
            },
        )
    }
}

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
                        pair(le_u16, count(Environment::parser(), c as usize))
                    }),
                    parse_objects_u32(EnvironmentGroup::parser()),
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
