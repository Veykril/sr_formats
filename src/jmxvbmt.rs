use std::path::Path;

use bitflags::bitflags;
use mint::Vector4;
use nom::bytes::complete::tag;
use nom::combinator::map;
use nom::number::complete::{le_f32, le_u16, le_u32, le_u8};
use nom::sequence::{pair, preceded, tuple};
use nom::IResult;

#[cfg(feature = "serde")]
use serde::Serialize;

use crate::parser_ext::flags::flags_u32;
use crate::parser_ext::multi::parse_objects_u32;
use crate::parser_ext::number::vector4_f32;
use crate::parser_ext::string::{sized_path, sized_string};
use crate::ttr_closure;

bitflags! {
    #[cfg_attr(feature = "serde", derive(Serialize))]
    #[cfg_attr(feature = "serde", serde(transparent))]
    pub struct MaterialFlags: u32 {
        const UNK0 = 0x1;
        const UNK1 = 0x2;
        const UNK2 = 0x4;
        const UNK3 = 0x8;
        const UNK4 = 0x10;
        const UNK5 = 0x20;
        const UNK6 = 0x40;
        const UNK7 = 0x80;
        const UNK8 = 0x100;
        const UNK9 = 0x200;
        const UNK10 = 0x400;
        const UNK11 = 0x800;
        const UNK12 = 0x1000;
        const HAS_NORMAL_MAP = 0x2000;
    }
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct Material {
    pub name: Box<str>,
    pub diffuse: Vector4<f32>,
    pub ambient: Vector4<f32>,
    pub specular: Vector4<f32>,
    pub emissive: Vector4<f32>,
    pub specular_power: f32,
    pub material_flags: MaterialFlags,
    pub diffuse_map: Box<Path>,
    pub unk0: f32,
    pub unk1: u16,
    pub absolute_diffuse_map_path: bool,
    #[cfg_attr(feature = "serde", serde(skip))]
    pub normal_map: Option<(Box<Path>, u32)>,
}

impl Material {
    fn parse<'a>(i: &'a [u8]) -> IResult<&'a [u8], Self> {
        let (i, mut mat) = map(
            tuple((
                sized_string,
                vector4_f32,
                vector4_f32,
                vector4_f32,
                vector4_f32,
                le_f32,
                flags_u32(MaterialFlags::from_bits),
                sized_path,
                le_f32,
                le_u16,
                map(le_u8, |b| b != 0),
                |i| IResult::Ok((i, None)),
            )),
            ttr_closure! {
                Material {
                    name,
                    diffuse,
                    ambient,
                    specular,
                    emissive,
                    specular_power,
                    material_flags,
                    diffuse_map,
                    unk0,
                    unk1,
                    absolute_diffuse_map_path,
                    normal_map
                }
            },
        )(i)?;
        if mat.material_flags.contains(MaterialFlags::HAS_NORMAL_MAP) {
            let (i, normal_map) = pair(sized_path, le_u32)(i)?;
            mat.normal_map = Some(normal_map);
            Ok((i, mat))
        } else {
            Ok((i, mat))
        }
    }
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct JmxMat(pub Box<[Material]>);

impl JmxMat {
    pub fn parse<'i>(i: &'i [u8]) -> IResult<&'i [u8], Self> {
        map(
            preceded(tag(b"JMXVBMT 0102"), parse_objects_u32(Material::parse)),
            JmxMat,
        )(i)
    }
}
