use bitflags::bitflags;
use nalgebra::Vector4;
use nom::{
    bytes::complete::tag,
    combinator::{cond, map},
    number::complete::{le_f32, le_u16, le_u32, le_u8},
    sequence::{pair, preceded, tuple},
    IResult,
};
use serde::Serialize;

use crate::{parse_objects_u32, sized_string, vector4_f32};

bitflags! {
    #[derive(Serialize)]
    #[serde(transparent)]
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

#[derive(Debug, Serialize)]
pub struct Material {
    pub name: String,
    pub diffuse: Vector4<f32>,
    pub ambient: Vector4<f32>,
    pub specular: Vector4<f32>,
    pub emissive: Vector4<f32>,
    pub specular_power: f32,
    pub material_flags: MaterialFlags,
    pub diffuse_map: String,
    pub unk0: f32,
    pub unk1: u16,
    pub same_dir_as_set: bool,
    #[serde(skip)]
    pub normal_map: Option<(String, u32)>,
}

impl Material {
    pub fn parser<'a>() -> impl Fn(&'a [u8]) -> IResult<&'a [u8], Self> {
        |i| {
            map(
                tuple((
                    sized_string,
                    vector4_f32,
                    vector4_f32,
                    vector4_f32,
                    vector4_f32,
                    le_f32,
                    map(le_u32, |flags| {
                        MaterialFlags::from_bits(flags).unwrap_or_else(|| {
                            panic!("Unknown MaterialFlags encountered 0b{:b}", flags)
                        })
                    }),
                    sized_string,
                    le_f32,
                    le_u16,
                    map(le_u8, |b| b != 0),
                )),
                |data| Material {
                    name: data.0,
                    diffuse: data.1,
                    ambient: data.2,
                    specular: data.3,
                    emissive: data.4,
                    specular_power: data.5,
                    material_flags: data.6,
                    diffuse_map: data.7,
                    unk0: data.8,
                    unk1: data.9,
                    same_dir_as_set: data.10,
                    normal_map: None,
                },
            )(i)
            .and_then(|(i, mut mat)| {
                cond(
                    mat.material_flags.contains(MaterialFlags::HAS_NORMAL_MAP),
                    pair(sized_string, le_u32),
                )(i)
                .map(|(i, normal_map)| {
                    mat.normal_map = normal_map;
                    (i, mat)
                })
            })
        }
    }
}

#[derive(Debug, Serialize)]
pub struct JmxMat(pub Vec<Material>);

impl JmxMat {
    pub fn parse(i: &[u8]) -> Result<Self, nom::Err<(&[u8], nom::error::ErrorKind)>> {
        map(
            preceded(tag(b"JMXVBMT 0102"), parse_objects_u32(Material::parser())),
            JmxMat,
        )(i)
        .map(|r| r.1)
    }
}
