pub mod astar;
pub mod map;
pub mod types;

pub mod prelude {
    pub use super::map::{Coords, DisplayTile, Map, ParseTile};
    pub use super::types::Dir;
    #[cfg(feature = "derive")]
    pub use aoc_macros::{main, DisplayTile, ParseTile};

    pub use nom::branch::alt;
    pub use nom::bytes::complete::{tag, take_while_m_n};
    pub use nom::character::complete::{alpha0, alpha1, alphanumeric0, alphanumeric1, anychar};
    pub use nom::character::complete::{char as nom_char, line_ending, newline, space0, space1};
    pub use nom::character::complete::{
        i16 as nom_i16, i32 as nom_i32, i64 as nom_i64, i8 as nom_i8,
    };
    pub use nom::character::complete::{
        u16 as nom_u16, u32 as nom_u32, u64 as nom_u64, u8 as nom_u8,
    };
    pub use nom::combinator::{eof, iterator, map, map_res, opt, recognize, value};
    pub use nom::error::Error as NomError;
    pub use nom::multi::{many0, many1, separated_list0, separated_list1};
    pub use nom::sequence::{delimited, pair, preceded, separated_pair, terminated, tuple};
    pub use nom::IResult;

    pub use smallvec::SmallVec;

    pub use anyhow::{anyhow, Context, Error, Result};

    pub use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
    pub use std::fmt;
    pub use std::io::{stdin, Read};

    pub fn stdin_string() -> String {
        let mut buf = String::new();
        stdin().read_to_string(&mut buf).unwrap();
        buf
    }

    pub fn nom_err<T>(r: IResult<&str, T>) -> IResult<&str, T> {
        r
    }
}
