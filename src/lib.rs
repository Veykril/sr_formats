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
pub mod jmxveff;
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

/// ttr_closure!{} <- r-a hint to use braces
macro_rules! tuple_to_record_closure {
    // -> injection
    // #[warn(unused_parens)]
    ($( $closed:ident ),+ -> $ident:ident { $field:ident $(,)? }) => {
        move |$field| $ident { $( $closed, )+ $field }
    };
    ($( $closed:ident ),+ -> $ident:ident { $( $field:ident ),+ $(,)? }) => {
        move |($( $field ),+)| $ident { $( $closed, )+ $( $field ),+ }
    };
    // #[warn(unused_parens)]
    ($ident:ident { $field:ident $(,)? }) => {
        | $field | $ident {  $field }
    };
    ($ident:ident { $( $field:ident ),+ $(,)? }) => {
        |($( $field ),+)| $ident { $( $field ),+ }
    };
}
use tuple_to_record_closure as ttr_closure;
