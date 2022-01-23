use std::path::Path;

use mint::Vector3;
use nom::bytes::complete::tag;
use nom::combinator::{cond, flat_map, map};
use nom::number::complete::{le_f32, le_u16, le_u32, le_u8};
use nom::sequence::{pair, preceded, tuple};
use nom::IResult;

#[cfg(feature = "serde")]
use serde_derive::Serialize;

use crate::parser_ext::multi::{count, parse_objects_u32};
use crate::parser_ext::number::{vector3_f32, vector6_f32};
use crate::parser_ext::string::{sized_path, sized_string};
use crate::ttr_closure;

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct RoomObjectPoint {
    pub name: Box<str>,
    pub position: Vector3<f32>,
    pub rotation: Vector3<f32>,
    pub size: Vector3<f32>,
    pub rotation2: Vector3<f32>,
    pub unk0: f32,
    pub unk2: f32,
    pub unk1: f32,
}

impl RoomObjectPoint {
    fn parse<'a>(i: &'a [u8]) -> IResult<&'a [u8], Self> {
        map(
            tuple((
                sized_string,
                vector3_f32,
                vector3_f32,
                vector3_f32,
                vector3_f32,
                le_f32,
                le_f32,
                le_f32,
            )),
            ttr_closure! {
                RoomObjectPoint {
                    name,
                    position,
                    rotation,
                    size,
                    rotation2,
                    unk0,
                    unk2,
                    unk1
                }
            },
        )(i)
    }
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct RoomObjectEntry {
    pub name: Box<str>,
    pub path: Box<Path>,
    pub position: Vector3<f32>,
    pub rotation: Vector3<f32>,
    pub scale: Vector3<f32>,
    pub flag: u32, //0 = None, 2 = ColObj, 4 = WaterObj
    pub water_extra: Option<u32>,
    pub id: u32,
    pub unk0: f32,
}

impl RoomObjectEntry {
    fn parse<'a>(i: &'a [u8]) -> IResult<&'a [u8], Self> {
        map(
            tuple((
                sized_string,
                sized_path,
                vector3_f32,
                vector3_f32,
                vector3_f32,
                // FIXME:
                flat_map(le_u32, |f| map(cond(f == 0x04, le_u32), move |w| (f, w))),
                le_u32,
                le_f32, // FIXME: <- this is what should be read for flag 0x04
            )),
            |(name, path, position, rotation, scale, (flag, water_extra), id, unk0)| {
                RoomObjectEntry {
                    name,
                    path,
                    position,
                    rotation,
                    scale,
                    flag,
                    water_extra,
                    id,
                    unk0,
                }
            },
        )(i)
    }
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct RoomObjectExtraA {
    pub unk0: f32,
    pub unk1: f32,
    pub unk2: f32,
    pub unk3: f32,
}

impl RoomObjectExtraA {
    fn parse<'a>(i: &'a [u8]) -> IResult<&'a [u8], Self> {
        map(
            tuple((le_f32, le_f32, le_f32, le_f32)),
            ttr_closure! {
                RoomObjectExtraA {
                    unk0,
                    unk1,
                    unk2,
                    unk3
                }
            },
        )(i)
    }
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct RoomObjectExtraB {
    pub unk0: f32,
    pub unk1: f32,
    pub unk2: f32,
    pub unk3: f32,
    pub unk4: f32,
    pub unk5: f32,
    pub unk6: f32,
}

impl RoomObjectExtraB {
    fn parse<'a>(i: &'a [u8]) -> IResult<&'a [u8], Self> {
        map(
            tuple((le_f32, le_f32, le_f32, le_f32, le_f32, le_f32, le_f32)),
            ttr_closure! {
                RoomObjectExtraB { unk0, unk1, unk2, unk3, unk4, unk5, unk6 }
            },
        )(i)
    }
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct RoomObject {
    pub path: Box<Path>,
    pub name: Box<str>,
    pub unk0: u32,
    pub position: Vector3<f32>,
    pub yaw: f32,
    pub is_entrance: f32,
    pub aabb: [f32; 6],
    pub unk1: u32,
    pub fog_color: f32,
    pub fog_near_plane: f32,
    pub fog_far_plane: f32,
    pub fog_intensity: f32,
    pub extra_a: Option<RoomObjectExtraA>,
    pub extra_b: Option<RoomObjectExtraB>,
    pub unk6: Box<str>,
    pub room_index: u32,
    pub floor_index: u32,
    pub connected_objects: Box<[u32]>,
    pub indirect_connected_objects: Box<[u32]>,
    pub unk7: u32,
    pub entries: Box<[RoomObjectEntry]>,
    pub points: Box<[RoomObjectPoint]>,
}

