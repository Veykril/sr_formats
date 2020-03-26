#![warn(clippy::all)]
use nom::bytes::complete::{tag, take};
use nom::character::complete::{digit1, hex_digit1};
use nom::combinator::{flat_map, map, map_res};
use nom::error::ParseError;
use nom::multi::count;
use nom::number::complete::{le_f32, le_u16, le_u32, le_u8};
use nom::sequence::{preceded, tuple};
use nom::{IResult, ToUsize};

use std::path::PathBuf;

pub mod gmwpfort;
pub mod jmxvban;
pub mod jmxvbms;
pub mod jmxvbmt;
pub mod jmxvbsk;
pub mod jmxvbsr;
pub mod jmxvcpd;
pub mod jmxvddj;
pub mod jmxvdof;
pub mod jmxvenvi;
pub mod jmxvmapm;
pub mod jmxvmapo;
pub mod jmxvmapt;
pub mod jmxvmfo;
pub mod jmxvnvm;
pub mod jmxvobji;
pub mod newinterface;

pub mod enums;
pub use enums::*;

pub fn struple_map<I, O1, O2, E, F>(first: F) -> impl Fn(I) -> IResult<I, O2, E>
where
    O1: struple::GenericTuple,
    O2: struple::Struple<Tuple = O1>,
    E: ParseError<I>,
    F: Fn(I) -> IResult<I, O1, E>,
{
    map(first, struple::Struple::from_tuple)
}

pub fn struple<I, O1, O2, E, List>(l: List) -> impl Fn(I) -> IResult<I, O2, E>
where
    I: Clone,
    O1: struple::GenericTuple,
    O2: struple::Struple<Tuple = O1>,
    E: ParseError<I>,
    List: nom::sequence::Tuple<I, O1, E>,
{
    struple_map(move |i: I| l.parse(i))
}

pub fn parse_u32_hex_str<'a, E>(input: &'a str) -> IResult<&'a str, u32, E>
where
    E: ParseError<&'a str>,
{
    preceded(
        tag("0x"),
        map_res(hex_digit1, |s| u32::from_str_radix(s, 16)),
    )(input)
}

pub fn parse_u32_str<'a, E>(input: &'a str) -> IResult<&'a str, u32, E>
where
    E: ParseError<&'a str>,
{
    map_res(digit1, |s| u32::from_str_radix(s, 10))(input)
}

pub fn parse_u16_str<'a, E>(input: &'a str) -> IResult<&'a str, u16, E>
where
    E: ParseError<&'a str>,
{
    map_res(digit1, |s| u16::from_str_radix(s, 10))(input)
}

pub fn flags<'a, E, F, T, B, BF>(
    bits_func: BF,
    from_bits: F,
) -> impl Fn(&'a [u8]) -> IResult<&'a [u8], T, E>
where
    E: ParseError<&'a [u8]>,
    F: Fn(B) -> Option<T>,
    BF: Fn(&'a [u8]) -> IResult<&'a [u8], B, E>,
{
    move |i| {
        let (i, bits) = bits_func(i)?;
        match from_bits(bits) {
            Some(flags) => Ok((i, flags)),
            // TODO: nom doesnt actually have custom error kinds which is kind of a shame
            None => Err(nom::Err::Error(E::add_context(
                &[],
                "unknown bitflags encountered",
                E::from_error_kind(i, nom::error::ErrorKind::OneOf),
            ))),
        }
    }
}

#[inline]
pub fn flags_u16<'a, E, F, T>(from_bits: F) -> impl Fn(&'a [u8]) -> IResult<&'a [u8], T, E>
where
    E: ParseError<&'a [u8]>,
    F: Fn(u16) -> Option<T>,
{
    flags(le_u16, from_bits)
}

#[inline]
pub fn flags_u32<'a, E, F, T>(from_bits: F) -> impl Fn(&'a [u8]) -> IResult<&'a [u8], T, E>
where
    E: ParseError<&'a [u8]>,
    F: Fn(u32) -> Option<T>,
{
    flags(le_u32, from_bits)
}

/// Reads a u32, then reads the amount of bytes specified by the u32 and parses it as a EUC_KR string encoded string
#[inline]
pub fn sized_string<'a, E: ParseError<&'a [u8]>>(i: &'a [u8]) -> IResult<&'a [u8], String, E> {
    map(flat_map(le_u32, take), |s| {
        encoding_rs::EUC_KR
            .decode_without_bom_handling(s)
            .0
            .into_owned()
    })(i)
}

/// Reads a u16, then reads the amount of bytes specified by the u32 and parses it as a EUC_KR string encoded string
#[inline]
pub fn small_sized_string<'a, E: ParseError<&'a [u8]>>(
    i: &'a [u8],
) -> IResult<&'a [u8], String, E> {
    map(flat_map(le_u16, take), |s| {
        encoding_rs::EUC_KR
            .decode_without_bom_handling(s)
            .0
            .into_owned()
    })(i)
}

