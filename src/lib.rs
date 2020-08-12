#![warn(clippy::all)]
pub mod divisioninfo;
pub mod gmwpfort;
pub mod jmxv2dti;
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

mod parser_ext;

pub mod enums;
pub use enums::*;

#[cfg(feature = "serde")]
use serde::Serialize;

trait SrFile {
    type Input: ?Sized;
    type Output;

    fn nom_parse<'i, E: nom::error::ParseError<&'i Self::Input>>(
        i: &'i Self::Input,
    ) -> nom::IResult<&'i Self::Input, Self::Output, E>;

    fn parse<'i, E: nom::error::ParseError<&'i Self::Input>>(
        i: &'i Self::Input,
    ) -> Result<Self::Output, nom::Err<E>> {
        Self::nom_parse(i).map(|(_, r)| r)
    }

    fn parse_verbose<'i>(
        i: &'i Self::Input,
    ) -> Result<Self::Output, nom::Err<nom::error::VerboseError<&'i Self::Input>>> {
        Self::parse(i)
    }

    fn parse_simple(
        i: &Self::Input,
    ) -> Result<Self::Output, nom::Err<(&Self::Input, nom::error::ErrorKind)>> {
        Self::parse(i)
    }
}
