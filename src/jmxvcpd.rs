use nom::bytes::complete::tag;
use nom::combinator::{map, map_res};
use nom::error::ParseError;
use nom::number::complete::le_u32;
use nom::sequence::{preceded, tuple};
use nom::IResult;

#[cfg(feature = "serde")]
use serde::Serialize;

use std::{convert::TryFrom, path::PathBuf};

use crate::{parse_objects_u32, sized_string, ResourceType};

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct JmxCompound {
    pub header: JmxCompoundHeader,
    pub collision_resource_path: PathBuf,
    pub resource_paths: Vec<PathBuf>,
}

impl JmxCompound {
    pub fn parse<'a, E: ParseError<&'a [u8]>>(i: &'a [u8]) -> Result<Self, nom::Err<E>> {
        let (_, header) = JmxCompoundHeader::parse(i)?;

        let path_parser = map(sized_string, From::from);
        let (_, collision_resource_path) = path_parser(&i[header.collision_resources as usize..])?;
        let (_, resource_paths) =
            parse_objects_u32(path_parser)(&i[header.resource_list as usize..])?;

        Ok(JmxCompound {
            header,
            collision_resource_path,
            resource_paths,
        })
    }
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct JmxCompoundHeader {
    pub collision_resources: u32,
    pub resource_list: u32,
    pub unk0: u32,
    pub unk1: u32,
    pub unk2: u32,
    pub unk3: u32,
    pub unk4: u32,
    pub typ: ResourceType,
    pub name: String,
    pub unk5: u32,
    pub unk6: u32,
}

impl JmxCompoundHeader {
    pub fn parse<'a, E: ParseError<&'a [u8]>>(i: &'a [u8]) -> IResult<&'a [u8], Self, E> {
        map(
            preceded(
                tag(b"JMXVCPD 0101"),
                tuple((
                    le_u32,
                    le_u32,
                    le_u32,
                    le_u32,
                    le_u32,
                    le_u32,
                    le_u32,
                    map_res(le_u32, TryFrom::try_from),
                    sized_string,
                    le_u32,
                    le_u32,
                )),
            ),
            |data| JmxCompoundHeader {
                collision_resources: data.0,
                resource_list: data.1,
                unk0: data.2,
                unk1: data.3,
                unk2: data.4,
                unk3: data.5,
                unk4: data.6,
                typ: data.7,
                name: data.8,
                unk5: data.9,
                unk6: data.10,
            },
        )(i)
    }
}
