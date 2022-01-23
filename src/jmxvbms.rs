use bitflags::bitflags;
use mint::{Vector2, Vector3};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::{cond, flat_map, map};
use nom::number::complete::{le_f32, le_i32, le_u16, le_u32, le_u8};
use nom::sequence::{pair, preceded, tuple};
use nom::IResult;

#[cfg(feature = "serde")]
use serde_derive::Serialize;

use crate::parser_ext::flags::flags_u32;
use crate::parser_ext::multi::{count, parse_objects_u32};
use crate::parser_ext::number::{vector2_f32, vector3_f32, vector6_f32};
use crate::parser_ext::string::sized_string;
use crate::ttr_closure;

bitflags! {
    #[cfg_attr(feature = "serde", derive(Serialize))]
    #[cfg_attr(feature = "serde", serde(transparent))]
    pub struct VertexFlags: u32 {
        const HAS_LIGHT_MAP = 0x400;
        const UNKNOWN =       0x800;
        const UNKNOWN2 =      0x1000;
    }
}

bitflags! {
    #[cfg_attr(feature = "serde", derive(Serialize))]
    #[cfg_attr(feature = "serde", serde(transparent))]
    pub struct NavFlags: u32 {
        const UNK0 = 0x1;
        const UNK1 = 0x2;
        const UNK2 = 0x4;
        const UNK3 = 0x8;
    }
}

#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct Vertex {
    pub position: Vector3<f32>,
    pub normal: Vector3<f32>,
    pub uv0: Vector2<f32>,
    pub uv1: Option<Vector2<f32>>,
    pub float0: f32,
    pub int0: i32,
    pub int1: i32,
}

impl Vertex {
    fn parser<'a>(light_map: bool) -> impl FnMut(&'a [u8]) -> IResult<&'a [u8], Self> {
        map(
            tuple((
                vector3_f32,
                vector3_f32,
                vector2_f32,
                cond(light_map, vector2_f32),
                le_f32,
                le_i32,
                le_i32,
            )),
            ttr_closure! {
                Vertex {
                    position,
                    normal,
                    uv0,
                    uv1,
                    float0,
                    int0,
                    int1,
                }
            },
        )
    }
}

#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct Unknown(pub f32, pub u32);

#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct ClothEdge {
    pub vertex_index0: u32,
    pub vertex_index1: u32,
    pub max_distance: f32,
}

impl ClothEdge {
    fn parse<'a>(i: &'a [u8]) -> IResult<&'a [u8], Self> {
        map(
            tuple((le_u32, le_u32, le_f32)),
            ttr_closure! {
                ClothEdge { vertex_index0, vertex_index1, max_distance }
            },
        )(i)
    }
}

#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct ClothSimParams {
    pub unk0: u32,
    pub unk1: f32,
    pub unk2: f32,
    pub unk3: f32,
    pub unk4: f32,
    pub unk5: f32,
    pub unk6: f32,
    pub unk7: f32,
    pub unk8: u32,
}

impl ClothSimParams {
    fn parse<'a>(i: &'a [u8]) -> IResult<&'a [u8], Self> {
        map(
            tuple((
                le_u32, le_f32, le_f32, le_f32, le_f32, le_f32, le_f32, le_f32, le_u32,
            )),
            ttr_closure! {
                ClothSimParams {
                    unk0,
                    unk1,
                    unk2,
                    unk3,
                    unk4,
                    unk5,
                    unk6,
                    unk7,
                    unk8,
                }
            },
        )(i)
    }
}

fn parse_cloth_edges<'a>(
    i: &'a [u8],
) -> IResult<&'a [u8], Option<(Box<[ClothEdge]>, Box<[u32]>, ClothSimParams)>> {
    flat_map(le_u32, move |c| {
        cond(
            c != 0,
            tuple((
                count(ClothEdge::parse, c as usize),
                count(le_u32, c as usize),
                ClothSimParams::parse,
            )),
        )
    })(i)
}

#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct ClothVertex {
    pub max_distance: f32,
    pub is_pinned: bool,
}

