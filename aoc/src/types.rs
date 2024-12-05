use std::ops::{Add, Sub};

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Dir {
    #[default]
    U,
    D,
    L,
    R,
}

impl Dir {
    pub fn as_quart(&self) -> u32 {
        match self {
            Dir::U => 0,
            Dir::R => 1,
            Dir::D => 2,
            Dir::L => 3,
        }
    }

    pub fn from_quart(quarts: u32) -> Self {
        match quarts % 4 {
            0 => Dir::U,
            1 => Dir::R,
            2 => Dir::D,
            3 => Dir::L,
            _ => unreachable!(),
        }
    }
}

impl Add<Dir> for Dir {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self::from_quart(self.as_quart().wrapping_add(other.as_quart()))
    }
}

impl Sub<Dir> for Dir {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self::from_quart(self.as_quart().wrapping_sub(other.as_quart()))
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum DiagDir {
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW,
}
