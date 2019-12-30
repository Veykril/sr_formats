use bitflags::bitflags;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::{cond, flat_map, map};
use nom::error::ParseError;
use nom::multi::count;
use nom::number::complete::{le_f32, le_i32, le_u16, le_u32, le_u8};
use nom::sequence::{pair, preceded, tuple};
use nom::IResult;

#[cfg(feature = "serde")]
use serde::Serialize;

use crate::{parse_objects_u32, sized_string, vector2_f32, vector3_f32, Vector2, Vector3};

bitflags! {
    #[cfg_attr(feature = "serde", derive(Serialize))]
    #[cfg_attr(feature = "serde", serde(transparent))]
    pub struct VertexFlags: u32 {
        const HAS_LIGHT_MAP = 0x400;
        const UNKNOWN = 0x800;
        const UNKNOWN2 = 0x1000;
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

#[derive(Debug)]
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
    fn parser<'a, E: ParseError<&'a [u8]>>(
        light_map: bool,
    ) -> impl Fn(&'a [u8]) -> IResult<&'a [u8], Self, E> {
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
            |data| Vertex {
                position: data.0,
                normal: data.1,
                uv0: data.2,
                uv1: data.3,
                float0: data.4,
                int0: data.5,
                int1: data.6,
            },
        )
    }
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct Unknown(pub f32, pub u32);

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct ClothEdge {
    pub vertex_index0: u32,
    pub vertex_index1: u32,
    pub max_distance: f32,
}

impl ClothEdge {
    pub fn parser<'a, E: ParseError<&'a [u8]>>() -> impl Fn(&'a [u8]) -> IResult<&'a [u8], Self, E>
    {
        map(tuple((le_u32, le_u32, le_f32)), |data| ClothEdge {
            vertex_index0: data.0,
            vertex_index1: data.1,
            max_distance: data.2,
        })
    }
}

#[derive(Debug)]
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
    pub fn parser<'a, E: ParseError<&'a [u8]>>() -> impl Fn(&'a [u8]) -> IResult<&'a [u8], Self, E>
    {
        map(
            tuple((
                le_u32, le_f32, le_f32, le_f32, le_f32, le_f32, le_f32, le_f32, le_u32,
            )),
            |data| ClothSimParams {
                unk0: data.0,
                unk1: data.1,
                unk2: data.2,
                unk3: data.3,
                unk4: data.4,
                unk5: data.5,
                unk6: data.6,
                unk7: data.7,
                unk8: data.8,
            },
        )
    }
}

fn parse_cloth_edges<'a, E: ParseError<&'a [u8]>>(
) -> impl Fn(&'a [u8]) -> IResult<&'a [u8], Option<(Vec<ClothEdge>, Vec<u32>, ClothSimParams)>, E> {
    flat_map(le_u32, move |c| {
        cond(
            c != 0,
            tuple((
                count(ClothEdge::parser(), c as usize),
                count(le_u32, c as usize),
                ClothSimParams::parser(),
            )),
        )
    })
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct ClothVertex {
    pub max_distance: f32,
    pub is_pinned: bool,
}

impl ClothVertex {
    pub fn parser<'a, E: ParseError<&'a [u8]>>() -> impl Fn(&'a [u8]) -> IResult<&'a [u8], Self, E>
    {
        map(tuple((le_f32, le_u32)), |data| ClothVertex {
            max_distance: data.0,
            is_pinned: data.1 != 0,
        })
    }
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct BoneIndexData {
    pub index0: u8,
    pub weight0: u16,
    pub index1: u8,
    pub weight1: u16,
}

impl BoneIndexData {
    pub fn parser<'a, E: ParseError<&'a [u8]>>() -> impl Fn(&'a [u8]) -> IResult<&'a [u8], Self, E>
    {
        map(tuple((le_u8, le_u16, le_u8, le_u16)), |data| {
            BoneIndexData {
                index0: data.0,
                weight0: data.1,
                index1: data.2,
                weight1: data.3,
            }
        })
    }
}

fn parse_bones<'a, E: ParseError<&'a [u8]>>(
    vertex_count: usize,
) -> impl Fn(&'a [u8]) -> IResult<&'a [u8], Option<(Vec<String>, Vec<BoneIndexData>)>, E> {
    flat_map(le_u32, move |bc| {
        cond(
            bc != 0,
            pair(
                count(sized_string, bc as usize),
                count(BoneIndexData::parser(), vertex_count),
            ),
        )
    })
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct Face(pub [u16; 3]);

impl Face {
    pub fn parser<'a, E: ParseError<&'a [u8]>>() -> impl Fn(&'a [u8]) -> IResult<&'a [u8], Self, E>
    {
        map(tuple((le_u16, le_u16, le_u16)), |data| {
            Face([data.0, data.1, data.2])
        })
    }
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct Gate {
    pub name: String,
    pub vertices: Vec<Vector3<f32>>,
    pub faces: Vec<Face>,
}

impl Gate {
    pub fn parser<'a, E: ParseError<&'a [u8]>>() -> impl Fn(&'a [u8]) -> IResult<&'a [u8], Self, E>
    {
        map(
            tuple((
                sized_string,
                parse_objects_u32(vector3_f32),
                parse_objects_u32(Face::parser()),
            )),
            |data| Gate {
                name: data.0,
                vertices: data.1,
                faces: data.2,
            },
        )
    }
}

#[derive(Debug)]
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
    pub fn parser<'a, E: ParseError<&'a [u8]>>(
        nav_flag: NavFlags,
    ) -> impl Fn(&'a [u8]) -> IResult<&'a [u8], Self, E> {
        map(
            tuple((
                le_u16,
                le_u16,
                le_u16,
                le_u16,
                le_u8,
                cond(nav_flag.contains(NavFlags::UNK0), le_u8),
            )),
            |data| ObjectLines {
                vertex_source: data.0,
                vertex_destination: data.1,
                cell_source: data.2,
                cell_destination: data.3,
                collision_flag: data.4,
                unk: data.5,
            },
        )
    }
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct NavMesh {
    pub vertices: Vec<(Vector3<f32>, u8)>,
    pub ground: Vec<(Face, u16, Option<u8>)>,
    pub outlines: Vec<ObjectLines>,
    pub inlines: Vec<ObjectLines>,
    pub event: Vec<String>,
    pub unk0: f32,
    pub unk1: f32,
    pub unk2: u32,
    pub unk3: u32,
    pub unk4: Vec<Vec<u16>>,
}

impl NavMesh {
    pub fn parser<'a, E: ParseError<&'a [u8]>>(
        nav_flag: NavFlags,
    ) -> impl Fn(&'a [u8]) -> IResult<&'a [u8], Self, E> {
        map(
            tuple((
                parse_objects_u32(pair(vector3_f32, le_u8)),
                parse_objects_u32(tuple((
                    Face::parser(),
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
            |data| NavMesh {
                vertices: data.0,
                ground: data.1,
                outlines: data.2,
                inlines: data.3,
                event: data.4,
                unk0: data.5,
                unk1: data.6,
                unk2: data.7,
                unk3: data.8,
                unk4: data.9,
            },
        )
    }
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct JmxBMesh {
    pub header: JmxBMeshHeader,
    pub vertices: Vec<Vertex>,
    pub light_map_path: Option<String>,
    pub bone_data: Option<(Vec<String>, Vec<BoneIndexData>)>,
    pub faces: Vec<Face>,
    pub cloth_vertex: Vec<ClothVertex>,
    pub cloth_edges: Option<(Vec<ClothEdge>, Vec<u32>, ClothSimParams)>,
    pub bounding_box: [f32; 6],
    pub gates: Vec<Gate>,
    pub nav_mesh: Option<NavMesh>,
}

impl JmxBMesh {
    pub fn parse<'a, E: ParseError<&'a [u8]>>(i: &'a [u8]) -> Result<Self, nom::Err<E>> {
        let (_, header) = JmxBMeshHeader::parse(i)?;
        let has_light_map = header.vertex_flags.contains(VertexFlags::HAS_LIGHT_MAP);
        let (_, (vertices, light_map_path)) = pair(
            parse_objects_u32(Vertex::parser(has_light_map)),
            cond(has_light_map, sized_string),
        )(&i[header.vertex as usize..])?;
        let (_, bone_data) = parse_bones(vertices.len())(&i[header.skin as usize..])?;
        let (_, faces) = parse_objects_u32(Face::parser())(&i[header.face as usize..])?;
        let (_, cloth_vertex) =
            parse_objects_u32(ClothVertex::parser())(&i[header.cloth_vertex as usize..])?;
        let (_, cloth_edges) = parse_cloth_edges()(&i[header.cloth_edge as usize..])?;
        let (_, bounding_box) = map(
            tuple((le_f32, le_f32, le_f32, le_f32, le_f32, le_f32)),
            |data| [data.0, data.1, data.2, data.3, data.4, data.5],
        )(&i[header.bounding_box as usize..])?;
        let (_, gates) = parse_objects_u32(Gate::parser())(&i[header.gate as usize..])?;
        let (_, nav_mesh) = cond(header.nav_mesh != 0, NavMesh::parser(header.nav_flags))(
            &i[header.nav_mesh as usize..],
        )?;

        Ok(JmxBMesh {
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
        })
    }
}

#[derive(Debug)]
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
    pub name: String,
    pub material: String,
    pub unk5: u32,
}

impl JmxBMeshHeader {
    pub fn parse<'a, E: ParseError<&'a [u8]>>(i: &'a [u8]) -> IResult<&'a [u8], Self, E> {
        map(
            preceded(
                alt((tag(b"JMXVBMS 0109"), tag(b"JMXVBMS 0110"))),
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
                    map(le_u32, |flags| {
                        NavFlags::from_bits(flags)
                            .unwrap_or_else(|| panic!("Unknown NavFlags encountered 0b{:b}", flags))
                    }),
                    le_u32,
                    map(le_u32, |flags| {
                        VertexFlags::from_bits(flags).unwrap_or_else(|| {
                            panic!("Unknown VertexFlags encountered 0b{:b}", flags)
                        })
                    }),
                    le_u32,
                    sized_string,
                    sized_string,
                    le_u32,
                )),
            ),
            |data| JmxBMeshHeader {
                vertex: data.0,
                skin: data.1,
                face: data.2,
                cloth_vertex: data.3,
                cloth_edge: data.4,
                bounding_box: data.5,
                gate: data.6,
                nav_mesh: data.7,
                unk0: data.8,
                unk1: data.9,
                unk3: data.10,
                nav_flags: data.11,
                sub_prim_count: data.12,
                vertex_flags: data.13,
                unk4: data.14,
                name: data.15,
                material: data.16,
                unk5: data.17,
            },
        )(i)
    }
}