impl ClothVertex {
    fn parse<'a>(i: &'a [u8]) -> IResult<&'a [u8], Self> {
        map(
            tuple((le_f32, map(le_u32, |int| int != 0))),
            ttr_closure! { ClothVertex { max_distance, is_pinned }},
        )(i)
    }
}

#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct BoneIndexData {
    pub index0: u8,
    pub weight0: u16,
    pub index1: u8,
    pub weight1: u16,
}

impl BoneIndexData {
    fn parse<'a>(i: &'a [u8]) -> IResult<&'a [u8], Self> {
        map(
            tuple((le_u8, le_u16, le_u8, le_u16)),
            ttr_closure! { BoneIndexData { index0, weight0, index1, weight1 }},
        )(i)
    }
}

fn parse_bones<'a>(
    vertex_count: usize,
) -> impl FnMut(&'a [u8]) -> IResult<&'a [u8], Option<(Box<[Box<str>]>, Box<[BoneIndexData]>)>> {
    flat_map(le_u32, move |bc| {
        cond(
            bc != 0,
            pair(
                count(sized_string, bc as usize),
                count(BoneIndexData::parse, vertex_count),
            ),
        )
    })
}

#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct Face(pub [u16; 3]);

impl Face {
    fn parse<'a>(i: &'a [u8]) -> IResult<&'a [u8], Self> {
        map(tuple((le_u16, le_u16, le_u16)), |data| {
            Face([data.0, data.1, data.2])
        })(i)
    }
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct Gate {
    pub name: Box<str>,
    pub vertices: Box<[Vector3<f32>]>,
    pub faces: Box<[Face]>,
}

impl Gate {
    fn parse<'a>(i: &'a [u8]) -> IResult<&'a [u8], Self> {
        map(
            tuple((
                sized_string,
                parse_objects_u32(vector3_f32),
                parse_objects_u32(Face::parse),
            )),
            ttr_closure! {
                Gate {
                    name, vertices, faces
                }
            },
        )(i)
    }
}

#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct ObjectLines {
    pub vertex_source: u16,
    pub vertex_destination: u16,
    //Index of neighbour triangle A --> ObjectGround
    pub cell_source: u16,
    //Index of neighbour triangle B --> ObjectGround --> FFFF --> no neighbour triangle
    pub cell_destination: u16,
    pub collision_flag: u8,
    pub unk: Option<u8>,
}

impl ObjectLines {
    fn parser<'a>(nav_flag: NavFlags) -> impl FnMut(&'a [u8]) -> IResult<&'a [u8], Self> {
        map(
            tuple((
                le_u16,
                le_u16,
                le_u16,
                le_u16,
                le_u8,
                cond(nav_flag.contains(NavFlags::UNK0), le_u8),
            )),
            ttr_closure! {
                ObjectLines {
                    vertex_source,
                    vertex_destination,
                    cell_source,
                    cell_destination,
                    collision_flag,
                    unk,
                }
            },
        )
    }
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct NavMesh {
    pub vertices: Box<[(Vector3<f32>, u8)]>,
    pub ground: Box<[(Face, u16, Option<u8>)]>,
    pub outlines: Box<[ObjectLines]>,
    pub inlines: Box<[ObjectLines]>,
    pub event: Box<[Box<str>]>,
    pub unk0: f32,
    pub unk1: f32,
    pub unk2: u32,
    pub unk3: u32,
    pub unk4: Box<[Box<[u16]>]>,
}

