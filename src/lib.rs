use nalgebra::{Vector2, Vector3, Vector4, Vector6};
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
pub mod jmxvddj;
pub mod jmxvenvi;
pub mod jmxvmapm;
pub mod jmxvmapo;
pub mod jmxvmapt;
pub mod jmxvnvm;

pub mod enums;
pub use enums::*;

#[inline]
pub fn sized_string<'a, E: ParseError<&'a [u8]>>(i: &'a [u8]) -> IResult<&'a [u8], String, E> {
    use encoding::{all::WINDOWS_949, DecoderTrap, Encoding};
    map(flat_map(le_u32, take), |s| {
        WINDOWS_949.decode(s, DecoderTrap::Replace).unwrap()
    })(i)
}

#[inline]
pub fn sized_path<'a, E: ParseError<&'a [u8]>>(i: &'a [u8]) -> IResult<&'a [u8], PathBuf, E> {
    map(sized_string, From::from)(i)
}

#[inline]
pub fn vector6_f32<'a, E: ParseError<&'a [u8]>>(i: &'a [u8]) -> IResult<&'a [u8], Vector6<f32>, E> {
    map(
        tuple((le_f32, le_f32, le_f32, le_f32, le_f32, le_f32)),
        |t| Vector6::new(t.0, t.1, t.2, t.3, t.4, t.5),
    )(i)
}

#[inline]
pub fn vector4_f32<'a, E: ParseError<&'a [u8]>>(i: &'a [u8]) -> IResult<&'a [u8], Vector4<f32>, E> {
    map(tuple((le_f32, le_f32, le_f32, le_f32)), |t| {
        Vector4::new(t.0, t.1, t.2, t.3)
    })(i)
}

#[inline]
pub fn vector3_f32<'a, E: ParseError<&'a [u8]>>(i: &'a [u8]) -> IResult<&'a [u8], Vector3<f32>, E> {
    map(tuple((le_f32, le_f32, le_f32)), |t| {
        Vector3::new(t.0, t.1, t.2)
    })(i)
}

#[inline]
pub fn vector2_f32<'a, E: ParseError<&'a [u8]>>(i: &'a [u8]) -> IResult<&'a [u8], Vector2<f32>, E> {
    map(tuple((le_f32, le_f32)), |t| Vector2::new(t.0, t.1))(i)
}

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

#[inline]
fn parse_objects_u32<'a, T, F: Fn(&'a [u8]) -> IResult<&'a [u8], T>>(
    parse_fn: F,
) -> impl Fn(&'a [u8]) -> IResult<&'a [u8], Vec<T>> {
    parse_objects(le_u32, parse_fn)
}

#[inline]
fn parse_objects_u16<'a, T, F: Fn(&'a [u8]) -> IResult<&'a [u8], T>>(
    parse_fn: F,
) -> impl Fn(&'a [u8]) -> IResult<&'a [u8], Vec<T>> {
    parse_objects(le_u16, parse_fn)
}

#[inline]
fn parse_objects_u8<'a, T, F: Fn(&'a [u8]) -> IResult<&'a [u8], T>>(
    parse_fn: F,
) -> impl Fn(&'a [u8]) -> IResult<&'a [u8], Vec<T>> {
    parse_objects(le_u8, parse_fn)
}

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
                }
                Err(nom::Err::Error(e)) => {
                    return Err(nom::Err::Error(E::append(
                        i,
                        nom::error::ErrorKind::Count,
                        e,
                    )));
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }

        Ok((input, res))
    }
}
