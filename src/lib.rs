#![warn(clippy::all)]
use nom::{
    bytes::complete::take,
    combinator::{flat_map, map},
    error::ParseError,
    multi::count,
    number::complete::{le_f32, le_u16, le_u32, le_u8},
    sequence::tuple,
    IResult, ToUsize,
};

use std::path::PathBuf;

pub mod jmxvbms;
pub mod jmxvbmt;
pub mod jmxvbsk;
pub mod jmxvbsr;
pub mod jmxvcpd;
pub mod jmxvddj;
pub mod jmxvenvi;
pub mod jmxvmapm;
pub mod jmxvmapo;
pub mod jmxvmapt;
pub mod jmxvnvm;

pub mod enums;
pub use enums::*;

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

/// Reads a sized_string and turns it into a PathBuf
#[inline]
pub fn sized_path<'a, E: ParseError<&'a [u8]>>(i: &'a [u8]) -> IResult<&'a [u8], PathBuf, E> {
    map(sized_string, From::from)(i)
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
    move |input: &[u8]| flat_map(&count_fn, |c| count(&parse_fn, c.to_usize()))(input)
}

/// Reads a u32 and then runs `parse_fn` that many times.
#[inline]
fn parse_objects_u32<'a, T, F: Fn(&'a [u8]) -> IResult<&'a [u8], T>>(
    parse_fn: F,
) -> impl Fn(&'a [u8]) -> IResult<&'a [u8], Vec<T>> {
    parse_objects(le_u32, parse_fn)
}

/// Reads a u16 and then runs `parse_fn` that many times.
#[inline]
fn parse_objects_u16<'a, T, F: Fn(&'a [u8]) -> IResult<&'a [u8], T>>(
    parse_fn: F,
) -> impl Fn(&'a [u8]) -> IResult<&'a [u8], Vec<T>> {
    parse_objects(le_u16, parse_fn)
}

/// Reads a u8 and then runs `parse_fn` that many times.
#[inline]
fn parse_objects_u8<'a, T, F: Fn(&'a [u8]) -> IResult<&'a [u8], T>>(
    parse_fn: F,
) -> impl Fn(&'a [u8]) -> IResult<&'a [u8], Vec<T>> {
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

use serde::Serialize;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize)]
pub struct Vector2<T> {
    x: T,
    y: T,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize)]
pub struct Vector3<T> {
    x: T,
    y: T,
    z: T,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize)]
pub struct Vector4<T> {
    x: T,
    y: T,
    z: T,
    w: T,
}
