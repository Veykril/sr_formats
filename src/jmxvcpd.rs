use nom::bytes::complete::tag;
use nom::combinator::map_res;
use nom::error::ParseError;
use nom::number::complete::le_u32;
use nom::sequence::preceded;
use nom::IResult;
use struple::Struple;

#[cfg(feature = "serde")]
use serde::Serialize;

use std::{convert::TryFrom, path::PathBuf};

use crate::parser_ext::combinator::struple;
use crate::parser_ext::{
    multi::parse_objects_u32,
    string::{sized_path, sized_string},
};
use crate::ResourceType;

#[derive(Debug, Struple)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct JmxCompound {
    pub header: JmxCompoundHeader,
    pub collision_resource_path: PathBuf,
    pub resource_paths: Vec<PathBuf>,
}

impl JmxCompound {
    pub fn parse<'a, E: ParseError<&'a [u8]>>(i: &'a [u8]) -> Result<Self, nom::Err<E>> {
        let (_, header) = JmxCompoundHeader::parse(i)?;

        let (_, collision_resource_path) = sized_path(&i[header.collision_resources as usize..])?;
        let (_, resource_paths) =
            parse_objects_u32(sized_path)(&i[header.resource_list as usize..])?;

        Ok(JmxCompound {
            header,
            collision_resource_path,
            resource_paths,
        })
    }
}

#[derive(Debug, Struple)]
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
        preceded(
            tag(b"JMXVCPD 0101"),
            struple((
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
        )(i)
    }
}