impl NavMesh {
    fn parser<'a>(nav_flag: NavFlags) -> impl FnMut(&'a [u8]) -> IResult<&'a [u8], Self> {
        map(
            tuple((
                parse_objects_u32(pair(vector3_f32, le_u8)),
                parse_objects_u32(tuple((
                    Face::parse,
                    le_u16,
                    cond(nav_flag.contains(NavFlags::UNK1), le_u8),
                ))),
                parse_objects_u32(ObjectLines::parser(nav_flag)),
                parse_objects_u32(ObjectLines::parser(nav_flag)),
                map(
                    cond(
                        nav_flag.contains(NavFlags::UNK2),
                        parse_objects_u32(sized_string),
                    ),
                    Option::unwrap_or_default,
                ),
                le_f32,
                le_f32,
                le_u32,
                le_u32,
                parse_objects_u32(parse_objects_u32(le_u16)),
            )),
            ttr_closure! {
                NavMesh {
                    vertices,
                    ground,
                    outlines,
                    inlines,
                    event,
                    unk0,
                    unk1,
                    unk2,
                    unk3,
                    unk4,
                }
            },
        )
    }
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct JmxBMesh {
    pub header: JmxBMeshHeader,
    pub vertices: Box<[Vertex]>,
    pub light_map_path: Option<Box<str>>,
    pub bone_data: Option<(Box<[Box<str>]>, Box<[BoneIndexData]>)>,
    pub faces: Box<[Face]>,
    pub cloth_vertex: Box<[ClothVertex]>,
    pub cloth_edges: Option<(Box<[ClothEdge]>, Box<[u32]>, ClothSimParams)>,
    pub bounding_box: [f32; 6],
    pub gates: Box<[Gate]>,
    pub nav_mesh: Option<NavMesh>,
}

impl JmxBMesh {
    pub fn parse<'i>(i: &'i [u8]) -> IResult<&'i [u8], Self> {
        let (_, header) = JmxBMeshHeader::parse(i)?;
        let has_light_map = header.vertex_flags.contains(VertexFlags::HAS_LIGHT_MAP);
        let (_, (vertices, light_map_path)) = pair(
            parse_objects_u32(Vertex::parser(has_light_map)),
            cond(has_light_map, sized_string),
        )(&i[header.vertex as usize..])?;
        let (_, bone_data) = parse_bones(vertices.len())(&i[header.skin as usize..])?;
        let (_, faces) = parse_objects_u32(Face::parse)(&i[header.face as usize..])?;
        let (_, cloth_vertex) =
            parse_objects_u32(ClothVertex::parse)(&i[header.cloth_vertex as usize..])?;
        let (_, cloth_edges) = parse_cloth_edges(&i[header.cloth_edge as usize..])?;
        let (_, bounding_box) = vector6_f32(&i[header.bounding_box as usize..])?;
        let (_, gates) = parse_objects_u32(Gate::parse)(&i[header.gate as usize..])?;
        let (_, nav_mesh) = cond(header.nav_mesh != 0, NavMesh::parser(header.nav_flags))(
            &i[header.nav_mesh as usize..],
        )?;

        Ok((
            &[],
            JmxBMesh {
                header,
                vertices,
                light_map_path,
                bone_data,
                faces,
                cloth_vertex,
                cloth_edges,
                bounding_box,
                gates,
                nav_mesh,
            },
        ))
    }
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct JmxBMeshHeader {
    pub vertex: u32,
    pub skin: u32,
    pub face: u32,
    pub cloth_vertex: u32,
    pub cloth_edge: u32,
    pub bounding_box: u32,
    pub gate: u32,
    pub nav_mesh: u32,
    pub unk0: u32,
    pub unk1: u32,
    pub unk3: u32,
    pub nav_flags: NavFlags,
    pub sub_prim_count: u32,
    pub vertex_flags: VertexFlags,
    pub unk4: u32,
    pub name: Box<str>,
    pub material: Box<str>,
    pub unk5: u32,
}

impl JmxBMeshHeader {
    pub fn parse(i: &[u8]) -> IResult<&[u8], Self> {
        preceded(
            alt((tag(b"JMXVBMS 0109"), tag(b"JMXVBMS 0110"))),
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
                    flags_u32(NavFlags::from_bits),
                    le_u32,
                    flags_u32(VertexFlags::from_bits),
                    le_u32,
                    sized_string,
                    sized_string,
                    le_u32,
                )),
                ttr_closure! {
                    JmxBMeshHeader {
                        vertex,
                        skin,
                        face,
                        cloth_vertex,
                        cloth_edge,
                        bounding_box,
                        gate,
                        nav_mesh,
                        unk0,
                        unk1,
                        unk3,
                        nav_flags,
                        sub_prim_count,
                        vertex_flags,
                        unk4,
                        name,
                        material,
                        unk5
                    }
                },
            ),
        )(i)
    }
}
