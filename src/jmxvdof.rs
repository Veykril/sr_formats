use nom::bytes::complete::tag;
use nom::combinator::{cond, flat_map, map};
use nom::error::ParseError;
use nom::multi::count;
use nom::number::complete::{le_f32, le_u16, le_u32, le_u8};
use nom::sequence::{pair, preceded, tuple};
use nom::IResult;

#[cfg(feature = "serde")]
use serde::Serialize;

use std::path::PathBuf;

use crate::{parse_objects_u32, small_sized_string, vector3_f32, vector6_f32, Vector3};

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct RoomObjectPoint {
    pub name: String,
    pub position: Vector3<f32>,
    pub rotation: Vector3<f32>,
    pub size: Vector3<f32>,
    pub rotation2: Vector3<f32>,
    pub unk0: f32,
    pub unk2: f32,
    pub unk1: f32,
}

impl RoomObjectPoint {
    pub fn parse<'a, E: ParseError<&'a [u8]>>(i: &'a [u8]) -> IResult<&'a [u8], Self, E> {
        map(
            tuple((
                small_sized_string,
                vector3_f32,
                vector3_f32,
                vector3_f32,
                vector3_f32,
                le_f32,
                le_f32,
                le_f32,
            )),
            |(name, position, rotation, size, rotation2, unk0, unk2, unk1)| RoomObjectPoint {
                name,
                position,
                rotation,
                size,
                rotation2,
                unk0,
                unk2,
                unk1,
            },
        )(i)
    }
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct RoomObjectEntry {
    pub name: String,
    pub path: PathBuf,
    pub position: Vector3<f32>,
    pub rotation: Vector3<f32>,
    pub scale: Vector3<f32>,
    pub flag: u32,
    pub water_extra: Option<u32>,
    pub id: u32,
    pub unk0: f32,
}

impl RoomObjectEntry {
    pub fn parse<'a, E: ParseError<&'a [u8]>>(i: &'a [u8]) -> IResult<&'a [u8], Self, E> {
        map(
            tuple((
                small_sized_string,
                map(small_sized_string, From::from),
                vector3_f32,
                vector3_f32,
                vector3_f32,
                flat_map(le_u32, |f| map(cond(f == 0x04, le_u32), move |w| (f, w))),
                le_u32,
                le_f32,
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
    pub fn parse<'a, E: ParseError<&'a [u8]>>(i: &'a [u8]) -> IResult<&'a [u8], Self, E> {
        map(
            tuple((le_f32, le_f32, le_f32, le_f32)),
            |(unk0, unk1, unk2, unk3)| RoomObjectExtraA {
                unk0,
                unk1,
                unk2,
                unk3,
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
    pub fn parse<'a, E: ParseError<&'a [u8]>>(i: &'a [u8]) -> IResult<&'a [u8], Self, E> {
        map(
            tuple((le_f32, le_f32, le_f32, le_f32, le_f32, le_f32, le_f32)),
            |(unk0, unk1, unk2, unk3, unk4, unk5, unk6)| RoomObjectExtraB {
                unk0,
                unk1,
                unk2,
                unk3,
                unk4,
                unk5,
                unk6,
            },
        )(i)
    }
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct RoomObject {
    pub path: PathBuf,
    pub name: PathBuf,
    // always 0.0?
    pub unk0: f32,
    pub position: Vector3<f32>,
    pub yaw: f32,
    pub pitch: f32,
    pub aabb: [f32; 6],
    pub unk1: f32,
    pub unk2: f32,
    pub unk3: f32,
    pub unk4: f32,
    pub unk5: f32,
    pub extra_a: Option<RoomObjectExtraA>,
    pub extra_b: Option<RoomObjectExtraB>,
    pub unk6: u32,
    pub room_index: u32,
    pub floor_index: u32,
    pub connected_objects: Vec<u32>,
    pub indirect_connected_objects: Vec<u32>,
    pub unk7: u32,
    pub entries: Vec<RoomObjectEntry>,
    pub points: Vec<RoomObjectPoint>,
}

impl RoomObject {
    pub fn parse<'a, E: ParseError<&'a [u8]>>(i: &'a [u8]) -> IResult<&'a [u8], Self, E> {
        map(
            tuple((
                map(small_sized_string, From::from),
                map(small_sized_string, From::from),
                le_f32,
                vector3_f32,
                le_f32,
                le_f32,
                vector6_f32,
                le_f32,
                le_f32,
                le_f32,
                le_f32,
                le_f32,
                flat_map(le_u8, |val| cond(val == 0x01, RoomObjectExtraA::parse)),
                flat_map(le_u8, |val| cond(val == 0x02, RoomObjectExtraB::parse)),
                le_u32,
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
                unk2,
                unk3,
                unk4,
                unk5,
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
                pitch,
                aabb,
                unk1,
                unk2,
                unk3,
                unk4,
                unk5,
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
    pub name: String,
    pub flag: u32,
    pub object_indices: Vec<u32>,
}

impl ObjectGroup {
    pub fn parse<'a, E: ParseError<&'a [u8]>>(i: &'a [u8]) -> IResult<&'a [u8], Self, E> {
        map(
            tuple((small_sized_string, le_u32, parse_objects_u32(le_u32))),
            |(name, flag, object_indices)| ObjectGroup {
                name,
                flag,
                object_indices,
            },
        )(i)
    }
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct Link {
    pub id: u32,
    pub connections: Vec<u32>,
}

impl Link {
    pub fn parse<'a, E: ParseError<&'a [u8]>>(i: &'a [u8]) -> IResult<&'a [u8], Self, E> {
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
    pub links: Vec<Link>,
}

impl Links {
    pub fn parse<'a, E: ParseError<&'a [u8]>>(i: &'a [u8]) -> IResult<&'a [u8], Self, E> {
        map(
            tuple((le_u32, le_u32, le_u32, parse_objects_u32(Link::parse))),
            |(unk0, unk1, unk2, links)| Links {
                unk0,
                unk1,
                unk2,
                links,
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
    pub room_objects: Vec<RoomObject>,
    pub links: Links,
    pub object_connections: Vec<Vec<u32>>,
    pub room_names: Vec<String>,
    pub floor_names: Vec<String>,
    pub object_groups: Vec<ObjectGroup>,
}

impl JmxDungeon {
    pub fn parse<'a, E: ParseError<&'a [u8]>>(i: &'a [u8]) -> Result<Self, nom::Err<E>> {
        let (_, header) = JmxDungeonHeader::parse(i)?;
        let (_, (aabb, oobb)) =
            pair(vector6_f32, vector6_f32)(&i[header.bounding_boxes as usize..])?;
        let (_, room_objects) =
            parse_objects_u32(RoomObject::parse)(&i[header.room_objects as usize..])?;
        let (_, links) = Links::parse(&i[header.links as usize..])?;
        let (_, object_connections) =
            parse_objects_u32(parse_objects_u32(le_u32))(&i[header.object_connections as usize..])?;
        let (_, (room_names, floor_names)) = pair(
            parse_objects_u32(small_sized_string),
            parse_objects_u32(small_sized_string),
        )(&i[header.index_names as usize..])?;
        let (_, object_groups) =
            parse_objects_u32(ObjectGroup::parse)(&i[header.object_groups as usize..])?;

        Ok(JmxDungeon {
            header,
            aabb,
            oobb,
            room_objects,
            links,
            object_connections,
            room_names,
            floor_names,
            object_groups,
        })
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
    pub unk2: u16,
    pub unk3: u16,
    pub dungeon_name: String,
    pub unk4: u32,
    pub unk5: u32,
    pub region_id: u16,
}

impl JmxDungeonHeader {
    pub fn parse<'a, E: ParseError<&'a [u8]>>(i: &'a [u8]) -> IResult<&'a [u8], Self, E> {
        map(
            preceded(
                tag(b"JMXVDOF 0101"),
                tuple((
                    le_u32,
                    le_u32,
                    le_u32,
                    le_u32,
                    le_u32,
                    le_u32,
                    le_u32,
                    le_u32,
                    le_u16,
                    le_u16,
                    small_sized_string,
                    le_u32,
                    le_u32,
                    le_u16,
                )),
            ),
            |(
                room_objects,
                object_connections,
                links,
                object_groups,
                index_names,
                unk0,
                unk1,
                bounding_boxes,
                unk2,
                unk3,
                dungeon_name,
                unk4,
                unk5,
                region_id,
            )| JmxDungeonHeader {
                room_objects,
                object_connections,
                links,
                object_groups,
                index_names,
                unk0,
                unk1,
                bounding_boxes,
                unk2,
                unk3,
                dungeon_name,
                unk4,
                unk5,
                region_id,
            },
        )(i)
    }
}
