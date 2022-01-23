use bitflags::bitflags;
use mint::{Vector2, Vector3};
use nom::bytes::complete::tag;
use nom::combinator::{flat_map, map};
use nom::number::complete::{le_f32, le_u16, le_u32, le_u8};
use nom::sequence::{pair, preceded, tuple};
use nom::IResult;

use crate::parser_ext::flags::flags_u16;
use crate::parser_ext::multi::{count, parse_objects_u16, parse_objects_u32, parse_objects_u8};
use crate::parser_ext::number::{vector2_f32, vector3_f32};
use crate::ttr_closure;

#[cfg(feature = "serde")]
use serde_derive::Serialize;

bitflags! {
    #[cfg_attr(feature = "serde", derive(Serialize))]
    #[cfg_attr(feature = "serde", serde(transparent))]
    pub struct CollisionFlag: u16 {
        const HAS_COLLISION = 0xFFFF;
    }
}

bitflags! {
    #[cfg_attr(feature = "serde", derive(Serialize))]
    #[cfg_attr(feature = "serde", serde(transparent))]
    pub struct EventZoneFlag: u16 {
        const UNK0 = 0x1;
        const HAS_COLLISION = 0x100;
    }
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct NavEntry {
    pub id: u32,
    pub position: Vector3<f32>,
    pub collision_flag: CollisionFlag,
    pub yaw: f32,
    pub unique_id: u16,
    pub scale: u16,
    pub event_zone_flag: EventZoneFlag,
    pub region_id: u16,
    pub mount_points: Box<[(u8, u8, u8, u8, u8, u8)]>,
}

impl NavEntry {
    fn parse<'a>(i: &'a [u8]) -> IResult<&'a [u8], Self> {
        map(
            tuple((
                le_u32,
                vector3_f32,
                flags_u16(CollisionFlag::from_bits),
                le_f32,
                le_u16,
                le_u16,
                flags_u16(EventZoneFlag::from_bits),
                le_u16,
                parse_objects_u16(tuple((le_u8, le_u8, le_u8, le_u8, le_u8, le_u8))),
            )),
            ttr_closure! {
                NavEntry {
                    id,
                    position,
                    collision_flag,
                    yaw,
                    unique_id,
                    scale,
                    event_zone_flag,
                    region_id,
                    mount_points
                }
            },
        )(i)
    }
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct NavCell {
    pub min: Vector2<f32>,
    pub max: Vector2<f32>,
    pub entries: Box<[u16]>,
}

impl NavCell {
    fn parse<'a>(i: &'a [u8]) -> IResult<&'a [u8], Self> {
        map(
            tuple((vector2_f32, vector2_f32, parse_objects_u8(le_u16))),
            ttr_closure! {
                NavCell {
                    min,
                    max,
                    entries
                }
            },
        )(i)
    }
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct NavRegionLink {
    pub min: Vector2<f32>,
    pub max: Vector2<f32>,
    pub line_flag: u8,
    pub line_source: u8,
    pub line_destination: u8,
    pub cell_source: u16,
    pub cell_destination: u16,
    pub region_source: u16,
    pub region_destination: u16,
}

impl NavRegionLink {
    fn parse<'a>(i: &'a [u8]) -> IResult<&'a [u8], Self> {
        map(
            tuple((
                vector2_f32,
                vector2_f32,
                le_u8,
                le_u8,
                le_u8,
                le_u16,
                le_u16,
                le_u16,
                le_u16,
            )),
            ttr_closure! {
                    NavRegionLink {
                    min,
                    max,
                    line_flag,
                    line_source,
                    line_destination,
                    cell_source,
                    cell_destination,
                    region_source,
                    region_destination
                }
            },
        )(i)
    }
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct NavCellLink {
    pub min: Vector2<f32>,
    pub max: Vector2<f32>,
    pub line_flag: u8,
    pub line_source: u8,
    pub line_destination: u8,
    pub cell_source: u16,
    pub cell_destination: u16,
}

impl NavCellLink {
    fn parse<'a>(i: &'a [u8]) -> IResult<&'a [u8], Self> {
        map(
            tuple((
                vector2_f32,
                vector2_f32,
                le_u8,
                le_u8,
                le_u8,
                le_u16,
                le_u16,
            )),
            ttr_closure! {
                NavCellLink {
                    min,
                    max,
                    line_flag,
                    line_source,
                    line_destination,
                    cell_source,
                    cell_destination
                }
            },
        )(i)
    }
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct JmxNvm {
    pub nav_entries: Box<[NavEntry]>,
    pub nav_extra_count: u32,
    pub nav_cells: Box<[NavCell]>,
    pub nav_region_links: Box<[NavRegionLink]>,
    pub nav_cell_links: Box<[NavCellLink]>,
    pub texture_map: Box<[(u16, u16, u16, u16)]>,
    pub height_map: Box<[f32]>,
}

impl JmxNvm {
    pub fn parse<'i>(i: &'i [u8]) -> IResult<&'i [u8], Self> {
        map(
            preceded(
                tag(b"JMXVNVM 1000"),
                tuple((
                    parse_objects_u16(NavEntry::parse),
                    flat_map(le_u32, |c| pair(le_u32, count(NavCell::parse, c as usize))),
                    parse_objects_u32(NavRegionLink::parse),
                    parse_objects_u32(NavCellLink::parse),
                    count(tuple((le_u16, le_u16, le_u16, le_u16)), 96 * 96),
                    count(le_f32, 97 * 97),
                )),
            ),
            |data| JmxNvm {
                nav_entries: data.0,
                nav_extra_count: (data.1).0,
                nav_cells: (data.1).1,
                nav_region_links: data.2,
                nav_cell_links: data.3,
                texture_map: data.4,
                height_map: data.5,
            },
        )(i)
    }
}