impl RoomObject {
    fn parse<'a>(i: &'a [u8]) -> IResult<&'a [u8], Self> {
        map(
            tuple((
                sized_path,
                sized_string,
                le_u32,
                vector3_f32,
                le_f32,
                le_f32,
                vector6_f32,
                le_u32,
                le_f32,
                le_f32,
                le_f32,
                le_f32,
                flat_map(le_u8, |val| cond(val == 0x01, RoomObjectExtraA::parse)),
                flat_map(le_u8, |val| cond(val == 0x02, RoomObjectExtraB::parse)),
                sized_string,
                le_u32,
                le_u32,
                parse_objects_u32(le_u32),
                parse_objects_u32(le_u32),
                flat_map(le_u32, |c| {
                    pair(le_u32, count(RoomObjectEntry::parse, c as usize))
                }),
                parse_objects_u32(RoomObjectPoint::parse),
            )),
            |(
                path,
                name,
                unk0,
                position,
                yaw,
                pitch,
                aabb,
                unk1,
                fog_color,
                fog_near_plane,
                fog_far_plane,
                fog_intensity,
                extra_a,
                extra_b,
                unk6,
                room_index,
                floor_index,
                connected_objects,
                indirect_connected_objects,
                (unk7, entries),
                points,
            )| RoomObject {
                path,
                name,
                unk0,
                position,
                yaw,
                is_entrance: pitch,
                aabb,
                unk1,
                fog_color,
                fog_near_plane,
                fog_far_plane,
                fog_intensity,
                extra_a,
                extra_b,
                unk6,
                room_index,
                floor_index,
                connected_objects,
                indirect_connected_objects,
                unk7,
                entries,
                points,
            },
        )(i)
    }
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct ObjectGroup {
    pub name: Box<str>,
    pub flag: u32,
    pub object_indices: Box<[u32]>,
}

impl ObjectGroup {
    fn parse<'a>(i: &'a [u8]) -> IResult<&'a [u8], Self> {
        map(
            tuple((sized_string, le_u32, parse_objects_u32(le_u32))),
            ttr_closure! {
                ObjectGroup {
                    name, flag, object_indices
                }
            },
        )(i)
    }
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct Link {
    pub id: u32,
    pub connections: Box<[u32]>,
}

impl Link {
    fn parse<'a>(i: &'a [u8]) -> IResult<&'a [u8], Self> {
        map(
            pair(le_u32, parse_objects_u32(le_u32)),
            |(id, connections)| Link { id, connections },
        )(i)
    }
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct Links {
    pub unk0: u32,
    pub unk1: u32,
    pub unk2: u32,
    pub links: Box<[Link]>,
}

impl Links {
    fn parse<'a>(i: &'a [u8]) -> IResult<&'a [u8], Self> {
        map(
            tuple((le_u32, le_u32, le_u32, parse_objects_u32(Link::parse))),
            ttr_closure! {
                Links {
                    unk0, unk1, unk2, links
                }
            },
        )(i)
    }
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct JmxDungeon {
    pub header: JmxDungeonHeader,
    pub aabb: [f32; 6],
    pub oobb: [f32; 6],
    pub room_objects: Box<[RoomObject]>,
    pub links: Links,
    pub object_connections: Box<[Box<[u32]>]>,
    pub room_names: Box<[Box<str>]>,
    pub floor_names: Box<[Box<str>]>,
    pub object_groups: Box<[ObjectGroup]>,
}

impl JmxDungeon {
    pub fn parse<'i>(i: &'i [u8]) -> IResult<&'i [u8], Self> {
        let (_, header) = JmxDungeonHeader::parse(i)?;
        let (_, (aabb, oobb)) =
            pair(vector6_f32, vector6_f32)(&i[header.bounding_boxes as usize..])?;
        let (_, room_objects) =
            parse_objects_u32(RoomObject::parse)(&i[header.room_objects as usize..])?;
        let (_, links) = Links::parse(&i[header.links as usize..])?;
        let (_, object_connections) =
            parse_objects_u32(parse_objects_u32(le_u32))(&i[header.object_connections as usize..])?;
        let (_, (room_names, floor_names)) = pair(
            parse_objects_u32(sized_string),
            parse_objects_u32(sized_string),
        )(&i[header.index_names as usize..])?;
        let (_, object_groups) =
            parse_objects_u32(ObjectGroup::parse)(&i[header.object_groups as usize..])?;

        Ok((
            &[],
            JmxDungeon {
                header,
                aabb,
                oobb,
                room_objects,
                links,
                object_connections,
                room_names,
                floor_names,
                object_groups,
            },
        ))
    }
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct JmxDungeonHeader {
    pub room_objects: u32,
    pub object_connections: u32,
    pub links: u32,
    pub object_groups: u32,
    pub index_names: u32,
    pub unk0: u32,
    pub unk1: u32,
    pub bounding_boxes: u32,
    pub ty: u32,
    pub dungeon_name: Box<str>,
    pub unk4: u32,
    pub unk5: u32,
    pub region_id: u16,
}

impl JmxDungeonHeader {
    fn parse<'a>(i: &'a [u8]) -> IResult<&'a [u8], Self> {
        preceded(
            tag(b"JMXVDOF 0101"),
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
                    sized_string,
                    le_u32,
                    le_u32,
                    le_u16,
                )),
                ttr_closure! {
                    JmxDungeonHeader {
                        room_objects,
                        object_connections,
                        links,
                        object_groups,
                        index_names,
                        unk0,
                        unk1,
                        bounding_boxes,
                        ty,
                        dungeon_name,
                        unk4,
                        unk5,
                        region_id
                    }
                },
            ),
        )(i)
    }
}
