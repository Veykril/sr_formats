use mint::{Vector2, Vector3, Vector4};
use nom::bytes::complete::{tag, take, take_till};
use nom::character::complete::{char, digit1, hex_digit1};
use nom::combinator::{flat_map, map, map_opt, map_res};
use nom::error::ParseError;
use nom::number::complete::{le_f32, le_u16, le_u32, le_u8};
use nom::sequence::{delimited, preceded, tuple};
use nom::{IResult, ToUsize};

use std::path::PathBuf;

#[allow(dead_code)]
#[track_caller]
pub fn dbg<'i, O: std::fmt::Debug>(
    mut f: impl FnMut(&'i [u8]) -> IResult<&'i [u8], O>,
) -> impl FnMut(&'i [u8]) -> IResult<&'i [u8], O> {
    let loc = std::panic::Location::caller();
    move |i| match f(i) {
        Ok((i, o)) => {
            eprintln!("{:?} {:?}", loc, o);
            Ok((i, o))
        },
        Err(e) => Err(e),
    }
}

pub mod complete {
    use nom::error::{make_error, ErrorKind};

    use super::*;

    pub fn take_fixed<const C: usize>(i: &[u8]) -> IResult<&[u8], [u8; C]> {
        take(C)(i).and_then(|(i, arr)| match <[u8; C]>::try_from(arr) {
            Ok(arr) => Ok((i, arr)),
            _ => Err(nom::Err::Failure(make_error(i, ErrorKind::Eof))),
        })
    }
}

pub mod text {
    use std::num::ParseIntError;
    use std::path::Path;

    use nom::error::FromExternalError;

    use super::*;

    pub fn parse_u32_hex_str<'i, E>(input: &'i str) -> IResult<&'i str, u32, E>
    where
        E: ParseError<&'i str> + FromExternalError<&'i str, ParseIntError>,
    {
        preceded(
            tag("0x"),
            map_res(hex_digit1, |s| u32::from_str_radix(s, 16)),
        )(input)
    }

    pub fn parse_u8_str<'i, E>(input: &'i str) -> IResult<&'i str, u8, E>
    where
        E: ParseError<&'i str> + FromExternalError<&'i str, ParseIntError>,
    {
        map_res(digit1, |s: &str| s.parse::<u8>())(input)
    }

    pub fn parse_u16_str<'i, E>(input: &'i str) -> IResult<&'i str, u16, E>
    where
        E: ParseError<&'i str> + FromExternalError<&'i str, ParseIntError>,
    {
        map_res(digit1, |s: &str| s.parse::<u16>())(input)
    }

    pub fn parse_quoted_str<'i, E>(input: &'i str) -> IResult<&'i str, &'i str, E>
    where
        E: ParseError<&'i str>,
    {
        delimited(char('"'), take_till(|c| c == '"'), char('"'))(input)
    }

    pub fn parse_quoted_string<'i, E>(input: &'i str) -> IResult<&'i str, Box<str>, E>
    where
        E: ParseError<&'i str>,
    {
        map(parse_quoted_str, From::from)(input)
    }

    pub fn parse_quoted_path_buf<'i, E>(input: &'i str) -> IResult<&'i str, Box<Path>, E>
    where
        E: ParseError<&'i str>,
    {
        map(map(parse_quoted_str, PathBuf::from), Into::into)(input)
    }
}
pub mod string {
    use std::borrow::Cow;
    use std::path::Path;

    use super::*;

