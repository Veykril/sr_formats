use mint::{RowMatrix4, Vector3, Vector4};
use nom::bytes::complete::tag;
use nom::combinator::{cond, flat_map, map};
use nom::error::{make_error, ErrorKind};
use nom::number::complete::{le_f32, le_i32, le_u32, le_u8};
use nom::sequence::{pair, tuple};
use nom::IResult;

use crate::parser_ext::complete::take_fixed;
use crate::parser_ext::multi::parse_objects_u32;
use crate::parser_ext::number::{matrix4x4, vector3_f32, vector4_f32};
use crate::parser_ext::string::{sized_string, sized_string_ref};
use crate::ttr_closure;

#[derive(Clone, Debug, PartialEq)]
pub struct JmxvEff {
    pub header: JmxvEffHeader,
    pub root: EFStoredObject,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct JmxvEffHeader {
    pub version: [u8; 4],
    pub v12_unk0: u32,
    pub v13_unk0: u32,
    pub v13_unk1: u32,
    pub v13_unk2: u32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct EFStoredObject {
    pub name: Box<str>,
    pub controllers: Box<[EFController]>,
    /// Initial state data?
    pub global_data: EEGlobalData,
    pub empty_sl0: EESourceList,
    // EFStaticEmit
    pub emitter_sl: EESourceList, // EECFuncP1
    pub empty_sl2: EESourceList,
    // NormalTimeExtinct, NormalTimeLoop, NeverExtinct
    pub lifetime_source: EESource, // EECFuncP0
    // ProgramUpdate
    pub program_sl: EESourceList, // EECFuncP0
    pub unkb0: u8,
    pub unkb1: u8,
    pub unk0: u32,
    pub unk1: u32,
    pub unk2: u32,
    pub unkb2: u8,
    pub unk3: u32,
    pub unkb3: u8,
    // ViewNone, ViewBillboard, ViewVBillboard, ViewYBillboard
    pub view_mode_source: EESource, // EECCodeP0
    pub resource: EEResource,
    // RenderPlate, RenderLinkObj, RenderMesh, RenderNone, RenderLinkPipe, RenderLinkDPipe
    pub render_source: EESource, // EECCodeP0
    pub empty_sl3: EESourceList,
    // SetShapeRotVel, SetGraphScale, SetPosition, SetGraphRandomScale, SetVelocity, SetConePos, Force, SetRVelocity, SetBANRot, SetConeVel, SetSpherePos, TextureSlide, ConeForce, SetBANPos, SetShapeRot, SetGraphDiffuse, Attraction, SetRotationSetRotation
    pub render_sl: EESourceList, // EECCodeP1
    pub children: Box<[EFStoredObject]>,
}

/// I wonder what these are for? 🤔
#[derive(Clone, Debug, PartialEq)]
pub enum EFController {
    // Basic
    NormalTimeLife,
    NormalTimeLoopLife,
    Program(EEProgram),
    StaticEmit(EFStaticEmit),

    // Position
    Ban(BSAnimation),
    LinkMode {
        unk0: u32,
        unk1: u32,
        unk2: u32,
        unk3: u32,
    },

    // Decoration
    DiffuseGraph {
        scale_x: EEBlend<u8>,
        scale_y: EEBlend<Color>,
    },
    Shape {
        shape: RenderShape,
        resource: EEResource,
    },
    ScaleGraph {
        scale_x: EEBlend<f32>,
        scale_y: EEBlend<f32>,
        scale_z: EEBlend<f32>,
        float0: f32,
        float1: f32,
    },
    ViewMode(ViewMode),
}

/// Specifies whether the particles produced by the system should be locked towards the camera.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum ViewMode {
    /// No locking
    None,
    /// Lock both axis towards the camera
    Billboard,
    /// Lock the vertical axis towards the camera.
    VBillboard,
    /// Lock the horizontal axis towards the camera.
    YBillboard,
}

/// What kind of mesh does the particle use
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum RenderShape {
    /// Trail?
    LinkDPipe,
    /// Trail?
    LinkObj,
    /// Trail?
    LinkPipe,
    /// Custom mesh
    Mesh,
    /// None, it won't be rendered?
    None,
    /// Classic quad
    Plate,
}

#[derive(Clone, Debug, PartialEq)]
pub struct EESourceList(pub Box<[Option<EESourceData>]>);

#[derive(Clone, Debug, PartialEq)]
pub struct EESource(pub Option<EESourceData>);

#[derive(Clone, Debug, PartialEq)]
pub struct EESourceData {
    pub command: EECommand,
    pub subtype: u8,
    pub unkb1: u8,
    pub start: f32,
    pub end: f32,
    pub unkf0: f32,
}

#[derive(Clone, Debug, PartialEq)]
pub enum EECommand {
    // EECFuncP0
    NeverExtinct,
    NormalTimeExtinct,
    NormalTimeLoop,
    ProgramUpdate,
    // EECFuncP1
    StaticEmit(EFStaticEmit),

