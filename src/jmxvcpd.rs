use std::path::Path;

use nom::bytes::complete::tag;
use nom::combinator::{map, map_res};
use nom::number::complete::le_u32;
use nom::sequence::{preceded, tuple};
use nom::IResult;

#[cfg(feature = "serde")]
use serde_derive::Serialize;

use crate::parser_ext::multi::parse_objects_u32;
use crate::parser_ext::string::{sized_path, sized_string};
use crate::{ttr_closure, ResourceType};

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct JmxCompound {
    pub header: JmxCompoundHeader,
    pub collision_resource_path: Box<Path>,
    pub resource_paths: Box<[Box<Path>]>,
}

impl JmxCompound {
    pub fn parse<'i>(i: &'i [u8]) -> IResult<&'i [u8], Self> {
        let (_, header) = JmxCompoundHeader::parse(i)?;

        let (_, collision_resource_path) = sized_path(&i[header.collision_resources as usize..])?;
        let (_, resource_paths) =
            parse_objects_u32(sized_path)(&i[header.resource_list as usize..])?;

        Ok((
            &[],
            JmxCompound {
                header,
                collision_resource_path,
                resource_paths,
            },
        ))
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
    pub name: Box<str>,
    pub unk5: u32,
    pub unk6: u32,
}

impl JmxCompoundHeader {
    fn parse<'a>(i: &'a [u8]) -> IResult<&'a [u8], Self> {
        preceded(
            tag(b"JMXVCPD 0101"),
            map(
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
                ttr_closure! {
                    JmxCompoundHeader {
                        collision_resources,
                        resource_list,
                        unk0,
                        unk1,
                        unk2,
                        unk3,
                        unk4,
                        typ,
                        name,
                        unk5,
                        unk6
                    }
                },
            ),
        )(i)
    }
}
