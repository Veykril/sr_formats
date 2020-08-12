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

pub type VerboseError<'a> = nom::error::VerboseError<&'a [u8]>;
pub type NormalError<'a> = (&'a [u8], nom::error::ErrorKind);