    // EECCodeP0
    RenderShape(RenderShape),
    ViewMode(ViewMode),

    // EECCodeP1
    Attraction(f32),
    ConeForce(AngleVector1),
    Force(Vector3<f32>),
    SetBANPos(FrameBANPosition),
    SetBANRot(FrameBANRotation),
    SetConePos(AngleVector1),
    SetConeVel(AngleVector1),
    SetGraphDiffuse(FrameDiffuse),
    SetGraphRandomScale(f32),
    SetGraphScale(FrameScale),
    SetPosition(Vector3<f32>),
    SetRotation(RotVector),
    SetRotationAxis(AxisVector4),
    SetRotationMat(RowMatrix4<f32>),
    SetRVelocity(RotVector),
    SetRVelocityAxis(AxisVector4),
    SetRVelocityMat(RowMatrix4<f32>),
    SetShapeRot(AxisVector4),
    SetShapeRotVel(AxisVector4),
    SetSpherePos(Vector3<f32>),
    SetVelocity(Vector3<f32>),
    TextureSlide(FrameTextureSlide),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct EEResource {
    pub two_sided: bool,
    pub src_blend: u32,
    pub dst_blend: u32,
    pub src_texture_arg0: u32,
    pub src_texture_arg1: u32,
    pub src_texture_op: u32,
    pub dst_texture_arg0: u32,
    pub dst_texture_arg1: u32,
    pub dst_texture_op: u32,
    // (mesh, texture)
    pub meshes: Box<[(Box<str>, Box<[Box<str>]>)]>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct BSAnimation(Box<[Box<str>]>);

#[derive(Clone, Debug, PartialEq)]
pub struct EEGlobalData {
    pub unk0: u32,
    pub parameters: Box<[EEParameter]>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct EEProgram(pub EESourceList);

#[derive(Clone, Debug, PartialEq)]
pub struct EFStaticEmit {
    pub min: i32,
    pub max: i32,
    pub burst_rate: i32,
    pub min_particles: i32,
    pub spawn_rate: f32,
}

#[derive(Clone, Debug, PartialEq)]
pub enum EEParameter {
    Float(f32),
    Vector(Vector3<f32>),
    Matrix(RowMatrix4<f32>),
    StaticEmit(EFStaticEmit),
    AxisVector4(AxisVector4),
    RotVector(RotVector),
    AngleVector1(AngleVector1),
    FrameScale(FrameScale),
    FrameDiffuse(FrameDiffuse),
    FrameBANRotation(FrameBANRotation),
    FrameBANPosition(FrameBANPosition),
    FrameTextureSlide(FrameTextureSlide),
    BSAnimation(BSAnimation),
    BlendScaleGraph(EEBlend<Vector3<f32>>),
    BlendScaleGraphPointer(f32),
    BlendDiffuseGraph(EEBlend<Color>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct AngleVector1(pub Vector3<f32>, pub Vector3<f32>);

#[derive(Clone, Debug, PartialEq)]
pub struct AxisVector4(pub Vector4<f32>, pub RowMatrix4<f32>);

#[derive(Clone, Debug, PartialEq)]
pub struct FrameTextureSlide(pub Vector3<f32>, pub Box<[Vector4<f32>]>);

#[derive(Clone, Debug, PartialEq)]
pub struct RotVector(pub Vector3<f32>, pub RowMatrix4<f32>);

#[derive(Clone, Debug, PartialEq)]
pub struct FrameBANPosition(pub f32, pub Box<[Vector3<f32>]>);

#[derive(Clone, Debug, PartialEq)]
pub struct FrameBANRotation(pub f32, pub Box<[RowMatrix4<f32>]>);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct FrameDiffuse(pub Box<[Color]>);

#[derive(Clone, Debug, PartialEq)]
pub struct FrameScale(pub Box<[Vector3<f32>]>);

#[derive(Clone, Debug, PartialEq)]
pub struct EEBlend<T> {
    pub begin: f32,
    pub end: f32,
    pub blends: Box<[(f32, T)]>,
}

// ARGB32
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Color(pub u32);

impl JmxvEff {
    pub fn parse(i: &[u8]) -> IResult<&[u8], JmxvEff> {
        map(
            pair(JmxvEffHeader::parse, EFStoredObject::parse),
            ttr_closure!(JmxvEff { header, root }),
        )(i)
    }
}

impl JmxvEffHeader {
    fn parse(i: &[u8]) -> IResult<&[u8], Self> {
        let (i, (_, version)) = pair(tag(b"JMXVEFF "), take_fixed::<4>)(i)?;
        let mut header = JmxvEffHeader {
            version,
            v12_unk0: 0,
            v13_unk0: 0,
            v13_unk1: 0,
            v13_unk2: 0,
        };

        let i = match &version {
            b"0010" | b"0011" => i,
            b"0012" | b"0013" => {
                let (i, v12_unk0) = le_u32(i)?;
                header.v12_unk0 = v12_unk0;
                let (i, unk) = cond(&version == b"0013", tuple((le_u32, le_u32, le_u32)))(i)?;
                if let Some((v13_unk0, v13_unk1, v13_unk2)) = unk {
                    header.v13_unk0 = v13_unk0;
                    header.v13_unk1 = v13_unk1;
                    header.v13_unk2 = v13_unk2;
                }
                i
            },
            _ => return Err(nom::Err::Failure(make_error(i, ErrorKind::Alt))),
        };
        Ok((i, header))
    }
}

impl EFStoredObject {
    fn parse(i: &[u8]) -> IResult<&[u8], EFStoredObject> {
        let (_, (data_offset, name, controllers)) =
            tuple((le_u32, sized_string, parse_objects_u32(EFController::parse)))(i)?;
        let i = i
            .get(data_offset as usize + 4..)
            .ok_or_else(|| nom::Err::Failure(make_error(i, ErrorKind::Eof)))?;

        let (i, res) = tuple((
            EEGlobalData::parse,
            EESourceList::parse,
            EESourceList::parse,
            EESourceList::parse,
            EESource::parse,
            EESourceList::parse,
            le_u8,
            le_u8,
            le_u32,
            le_u32,
            le_u32,
            le_u8,
            le_u32,
            le_u8,
            EESource::parse,
            EEResource::parse,
            EESource::parse,
            EESourceList::parse,
            EESourceList::parse,
            parse_objects_u32(EFStoredObject::parse),
        ))(i)?;
        let this = ttr_closure!(name, controllers -> EFStoredObject {
            global_data,
            empty_sl0,
            emitter_sl,
            empty_sl2,
            lifetime_source,
            program_sl,
            unkb0,
            unkb1,
            unk0,
            unk1,
            unk2,
            unkb2,
            unk3,
            unkb3,
            view_mode_source,
            resource,
            render_source,
            empty_sl3,
            render_sl,
            children,
        })(res);
        Ok((i, this))
    }
}

impl EECommand {
    fn parser_for(name: &str) -> impl FnMut(&[u8]) -> IResult<&[u8], Self> + '_ {
        use EECommand::*;
        move |i| match &*name {
            "NeverExtinct" => Ok((i, NeverExtinct)),
            "NormalTimeExtinct" => Ok((i, NormalTimeExtinct)),
            "NormalTimeLoop" => Ok((i, NormalTimeLoop)),

            "StaticEmit" => map(EFStaticEmit::parse, StaticEmit)(i),

            "ProgramUpdate" => Ok((i, ProgramUpdate)),
            "ViewNone" => Ok((i, ViewMode(self::ViewMode::None))),
            "ViewBillboard" => Ok((i, ViewMode(self::ViewMode::Billboard))),
            "ViewYBillboard" => Ok((i, ViewMode(self::ViewMode::YBillboard))),
            "ViewVBillboard" => Ok((i, ViewMode(self::ViewMode::VBillboard))),
            "RenderNone" => Ok((i, RenderShape(self::RenderShape::None))),
            "RenderPlate" => Ok((i, RenderShape(self::RenderShape::Plate))),
            "RenderMesh" => Ok((i, RenderShape(self::RenderShape::Mesh))),
            "RenderLinkDPipe" => Ok((i, RenderShape(self::RenderShape::LinkDPipe))),
            "RenderLinkPipe" => Ok((i, RenderShape(self::RenderShape::LinkPipe))),
            "RenderLinkObj" => Ok((i, RenderShape(self::RenderShape::LinkObj))),

            "Attraction" => map(le_f32, Attraction)(i),
            "ConeForce" => map(AngleVector1::parse, ConeForce)(i),
            "Force" => map(vector3_f32, Force)(i),
            "SetBANPos" => map(FrameBANPosition::parse, SetBANPos)(i),
            "SetBANRot" => map(FrameBANRotation::parse, SetBANRot)(i),
            "SetConePos" => map(AngleVector1::parse, SetConePos)(i),
            "SetConeVel" => map(AngleVector1::parse, SetConeVel)(i),
            "SetGraphDiffuse" => map(FrameDiffuse::parse, SetGraphDiffuse)(i),
            "SetGraphRandomScale" => map(le_f32, SetGraphRandomScale)(i),
            "SetGraphScale" => map(FrameScale::parse, SetGraphScale)(i),
            "SetPosition" => map(vector3_f32, SetPosition)(i),
            "SetRotation" => map(RotVector::parse, SetRotation)(i),
            "SetRotationAxis" => map(AxisVector4::parse, SetRotationAxis)(i),
            "SetRotationMat" => map(matrix4x4, SetRotationMat)(i),
            "SetRVelocity" => map(RotVector::parse, SetRVelocity)(i),
            "SetRVelocityAxis" => map(AxisVector4::parse, SetRVelocityAxis)(i),
            "SetRVelocityMat" => map(matrix4x4, SetRVelocityMat)(i),
            "SetShapeRot" => map(AxisVector4::parse, SetShapeRot)(i),
            "SetShapeRotVel" => map(AxisVector4::parse, SetShapeRotVel)(i),
            "SetSpherePos" => map(vector3_f32, SetSpherePos)(i),
            "SetVelocity" => map(vector3_f32, SetVelocity)(i),
            "TextureSlide" => map(FrameTextureSlide::parse, TextureSlide)(i),
            _ => Err(nom::Err::Failure(make_error(i, ErrorKind::Alt))),
        }
    }
}

impl EESourceList {
    fn parse(i: &[u8]) -> IResult<&[u8], Self> {
        map(parse_objects_u32(EESourceData::parse), Self)(i)
    }
}

impl EESource {
    fn parse(i: &[u8]) -> IResult<&[u8], Self> {
        map(EESourceData::parse, Self)(i)
    }
}

impl EESourceData {
    fn parse(i: &[u8]) -> IResult<&[u8], Option<Self>> {
        let (i, non_empty) = map(le_u8, |u8| u8 != 0)(i)?;
        cond(
            non_empty,
            flat_map(
                tuple((sized_string_ref, le_u8, le_u8, le_f32, le_f32, le_f32)),
                |(command_name, subtype, unkb1, start, end, unkf0)| {
                    // closure wrap as we need to move command_name here
                    move |i| {
                        map(
                            EECommand::parser_for(&command_name),
                            ttr_closure! {
                                subtype, unkb1, start, end, unkf0 -> EESourceData { command }
                            },
                        )(i)
                    }
                },
            ),
        )(i)
    }
}

impl EEResource {
    fn parse(i: &[u8]) -> IResult<&[u8], Self> {
        map(
            tuple((
                map(le_u32, |u32| u32 != 0),
                le_u32,
                le_u32,
                le_u32,
                le_u32,
                le_u32,
                le_u32,
                le_u32,
                le_u32,
                parse_objects_u32(pair(sized_string, parse_objects_u32(sized_string))),
            )),
            ttr_closure!(EEResource {
                two_sided,
                src_blend,
                dst_blend,
                src_texture_arg0,
                src_texture_arg1,
                src_texture_op,
                dst_texture_arg0,
                dst_texture_arg1,
                dst_texture_op,
                meshes,
            }),
        )(i)
    }
}

impl BSAnimation {
    pub fn parse(i: &[u8]) -> IResult<&[u8], Self> {
        map(parse_objects_u32(sized_string), Self)(i)
    }
}

impl ViewMode {
    pub fn parse(i: &[u8]) -> IResult<&[u8], Self> {
        let (i, controller) = sized_string_ref(i)?;
        match controller.as_bytes() {
            b"ViewNone" => Ok((i, Self::None)),
            b"ViewBillboard" => Ok((i, Self::Billboard)),
            b"ViewYBillboard" => Ok((i, Self::YBillboard)),
            b"ViewVBillboard" => Ok((i, Self::VBillboard)),
            _ => Err(nom::Err::Failure(make_error(i, ErrorKind::Alt))),
        }
    }
}

impl RenderShape {
    pub fn parse(i: &[u8]) -> IResult<&[u8], Self> {
        let (i, controller) = sized_string_ref(i)?;
        match controller.as_bytes() {
            b"RenderNone" => Ok((i, RenderShape::None)),
            b"RenderPlate" => Ok((i, RenderShape::Plate)),
            b"RenderMesh" => Ok((i, RenderShape::Mesh)),
            b"RenderLinkDPipe" => Ok((i, RenderShape::LinkDPipe)),
            b"RenderLinkPipe" => Ok((i, RenderShape::LinkPipe)),
            b"RenderLinkObj" => Ok((i, RenderShape::LinkObj)),
            _ => Err(nom::Err::Failure(make_error(i, ErrorKind::Alt))),
        }
    }
}

impl EFController {
    pub fn parse(i: &[u8]) -> IResult<&[u8], EFController> {
        use EFController::*;
        let (i, controller) = sized_string_ref(i)?;
        match controller.as_bytes() {
            b"NormalTimeLife" => Ok((i, NormalTimeLife)),
            b"NormalTimeLoopLife" => Ok((i, NormalTimeLoopLife)),
            b"StaticEmit" => map(self::EFStaticEmit::parse, StaticEmit)(i),
            b"Program" => map(EEProgram::parse, Program)(i),

            b"LinkMode" => map(
                tuple((le_u32, le_u32, le_u32, le_u32)),
                ttr_closure!(LinkMode {
                    unk0,
                    unk1,
                    unk2,
                    unk3,
                }),
            )(i),
            b"BAN" => map(BSAnimation::parse, Ban)(i),

            b"ViewMode" => map(self::ViewMode::parse, ViewMode)(i),
            b"Shape" => map(
                pair(RenderShape::parse, EEResource::parse),
                ttr_closure!(Shape { shape, resource }),
            )(i),
            b"ScaleGraph" => map(
                tuple((
                    <EEBlend<f32>>::parse,
                    <EEBlend<f32>>::parse,
                    <EEBlend<f32>>::parse,
                    le_f32,
                    le_f32,
                )),
                ttr_closure!(ScaleGraph {
                    scale_x,
                    scale_y,
                    scale_z,
                    float0,
                    float1,
                }),
            )(i),
            b"DiffuseGraph" => map(
                tuple((<EEBlend<u8>>::parse, <EEBlend<Color>>::parse)),
                ttr_closure!(DiffuseGraph { scale_x, scale_y }),
            )(i),
            _ => Err(nom::Err::Failure(make_error(i, ErrorKind::Alt))),
        }
    }
}

impl EEGlobalData {
    fn parse(i: &[u8]) -> IResult<&[u8], EEGlobalData> {
        map(
            pair(le_u32, parse_objects_u32(EEParameter::parse)),
            ttr_closure!(EEGlobalData { unk0, parameters }),
        )(i)
    }
}

impl EEProgram {
    fn parse(i: &[u8]) -> IResult<&[u8], Self> {
        map(EESourceList::parse, Self)(i)
    }
}
impl EFStaticEmit {
    fn parse(i: &[u8]) -> IResult<&[u8], Self> {
        map(
            tuple((le_i32, le_i32, le_i32, le_i32, le_f32)),
            ttr_closure!(EFStaticEmit {
                min,
                max,
                burst_rate,
                min_particles,
                spawn_rate
            }),
        )(i)
    }
}

impl AngleVector1 {
    fn parse(i: &[u8]) -> IResult<&[u8], Self> {
        map(pair(vector3_f32, vector3_f32), |(left, right)| {
            AngleVector1(left, right)
        })(i)
    }
}

impl AxisVector4 {
    fn parse(i: &[u8]) -> IResult<&[u8], Self> {
        map(pair(vector4_f32, matrix4x4), |(left, right)| {
            AxisVector4(left, right)
        })(i)
    }
}

impl FrameTextureSlide {
    fn parse(i: &[u8]) -> IResult<&[u8], Self> {
        map(
            pair(vector3_f32, parse_objects_u32(vector4_f32)),
            |(first, last)| FrameTextureSlide(first, last),
        )(i)
    }
}

impl RotVector {
    fn parse(i: &[u8]) -> IResult<&[u8], Self> {
        map(pair(vector3_f32, matrix4x4), |(left, right)| {
            RotVector(left, right)
        })(i)
    }
}

impl FrameBANPosition {
    fn parse(i: &[u8]) -> IResult<&[u8], Self> {
        map(pair(le_f32, parse_objects_u32(vector3_f32)), |(l, r)| {
            FrameBANPosition(l, r)
        })(i)
    }
}
impl FrameBANRotation {
    fn parse(i: &[u8]) -> IResult<&[u8], Self> {
        map(pair(le_f32, parse_objects_u32(matrix4x4)), |(l, r)| {
            FrameBANRotation(l, r)
        })(i)
    }
}

impl FrameDiffuse {
    fn parse(i: &[u8]) -> IResult<&[u8], Self> {
        map(parse_objects_u32(Color::parse), FrameDiffuse)(i)
    }
}

impl FrameScale {
    fn parse(i: &[u8]) -> IResult<&[u8], Self> {
        map(parse_objects_u32(vector3_f32), FrameScale)(i)
    }
}

impl EEParameter {
    fn parse(i: &[u8]) -> IResult<&[u8], Self> {
        let (src, name) = sized_string_ref(i)?;
        match name.as_bytes() {
            b"Float" => map(le_f32, EEParameter::Float)(src),
            b"Vector" => map(vector3_f32, EEParameter::Vector)(src),
            b"Matrix" => map(matrix4x4, EEParameter::Matrix)(src),
            b"AxisVector4" => map(AxisVector4::parse, EEParameter::AxisVector4)(src),
            b"RotVector" => map(RotVector::parse, EEParameter::RotVector)(src),
            b"AngleVector1" => map(AngleVector1::parse, EEParameter::AngleVector1)(src),
            b"FrameScale" => map(FrameScale::parse, EEParameter::FrameScale)(src),
            b"FrameDiffuse" => map(FrameDiffuse::parse, EEParameter::FrameDiffuse)(src),
            b"FrameBANRotation" => map(FrameBANRotation::parse, EEParameter::FrameBANRotation)(src),
            b"FrameBANPosition" => map(FrameBANPosition::parse, EEParameter::FrameBANPosition)(src),
            b"FrameTextureSlide" => {
                map(FrameTextureSlide::parse, EEParameter::FrameTextureSlide)(src)
            },
            b"BSAnimation" => map(BSAnimation::parse, EEParameter::BSAnimation)(src),
            b"BlendeScaleGraphPointer" => map(le_f32, EEParameter::BlendScaleGraphPointer)(src),
            b"StaticEmit" => map(self::EFStaticEmit::parse, EEParameter::StaticEmit)(src),
            b"BlendDiffuseGraph" => {
                map(<EEBlend<Color>>::parse, EEParameter::BlendDiffuseGraph)(src)
            },
            b"BlendScaleGraph" => {
                map(<EEBlend<Vector3<f32>>>::parse, EEParameter::BlendScaleGraph)(src)
            },
            _ => Err(nom::Err::Failure(make_error(i, ErrorKind::Alt))),
        }
    }
}

impl<T> EEBlend<T> {
    fn parser<'i>(
        parser: impl Fn(&'i [u8]) -> IResult<&'i [u8], T>,
    ) -> impl FnMut(&'i [u8]) -> IResult<&'i [u8], EEBlend<T>> {
        map(
            pair(
                pair(le_f32, le_f32),
                parse_objects_u32(pair(le_f32, parser)),
            ),
            |((begin, end), blends)| EEBlend { begin, end, blends },
        )
    }
}

impl EEBlend<u8> {
    fn parse(i: &[u8]) -> IResult<&[u8], Self> {
        Self::parser(le_u8)(i)
    }
}

impl EEBlend<f32> {
    fn parse(i: &[u8]) -> IResult<&[u8], Self> {
        Self::parser(le_f32)(i)
    }
}

impl EEBlend<Vector3<f32>> {
    fn parse(i: &[u8]) -> IResult<&[u8], Self> {
        Self::parser(vector3_f32)(i)
    }
}

impl EEBlend<Color> {
    fn parse(i: &[u8]) -> IResult<&[u8], Self> {
        Self::parser(Color::parse)(i)
    }
}

impl Color {
    fn parse(i: &[u8]) -> IResult<&[u8], Self> {
        map(le_u32, Color)(i)
    }
}