    /// Reads a u32, then reads the amount of bytes specified by the u32 and parses it as a EUC_KR string encoded string
    #[inline]
    pub fn sized_string<'i>(i: &'i [u8]) -> IResult<&'i [u8], Box<str>> {
        map(flat_map(le_u32, take), |s| {
            match encoding_rs::EUC_KR.decode_without_bom_handling(s).0 {
                Cow::Borrowed(it) => it.into(),
                Cow::Owned(it) => it.into_boxed_str(),
            }
        })(i)
    }

    /// Reads a u16, then reads the amount of bytes specified by the u32 and parses it as a EUC_KR string encoded string
    #[inline]
    pub fn small_sized_string<'i>(i: &'i [u8]) -> IResult<&'i [u8], Box<str>> {
        map(flat_map(le_u16, take), |s| {
            match encoding_rs::EUC_KR.decode_without_bom_handling(s).0 {
                Cow::Borrowed(it) => it.into(),
                Cow::Owned(it) => it.into_boxed_str(),
            }
        })(i)
    }

    /// Reads a sized_string and turns it into a PathBuf
    #[inline]
    pub fn sized_path<'i>(i: &'i [u8]) -> IResult<&'i [u8], Box<Path>> {
        map(flat_map(le_u32, take), |s| {
            Box::from(match encoding_rs::EUC_KR.decode_without_bom_handling(s).0 {
                Cow::Borrowed(it) => Cow::Borrowed(it.as_ref()),
                Cow::Owned(it) => Cow::Owned(PathBuf::from(it)),
            })
        })(i)
    }

    /// Reads a sized_string and turns it into a PathBuf
    #[inline]
    pub fn small_sized_path<'i>(i: &'i [u8]) -> IResult<&'i [u8], Box<Path>> {
        map(flat_map(le_u16, take), |s| {
            Box::from(match encoding_rs::EUC_KR.decode_without_bom_handling(s).0 {
                Cow::Borrowed(it) => Cow::Borrowed(it.as_ref()),
                Cow::Owned(it) => Cow::Owned(PathBuf::from(it)),
            })
        })(i)
    }

    #[inline]
    pub fn fixed_string<'i, const LEN: usize>(data: &'i [u8]) -> IResult<&'i [u8], Box<str>> {
        map(take(LEN), move |bytes: &'i [u8]| {
            let len = bytes.iter().position(|&b| b == 0).unwrap_or(LEN);
            match encoding_rs::EUC_KR
                .decode_without_bom_handling(&bytes[..len])
                .0
            {
                Cow::Borrowed(it) => it.into(),
                Cow::Owned(it) => it.into_boxed_str(),
            }
        })(data)
    }

    #[inline]
    pub fn fixed_path<'i, const LEN: usize>(data: &'i [u8]) -> IResult<&'i [u8], Box<Path>> {
        map(take(LEN), move |bytes: &'i [u8]| {
            let len = bytes.iter().position(|&b| b == 0).unwrap_or(LEN);
            Box::from(
                match encoding_rs::EUC_KR
                    .decode_without_bom_handling(&bytes[..len])
                    .0
                {
                    Cow::Borrowed(it) => Cow::Borrowed(it.as_ref()),
                    Cow::Owned(it) => Cow::Owned(PathBuf::from(it)),
                },
            )
        })(data)
    }

    #[inline]
    pub fn fixed_string_64<'i>(i: &'i [u8]) -> IResult<&'i [u8], Box<str>> {
        fixed_string::<64>(i)
    }

    #[inline]
    pub fn fixed_string_128<'i>(i: &'i [u8]) -> IResult<&'i [u8], Box<str>> {
        fixed_string::<128>(i)
    }

    #[inline]
    pub fn fixed_string_256<'i>(i: &'i [u8]) -> IResult<&'i [u8], Box<str>> {
        fixed_string::<256>(i)
    }
}

pub mod flags {
    use super::*;

    #[inline]
    pub fn flags_u16<'i, F, T>(from_bits: F) -> impl FnMut(&'i [u8]) -> IResult<&'i [u8], T>
    where
        F: FnMut(u16) -> Option<T>,
    {
        map_opt(le_u16, from_bits)
    }

    #[inline]
    pub fn flags_u32<'i, F, T>(from_bits: F) -> impl FnMut(&'i [u8]) -> IResult<&'i [u8], T>
    where
        F: FnMut(u32) -> Option<T>,
    {
        map_opt(le_u32, from_bits)
    }
}

pub mod number {
    use mint::RowMatrix4;

    use super::*;