/// Reads a sized_string and turns it into a PathBuf
#[inline]
pub fn sized_path<'a, E: ParseError<&'a [u8]>>(i: &'a [u8]) -> IResult<&'a [u8], PathBuf, E> {
    map(sized_string, From::from)(i)
}

#[inline]
pub fn fixed_string<'a, E: ParseError<&'a [u8]>>(
    size: usize,
) -> impl Fn(&'a [u8]) -> IResult<&'a [u8], String, E> {
    map(take(size), move |bytes: &'a [u8]| {
        let len = bytes.iter().position(|&b| b == 0).unwrap_or(size);
        encoding_rs::EUC_KR
            .decode_without_bom_handling(&bytes[..len])
            .0
            .into_owned()
    })
}

#[inline]
pub fn fixed_string_64<'a, E: ParseError<&'a [u8]>>(i: &'a [u8]) -> IResult<&'a [u8], String, E> {
    fixed_string(64)(i)
}

#[inline]
pub fn fixed_string_128<'a, E: ParseError<&'a [u8]>>(i: &'a [u8]) -> IResult<&'a [u8], String, E> {
    fixed_string(128)(i)
}

#[inline]
pub fn fixed_string_256<'a, E: ParseError<&'a [u8]>>(i: &'a [u8]) -> IResult<&'a [u8], String, E> {
    fixed_string(256)(i)
}

/// Reads a [f32; 6] array
#[inline]
pub fn vector6_f32<'a, E: ParseError<&'a [u8]>>(i: &'a [u8]) -> IResult<&'a [u8], [f32; 6], E> {
    map(
        tuple((le_f32, le_f32, le_f32, le_f32, le_f32, le_f32)),
        |t| [t.0, t.1, t.2, t.3, t.4, t.5],
    )(i)
}

/// Reads a Vector3<f32>
#[inline]
pub fn vector4_f32<'a, E: ParseError<&'a [u8]>>(i: &'a [u8]) -> IResult<&'a [u8], Vector4<f32>, E> {
    map(tuple((le_f32, le_f32, le_f32, le_f32)), |t| Vector4 {
        x: t.0,
        y: t.1,
        z: t.2,
        w: t.3,
    })(i)
}

/// Reads a Vector3<f32>
#[inline]
pub fn vector3_f32<'a, E: ParseError<&'a [u8]>>(i: &'a [u8]) -> IResult<&'a [u8], Vector3<f32>, E> {
    map(tuple((le_f32, le_f32, le_f32)), |t| Vector3 {
        x: t.0,
        y: t.1,
        z: t.2,
    })(i)
}

/// Reads a Vector2<f32>
#[inline]
pub fn vector2_f32<'a, E: ParseError<&'a [u8]>>(i: &'a [u8]) -> IResult<&'a [u8], Vector2<f32>, E> {
    map(tuple((le_f32, le_f32)), |t| Vector2 { x: t.0, y: t.1 })(i)
}

/// Runs the `parse_fn` as many times as what the `count_fn` returns as a number.
#[inline]
fn parse_objects<'a, E, T, F, S, R>(
    count_fn: S,
    parse_fn: F,
) -> impl Fn(&'a [u8]) -> IResult<&'a [u8], Vec<T>, E>
where
    E: ParseError<&'a [u8]>,
    F: Fn(&'a [u8]) -> IResult<&'a [u8], T, E>,
    S: Fn(&'a [u8]) -> IResult<&'a [u8], R, E>,
    R: ToUsize,
{
    // the outer move closure moves count_fn into it, keeping it alive long enough to be used by the flat_map closure
    move |input: &[u8]| flat_map(&count_fn, |c| count(&parse_fn, c.to_usize()))(input)
}

/// Reads a u32 and then runs `parse_fn` that many times.
#[inline]
fn parse_objects_u32<'a, T, E: ParseError<&'a [u8]>, F: Fn(&'a [u8]) -> IResult<&'a [u8], T, E>>(
    parse_fn: F,
) -> impl Fn(&'a [u8]) -> IResult<&'a [u8], Vec<T>, E> {
    parse_objects(le_u32, parse_fn)
}

/// Reads a u16 and then runs `parse_fn` that many times.
#[inline]
fn parse_objects_u16<'a, T, E: ParseError<&'a [u8]>, F: Fn(&'a [u8]) -> IResult<&'a [u8], T, E>>(
    parse_fn: F,
) -> impl Fn(&'a [u8]) -> IResult<&'a [u8], Vec<T>, E> {
    parse_objects(le_u16, parse_fn)
}

/// Reads a u8 and then runs `parse_fn` that many times.
#[inline]
fn parse_objects_u8<'a, T, E: ParseError<&'a [u8]>, F: Fn(&'a [u8]) -> IResult<&'a [u8], T, E>>(
    parse_fn: F,
) -> impl Fn(&'a [u8]) -> IResult<&'a [u8], Vec<T>, E> {
    parse_objects(le_u8, parse_fn)
}

/// Runs f count times, while passing the iteration index to f
pub fn count_indexed<I, O, E, F>(f: F, count: usize) -> impl Fn(I) -> IResult<I, Vec<O>, E>
where
    I: Clone + PartialEq,
    F: Fn(I, usize) -> IResult<I, O, E>,
    E: ParseError<I>,
{
    move |i: I| {
        let mut input = i.clone();
        let mut res = Vec::new();

        for idx in 0..count {
            let input_ = input.clone();
            match f(input_, idx) {
                Ok((i, o)) => {
                    res.push(o);
                    input = i;
                },
                Err(nom::Err::Error(e)) => {
                    return Err(nom::Err::Error(E::append(
                        i,
                        nom::error::ErrorKind::Count,
                        e,
                    )));
                },
                Err(e) => {
                    return Err(e);
                },
            }
        }

        Ok((input, res))
    }
}

#[cfg(feature = "serde")]
use serde::Serialize;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct Vector2<T> {
    pub x: T,
    pub y: T,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct Vector3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct Vector4<T> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}

pub type VerboseError<'a> = nom::error::VerboseError<&'a [u8]>;
pub type NormalError<'a> = (&'a [u8], nom::error::ErrorKind);
