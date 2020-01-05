use nom::bytes::complete::tag;
use nom::combinator::{cond, flat_map, map};
use nom::error::ParseError;
use nom::multi::count;
use nom::number::complete::{le_f32, le_u32, le_u8};
use nom::sequence::{pair, preceded, tuple};
use nom::IResult;

#[cfg(feature = "serde")]
use serde::Serialize;

use std::path::PathBuf;

use crate::{
    parse_objects_u32, sized_path, sized_string, vector2_f32, vector6_f32, ResourceAnimationType,
    ResourceType, Vector2,
};

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct BoundingBox {
    pub root_mesh: String,
    pub bounding_box0: [f32; 6],
    pub bounding_box1: [f32; 6],
    pub extra_bounding_data: Vec<u8>,
}

impl BoundingBox {
    pub fn parse<'a, E: ParseError<&'a [u8]>>(i: &'a [u8]) -> IResult<&'a [u8], Self, E> {
        map(
            tuple((
                sized_string,
                vector6_f32,
                vector6_f32,
                flat_map(le_u32, |val| cond(val != 0, count(le_u8, 64))),
            )),
            |(root_mesh, bounding_box0, bounding_box1, extra_bounding_data)| BoundingBox {
                root_mesh,
                bounding_box0,
                bounding_box1,
                extra_bounding_data: extra_bounding_data.unwrap_or_default(),
            },
        )(i)
    }
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct MaterialDescriptor {
    pub id: u32,
    pub path: PathBuf,
}

impl MaterialDescriptor {
    pub fn parse<'a, E: ParseError<&'a [u8]>>(i: &'a [u8]) -> IResult<&'a [u8], Self, E> {
        map(pair(le_u32, sized_path), |(id, path)| MaterialDescriptor {
            id,
            path,
        })(i)
    }
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct Animation {
    pub unk0: u32,
    pub unk1: u32,
    pub paths: Vec<PathBuf>,
}

impl Animation {
    pub fn parse<'a, E: ParseError<&'a [u8]>>(i: &'a [u8]) -> IResult<&'a [u8], Self, E> {
        map(
            tuple((le_u32, le_u32, parse_objects_u32(sized_path))),
            |(unk0, unk1, paths)| Animation { unk0, unk1, paths },
        )(i)
    }
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct MeshGroup {
    pub name: String,
    pub file_indices: Vec<u32>,
}

impl MeshGroup {
    pub fn parse<'a, E: ParseError<&'a [u8]>>(i: &'a [u8]) -> IResult<&'a [u8], Self, E> {
        map(
            pair(sized_string, parse_objects_u32(le_u32)),
            |(name, file_indices)| MeshGroup { name, file_indices },
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
    pub fn parse<'a, E: ParseError<&'a [u8]>>(i: &'a [u8]) -> IResult<&'a [u8], Self, E> {
        map(
            tuple((le_u32, le_u32, le_u32, le_u32)),
            |(key_time, typ, unk0, unk1)| AnimationEvent {
                key_time,
                typ,
                unk0,
                unk1,
            },
        )(i)
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
    pub fn parse<'a, E: ParseError<&'a [u8]>>(i: &'a [u8]) -> IResult<&'a [u8], Self, E> {
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
    pub name: String,
    pub animations: Vec<AnimationGroupEntry>,
}

impl AnimationGroup {
    pub fn parse<'a, E: ParseError<&'a [u8]>>(i: &'a [u8]) -> IResult<&'a [u8], Self, E> {
        map(
            pair(sized_string, parse_objects_u32(AnimationGroupEntry::parse)),
            |(name, animations)| AnimationGroup { name, animations },
        )(i)
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

impl JmxRes {
    pub fn parse<'a, E: ParseError<&'a [u8]>>(i: &'a [u8]) -> Result<Self, nom::Err<E>> {
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

        Ok(JmxRes {
            header,
            bounding_box,
            material_sets,
            mesh_paths,
            animation,
            skeleton_paths,
            mesh_groups,
            animation_groups,
        })
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
        map(
            preceded(
                tag("JMXVRES 0109"),
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
            ),
            |data| JmxResHeader {
                material_offset: data.0,
                mesh_offset: data.1,
                skeleton_offset: data.2,
                animation_offset: data.3,
                mesh_group_offset: data.4,
                animation_group_offset: data.5,
                sound_effect_offset: data.6,
                bounding_box_offset: data.7,
                unk0: data.8,
                unk1: data.9,
                unk2: data.10,
                unk3: data.11,
                unk4: data.12,
                res_type: data.13,
                name: data.14,
                //unk5: data.15
            },
        )(i)
    }
}
