use nom::bytes::complete::{tag, take_till};
use nom::character::complete::{char, digit1, hex_digit1, line_ending, multispace1};
use nom::combinator::map;
use nom::combinator::map_res;
use nom::error::ParseError;
use nom::multi::many0;
use nom::sequence::{delimited, preceded, terminated};
use nom::IResult;
use struple::Struple;

use std::path::PathBuf;
use std::str::FromStr;

use crate::struple;

#[derive(Debug, PartialEq, Struple)]
pub struct ObjectIfo {
    pub index: u16,
    pub flag: u32,
    pub path: PathBuf,
}

impl ObjectIfo {
    pub fn parse<'a, E: ParseError<&'a str>>(i: &'a str) -> Result<Vec<Self>, nom::Err<E>> {
        preceded(tag("JMXVOBJI1000"), many0(Self::parse_single))(i).map(|(_, r)| r)
    }

    fn parse_single<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, Self, E> {
        terminated(
            struple((
                map_res(digit1, u16::from_str),
                preceded(
                    multispace1,
                    preceded(
                        tag("0x"),
                        map_res(hex_digit1, |s| u32::from_str_radix(s, 16)),
                    ),
                ),
                preceded(
                    multispace1,
                    map(
                        delimited(char('"'), take_till(|c| c == '"'), char('"')),
                        From::from,
                    ),
                ),
            )),
            line_ending,
        )(i)
    }
}

#[test]
fn objifo_single() {
    assert_eq!(
        ObjectIfo::parse_single::<nom::error::VerboseError<&str>>(
            "01057 0x00000000 \"res\\bldg\\oasis\\karakorm\\kara-obj-new\\oas_kara_obj02.bsr\"\r\n",
        ),
        Ok((
            "",
            ObjectIfo {
                index: 1057,
                flag: 0x0,
                path: PathBuf::from("res\\bldg\\oasis\\karakorm\\kara-obj-new\\oas_kara_obj02.bsr")
            }
        ))
    );
}
