pub mod map;
pub mod types;

pub mod prelude {
    pub use super::map::{ParseTile, DisplayTile, Map, Coords};
    pub use super::types::{Dir};
    #[cfg(feature = "derive")]
    pub use aoc_macros::{ParseTile, DisplayTile, main};

    pub use nom::IResult;
    pub use nom::error::Error as NomError;
    pub use nom::character::complete::{char as nom_char, newline, line_ending, space1, space0};
    pub use nom::character::complete::{anychar, alpha1, alpha0};
    pub use nom::character::complete::{u8 as nom_u8, u16 as nom_u16, u32 as nom_u32, u64 as nom_u64};
    pub use nom::character::complete::{i8 as nom_i8, i16 as nom_i16, i32 as nom_i32, i64 as nom_i64};
    pub use nom::bytes::complete::{tag, take_while_m_n};
    pub use nom::sequence::{delimited, preceded, terminated, tuple, pair, separated_pair};
    pub use nom::branch::alt;
    pub use nom::combinator::{eof, map, map_res, value, recognize, iterator, opt};
    pub use nom::multi::{many1, many0, separated_list1, separated_list0};

    pub use smallvec::SmallVec;

    pub use anyhow::{Error, Context, Result, anyhow};

    pub use std::collections::{HashSet, HashMap, BTreeSet, BTreeMap, VecDeque, BinaryHeap};
    pub use std::io::{Read, stdin};
    pub use std::fmt;

    pub fn stdin_string() -> String {
        let mut buf = String::new();
        stdin().read_to_string(&mut buf).unwrap();
        buf
    }

    pub fn nom_err<T>(r: IResult<&str, T>) -> IResult<&str, T> {
        r
    }
}