    /// Reads a [f32; 6] array
    #[inline]
    pub fn vector6_f32<'i>(i: &'i [u8]) -> IResult<&'i [u8], [f32; 6]> {
        map(
            tuple((le_f32, le_f32, le_f32, le_f32, le_f32, le_f32)),
            |t| [t.0, t.1, t.2, t.3, t.4, t.5],
        )(i)
    }

    /// Reads a Vector3<f32>
    #[inline]
    pub fn vector4_f32<'i>(i: &'i [u8]) -> IResult<&'i [u8], Vector4<f32>> {
        map(tuple((le_f32, le_f32, le_f32, le_f32)), |t| Vector4 {
            x: t.0,
            y: t.1,
            z: t.2,
            w: t.3,
        })(i)
    }

    /// Reads a Vector3<f32>
    #[inline]
    pub fn vector3_f32<'i>(i: &'i [u8]) -> IResult<&'i [u8], Vector3<f32>> {
        map(tuple((le_f32, le_f32, le_f32)), |t| Vector3 {
            x: t.0,
            y: t.1,
            z: t.2,
        })(i)
    }

    /// Reads a Vector2<f32>
    #[inline]
    pub fn vector2_f32<'i>(i: &'i [u8]) -> IResult<&'i [u8], Vector2<f32>> {
        map(tuple((le_f32, le_f32)), |t| Vector2 { x: t.0, y: t.1 })(i)
    }

    #[rustfmt::skip]
    pub fn matrix4x4(i: &[u8]) -> IResult<&[u8], RowMatrix4<f32>> {
        let row = || tuple((le_f32, le_f32, le_f32, le_f32));
        map(
            tuple((row(), row(), row(), row())),
            |(
                (m00, m01, m02, m03),
                (m10, m11, m12, m13),
                (m20, m21, m22, m23),
                (m30, m31, m32, m33),
            )| {
                RowMatrix4::from([
                    m00, m01, m02, m03,
                    m10, m11, m12, m13,
                    m20, m21, m22, m23,
                    m30, m31, m32, m33,
                ])
            },
        )(i)
    }
}

pub mod multi {
    use nom::error::make_error;

    use super::*;
    /// Runs the `parse_fn` as many times as what the `count_fn` returns as a number.
    #[inline]
    pub fn parse_objects<'i, T, F, S, R>(
        mut count_fn: S,
        mut parse_fn: F,
    ) -> impl FnMut(&'i [u8]) -> IResult<&'i [u8], Box<[T]>>
    where
        F: FnMut(&'i [u8]) -> IResult<&'i [u8], T>,
        S: FnMut(&'i [u8]) -> IResult<&'i [u8], R>,
        R: ToUsize,
    {
        move |input: &[u8]| {
            let (input, c) = count_fn(input)?;
            count(&mut parse_fn, c.to_usize())(input)
        }
    }

    /// Reads a u32 and then runs `parse_fn` that many times.
    #[inline]
    pub fn parse_objects_u32<'i, T, F>(
        parse_fn: F,
    ) -> impl FnMut(&'i [u8]) -> IResult<&'i [u8], Box<[T]>>
    where
        F: FnMut(&'i [u8]) -> IResult<&'i [u8], T>,
    {
        parse_objects(le_u32, parse_fn)
    }

    /// Reads a u16 and then runs `parse_fn` that many times.
    #[inline]
    pub fn parse_objects_u16<'i, T, F>(
        parse_fn: F,
    ) -> impl FnMut(&'i [u8]) -> IResult<&'i [u8], Box<[T]>>
    where
        F: FnMut(&'i [u8]) -> IResult<&'i [u8], T>,
    {
        parse_objects(le_u16, parse_fn)
    }

    /// Reads a u8 and then runs `parse_fn` that many times.
    #[inline]
    pub fn parse_objects_u8<'i, T, F>(
        parse_fn: F,
    ) -> impl FnMut(&'i [u8]) -> IResult<&'i [u8], Box<[T]>>
    where
        F: FnMut(&'i [u8]) -> IResult<&'i [u8], T>,
    {
        parse_objects(le_u8, parse_fn)
    }

    pub fn count<I, O, F>(f: F, count: usize) -> impl FnMut(I) -> IResult<I, Box<[O]>>
    where
        I: Clone + PartialEq,
        F: FnMut(I) -> IResult<I, O>,
    {
        map(nom::multi::count(f, count), Vec::into_boxed_slice)
    }

    /// Runs f count times, while passing the iteration index to f
    pub fn count_indexed<I, O, F>(mut f: F, count: usize) -> impl FnMut(I) -> IResult<I, Box<[O]>>
    where
        I: Clone + PartialEq,
        F: FnMut(I, usize) -> IResult<I, O>,
    {
        move |i: I| {
            let mut input = i.clone();
            let mut res = Vec::with_capacity(count);

            for idx in 0..count {
                let input_ = input.clone();
                match f(input_, idx) {
                    Ok((i, o)) => {
                        res.push(o);
                        input = i;
                    },
                    Err(nom::Err::Error(e)) => {
                        return Err(nom::Err::Error(make_error(i, nom::error::ErrorKind::Count)));
                    },
                    Err(e) => {
                        return Err(e);
                    },
                }
            }

            Ok((input, res.into_boxed_slice()))
        }
    }
}
