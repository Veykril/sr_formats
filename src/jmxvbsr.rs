use mint::Vector2;
use nom::bytes::complete::tag;
use nom::combinator::{cond, flat_map, map};
use nom::number::complete::{le_f32, le_u32, le_u8};
use nom::sequence::{pair, preceded, tuple};
use nom::IResult;

#[cfg(feature = "serde")]
use serde::Serialize;

use std::path::Path;

use crate::parser_ext::multi::{count, parse_objects_u32};
use crate::parser_ext::number::{vector2_f32, vector6_f32};
use crate::parser_ext::string::{sized_path, sized_string};
use crate::{ttr_closure, ResourceAnimationType, ResourceType};

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct BoundingBox {
    pub root_mesh: Box<str>,
    pub bounding_box0: [f32; 6],
    pub bounding_box1: [f32; 6],
    pub extra_bounding_data: Box<[u8]>,
}

impl BoundingBox {
    fn parse<'a>(i: &'a [u8]) -> IResult<&'a [u8], Self> {
        map(
            tuple((
                sized_string,
                vector6_f32,
                vector6_f32,
                map(
                    flat_map(le_u32, |val| cond(val != 0, count(le_u8, 64))),
                    |ebd| ebd.unwrap_or_default(),
                ),
            )),
            ttr_closure! {
                BoundingBox {
                    root_mesh,
                    bounding_box0,
                    bounding_box1,
                    extra_bounding_data,
                }
            },
        )(i)
    }
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct MaterialDescriptor {
    pub id: u32,
    pub path: Box<Path>,
}

impl MaterialDescriptor {
    fn parse<'a>(i: &'a [u8]) -> IResult<&'a [u8], Self> {
        map(
            pair(le_u32, sized_path),
            ttr_closure! {
                MaterialDescriptor {
                    id, path
                }
            },
        )(i)
    }
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct Animation {
    pub unk0: u32,
    pub unk1: u32,
    pub paths: Box<[Box<Path>]>,
}

impl Animation {
    fn parse<'a>(i: &'a [u8]) -> IResult<&'a [u8], Self> {
        map(
            tuple((le_u32, le_u32, parse_objects_u32(sized_path))),
            ttr_closure! {
                Animation {
                    unk0, unk1, paths
                }
            },
        )(i)
    }
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct MeshGroup {
    pub name: Box<str>,
    pub file_indices: Box<[u32]>,
}

impl MeshGroup {
    fn parse<'a>(i: &'a [u8]) -> IResult<&'a [u8], Self> {
        map(
            pair(sized_string, parse_objects_u32(le_u32)),
            ttr_closure! {
                MeshGroup {
                    name, file_indices
                }
            },
        )(i)
    }
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct AnimationEvent {
    pub key_time: u32,
    pub typ: u32,
    pub unk0: u32,
    pub unk1: u32,
}

impl AnimationEvent {
    fn parse<'a>(i: &'a [u8]) -> IResult<&'a [u8], Self> {
        map(
            tuple((le_u32, le_u32, le_u32, le_u32)),
            ttr_closure! {
                AnimationEvent {
                    key_time,
                    typ,
                    unk0,
                    unk1
                }
            },
        )(i)
    }
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct AnimationGroupEntry {
    pub typ: ResourceAnimationType,
    pub file_index: u32,
    pub events: Box<[AnimationEvent]>,
    pub walk_length: f32,
    pub walk_graph: Box<[Vector2<f32>]>,
}

impl AnimationGroupEntry {
    fn parse<'a>(i: &'a [u8]) -> IResult<&'a [u8], Self> {
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

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct AnimationGroup {
    pub name: Box<str>,
    pub animations: Box<[AnimationGroupEntry]>,
}

impl AnimationGroup {
    fn parse<'a>(i: &'a [u8]) -> IResult<&'a [u8], Self> {
        map(
            pair(sized_string, parse_objects_u32(AnimationGroupEntry::parse)),
            ttr_closure! {
                AnimationGroup {
                    name, animations
                }
            },
        )(i)
    }
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct JmxRes {
    pub header: JmxResHeader,
    pub bounding_box: BoundingBox,
    pub material_sets: Box<[MaterialDescriptor]>,
    pub mesh_paths: Box<[(Box<Path>, Option<u32>)]>,
    pub animation: Animation,
    pub skeleton_paths: Box<[(Box<Path>, Box<[u8]>)]>,
    pub mesh_groups: Box<[MeshGroup]>,
    pub animation_groups: Box<[AnimationGroup]>,
}

impl JmxRes {
    pub fn parse<'i>(i: &'i [u8]) -> IResult<&'i [u8], Self> {
        let (_, header) = nom::error::context("resource header", JmxResHeader::parse)(i)?;
        let (_, bounding_box) = BoundingBox::parse(&i[header.collision_offset as usize..])?;
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

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct JmxResHeader {
    pub material_offset: u32,
    pub mesh_offset: u32,
    pub skeleton_offset: u32,
    pub animation_offset: u32,
    pub mesh_group_offset: u32,
    pub animation_group_offset: u32,
    // FIXME effects
    pub mod_palette_offset: u32,
    pub collision_offset: u32,
    pub unk0: u32,
    pub unk1: u32,
    pub unk2: u32,
    pub unk3: u32,
    pub unk4: u32,
    pub res_type: ResourceType,
    pub name: Box<str>,
    //pub unk5: [u8; 48],
}

impl JmxResHeader {
    pub fn parse<'a>(i: &'a [u8]) -> IResult<&'a [u8], Self> {
        preceded(
            // FIXME:  107 and 108 have differences from 109
            tag("JMXVRES 0109"),
            map(
                tuple((
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
                ttr_closure! {
                    JmxResHeader {
                        material_offset,
                        mesh_offset,
                        skeleton_offset,
                        animation_offset,
                        mesh_group_offset,
                        animation_group_offset,
                        mod_palette_offset,
                        collision_offset,
                        unk0,
                        unk1,
                        unk2,
                        unk3,
                        unk4,
                        res_type,
                        name
                    }
                },
            ),
        )(i)
    }
}
