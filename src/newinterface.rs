use nom::combinator::map;
use nom::error::ParseError;
use nom::number::complete::{le_f32, le_u32};
use nom::sequence::tuple;
use nom::IResult;

use std::path::PathBuf;

use crate::enums::NewInterfaceType;
use crate::{fixed_string_128, fixed_string_256, fixed_string_64, flags_u32, parse_objects_u32};

#[cfg(feature = "serde")]
use serde::Serialize;

bitflags::bitflags! {
    #[cfg_attr(feature = "serde", derive(Serialize))]
    #[cfg_attr(feature = "serde", serde(transparent))]
    pub struct NewInterfaceStyle: u32 {
        const CENTER = 256;
        const RIGHT = 512;
        const LINECENTER = 65536;
    }
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct NewInterface {
    pub name: String,
    pub image: PathBuf,
    pub background: PathBuf,
    pub text: PathBuf,
    pub description: String,
    pub prototype: String,
    pub ty: NewInterfaceType,
    pub id: u32,
    pub parent_id: u32,
    pub grand_parent_id: u32,
    pub unk00: u32,
    pub unk01: u32,
    /// RGBA
    pub color: u32,
    pub client_rectangle_x: u32,
    pub client_rectangle_y: u32,
    pub client_rectangle_width: u32,
    pub client_rectangle_height: u32,
    pub uv_top_left_x: f32,
    pub uv_top_left_y: f32,
    pub uv_top_right_x: f32,
    pub uv_top_right_y: f32,
    pub uv_bot_left_x: f32,
    pub uv_bot_left_y: f32,
    pub uv_bot_right_x: f32,
    pub uv_bot_right_y: f32,
    pub unk02: u32,
    //used on TabButton and is pointing to a Frame
    pub content_id: u32,
    // u32
    pub is_root: bool,
    pub unk03: u32,
    pub unk04: u32,
    pub unk05: u32,
    pub unk06: u32,
    pub unk07: u32,
    pub unk08: u32,
    pub unk09: u32,
    pub unk10: u32,
    pub unk11: u32,
    pub unk12: u32,
    pub unk13: u32,
    pub unk14: u32,
    pub unk15: u32,
    pub style: NewInterfaceStyle,
}

impl NewInterface {
    pub fn parse<'a, E: ParseError<&'a [u8]>>(i: &'a [u8]) -> Result<Vec<Self>, nom::Err<E>> {
        parse_objects_u32(Self::parse_single)(i).map(|(_, x)| x)
    }

    #[rustfmt::skip] // don't look past this line
    fn parse_single<'a, E: ParseError<&'a [u8]>>(i: &'a [u8]) -> IResult<&'a [u8], Self, E> {
        let (i, (name, image, background, text, description, prototype)) = tuple((
            fixed_string_64,
            map(fixed_string_256, From::from),
            map(fixed_string_256, From::from),
            map(fixed_string_128, From::from),
            fixed_string_64,
            fixed_string_64,
        ))(i)?;
        let (i,
            (
                ty, id, parent_id, grand_parent_id, unk00, unk01, color, client_rectangle_x,
                client_rectangle_y, client_rectangle_width, client_rectangle_height,
            ),
        ) = tuple((
            NewInterfaceType::parse, le_u32, le_u32, le_u32, le_u32,
            le_u32, le_u32, le_u32, le_u32, le_u32, le_u32,
        ))(i)?;
        let (i,
            (
                uv_top_left_x, uv_top_left_y, uv_top_right_x, uv_top_right_y, uv_bot_left_x,
                uv_bot_left_y, uv_bot_right_x, uv_bot_right_y, unk02, content_id, is_root,
            ),
        ) = tuple((
            le_f32, le_f32, le_f32, le_f32, le_f32, le_f32, le_f32, le_f32, le_u32, le_u32,
            map(le_u32, |int| int != 0),
        ))(i)?;
        let (i,
            (
                unk03, unk04, unk05, unk06, unk07, unk08, unk09,
                unk10, unk11, unk12, unk13, unk14, unk15, style,
            ),
        ) = tuple((
            le_u32, le_u32, le_u32, le_u32, le_u32, le_u32, le_u32, le_u32,
            le_u32, le_u32, le_u32, le_u32, le_u32, flags_u32(NewInterfaceStyle::from_bits),
        ))(i)?;
        Ok((
            i,
            NewInterface {
                name, image, background, text, description, prototype, ty, id, parent_id,
                grand_parent_id, unk00, unk01, color, client_rectangle_x, client_rectangle_y,
                client_rectangle_width, client_rectangle_height, uv_top_left_x, uv_top_left_y,
                uv_top_right_x, uv_top_right_y, uv_bot_left_x, uv_bot_left_y, uv_bot_right_x,
                uv_bot_right_y, unk02, content_id, is_root, unk03, unk04, unk05, unk06, unk07,
                unk08, unk09, unk10, unk11, unk12, unk13, unk14, unk15, style,
            },
        ))
    }
}
