use mint::Vector2;
use nom::bytes::complete::tag;
use nom::combinator::{cond, flat_map, map};
use nom::error::ParseError;
use nom::multi::count;
use nom::number::complete::{le_f32, le_u32, le_u8};
use nom::sequence::{pair, preceded, tuple};
use nom::IResult;
use struple::Struple;

#[cfg(feature = "serde")]
use serde::Serialize;

use std::path::PathBuf;

use crate::parser_ext::combinator::{struple, struple_map};
use crate::parser_ext::multi::parse_objects_u32;
use crate::parser_ext::number::{vector2_f32, vector6_f32};
use crate::parser_ext::string::{sized_path, sized_string};
use crate::SrFile;
use crate::{ResourceAnimationType, ResourceType};

#[derive(Debug, Struple)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct BoundingBox {
    pub root_mesh: String,
    pub bounding_box0: [f32; 6],
    pub bounding_box1: [f32; 6],
    pub extra_bounding_data: Vec<u8>,
}

impl BoundingBox {
    fn parse<'a, E: ParseError<&'a [u8]>>(i: &'a [u8]) -> IResult<&'a [u8], Self, E> {
        struple((
            sized_string,
            vector6_f32,
            vector6_f32,
            map(
                flat_map(le_u32, |val| cond(val != 0, count(le_u8, 64))),
                |ebd| ebd.unwrap_or_default(),
            ),
        ))(i)
    }
}

#[derive(Debug, Struple)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct MaterialDescriptor {
    pub id: u32,
    pub path: PathBuf,
}

impl MaterialDescriptor {
    fn parse<'a, E: ParseError<&'a [u8]>>(i: &'a [u8]) -> IResult<&'a [u8], Self, E> {
        struple_map(pair(le_u32, sized_path))(i)
    }
}

#[derive(Debug, Struple)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct Animation {
    pub unk0: u32,
    pub unk1: u32,
    pub paths: Vec<PathBuf>,
}

impl Animation {
    fn parse<'a, E: ParseError<&'a [u8]>>(i: &'a [u8]) -> IResult<&'a [u8], Self, E> {
        struple((le_u32, le_u32, parse_objects_u32(sized_path)))(i)
    }
}

#[derive(Debug, Struple)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct MeshGroup {
    pub name: String,
    pub file_indices: Vec<u32>,
}

impl MeshGroup {
    fn parse<'a, E: ParseError<&'a [u8]>>(i: &'a [u8]) -> IResult<&'a [u8], Self, E> {
        struple_map(pair(sized_string, parse_objects_u32(le_u32)))(i)
    }
}

#[derive(Debug, Struple)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct AnimationEvent {
    pub key_time: u32,
    pub typ: u32,
    pub unk0: u32,
    pub unk1: u32,
}

impl AnimationEvent {
    fn parse<'a, E: ParseError<&'a [u8]>>(i: &'a [u8]) -> IResult<&'a [u8], Self, E> {
        struple((le_u32, le_u32, le_u32, le_u32))(i)
    }
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct AnimationGroupEntry {
    pub typ: ResourceAnimationType,
    pub file_index: u32,
    pub events: Vec<AnimationEvent>,
    pub walk_length: f32,
    pub walk_graph: Vec<Vector2<f32>>,
}

impl AnimationGroupEntry {
    fn parse<'a, E: ParseError<&'a [u8]>>(i: &'a [u8]) -> IResult<&'a [u8], Self, E> {
        map(
            tuple((
                ResourceAnimationType::parse,
                le_u32,
                parse_objects_u32(AnimationEvent::parse),
                flat_map(le_u32, |c| pair(le_f32, count(vector2_f32, c as usize))),
            )),
            |(typ, file_index, events, (walk_length, walk_graph))| AnimationGroupEntry {
                typ,
                file_index,
                events,
                walk_length,
                walk_graph,
            },
        )(i)
    }
}

#[derive(Debug, Struple)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct AnimationGroup {
    pub name: String,
    pub animations: Vec<AnimationGroupEntry>,
}

impl AnimationGroup {
    fn parse<'a, E: ParseError<&'a [u8]>>(i: &'a [u8]) -> IResult<&'a [u8], Self, E> {
        struple_map(pair(
            sized_string,
            parse_objects_u32(AnimationGroupEntry::parse),
        ))(i)
    }
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct JmxRes {
    pub header: JmxResHeader,
    pub bounding_box: BoundingBox,
    pub material_sets: Vec<MaterialDescriptor>,
    pub mesh_paths: Vec<(PathBuf, Option<u32>)>,
    pub animation: Animation,
    pub skeleton_paths: Vec<(PathBuf, Vec<u8>)>,
    pub mesh_groups: Vec<MeshGroup>,
    pub animation_groups: Vec<AnimationGroup>,
}

impl SrFile for JmxRes {
    type Input = [u8];
    type Output = Self;
    fn nom_parse<'i, E: ParseError<&'i Self::Input>>(
        i: &'i Self::Input,
    ) -> IResult<&'i Self::Input, Self::Output, E> {
        let (_, header) = nom::error::context("resource header", JmxResHeader::parse)(i)?;
        let (_, bounding_box) = BoundingBox::parse(&i[header.bounding_box_offset as usize..])?;
        let (_, material_sets) =
            parse_objects_u32(MaterialDescriptor::parse)(&i[header.material_offset as usize..])?;
        let (_, mesh_paths) = parse_objects_u32(pair(sized_path, cond(header.unk0 == 1, le_u32)))(
            &i[header.mesh_offset as usize..],
        )?;
        let (_, animation) = Animation::parse(&i[header.animation_offset as usize..])?;
        let (_, skeleton_paths) = parse_objects_u32(pair(sized_path, parse_objects_u32(le_u8)))(
            &i[header.skeleton_offset as usize..],
        )?;
        let (_, mesh_groups) =
            parse_objects_u32(MeshGroup::parse)(&i[header.mesh_group_offset as usize..])?;
        let (_, animation_groups) =
            parse_objects_u32(AnimationGroup::parse)(&i[header.animation_group_offset as usize..])?;

        Ok((
            &[],
            JmxRes {
                header,
                bounding_box,
                material_sets,
                mesh_paths,
                animation,
                skeleton_paths,
                mesh_groups,
                animation_groups,
            },
        ))
    }
}

#[derive(Debug, Struple)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct JmxResHeader {
    pub material_offset: u32,
    pub mesh_offset: u32,
    pub skeleton_offset: u32,
    pub animation_offset: u32,
    pub mesh_group_offset: u32,
    pub animation_group_offset: u32,
    // FIXME effects
    pub sound_effect_offset: u32,
    pub bounding_box_offset: u32,
    pub unk0: u32,
    pub unk1: u32,
    pub unk2: u32,
    pub unk3: u32,
    pub unk4: u32,
    pub res_type: ResourceType,
    pub name: String,
    //pub unk5: [u8; 48],
}

impl JmxResHeader {
    pub fn parse<'a, E: ParseError<&'a [u8]>>(i: &'a [u8]) -> IResult<&'a [u8], Self, E> {
        preceded(
            tag("JMXVRES 0109"),
            struple((
                le_u32,
                le_u32,
                le_u32,
                le_u32,
                le_u32,
                le_u32,
                le_u32,
                le_u32,
                le_u32,
                le_u32,
                le_u32,
                le_u32,
                le_u32,
                ResourceType::parse,
                sized_string,
            )),
        )(i)
    }
}
