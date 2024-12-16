use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Dir {
    #[default]
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW,
}

impl Dir {
    pub fn as_quart(&self) -> usize {
        match self {
            Dir::N => 0,
            Dir::NE => 1,
            Dir::E => 2,
            Dir::SE => 3,
            Dir::S => 4,
            Dir::SW => 5,
            Dir::W => 6,
            Dir::NW => 7,
        }
    }

    pub fn from_quart(quarts: usize) -> Self {
        match quarts % 8 {
            0 => Dir::N,
            1 => Dir::NE,
            2 => Dir::E,
            3 => Dir::SE,
            4 => Dir::S,
            5 => Dir::SW,
            6 => Dir::W,
            7 => Dir::NW,
            _ => unreachable!(),
        }
    }

    pub fn add_coords(self, coords: (usize, usize), jump: usize) -> (usize, usize) {
        match self {
            Dir::N => (coords.0, coords.1 - jump),
            Dir::NE => (coords.0 + jump, coords.1 - jump),
            Dir::E => (coords.0 + jump, coords.1),
            Dir::SE => (coords.0 + jump, coords.1 + jump),
            Dir::S => (coords.0, coords.1 + jump),
            Dir::SW => (coords.0 - jump, coords.1 + jump),
            Dir::W => (coords.0 - jump, coords.1),
            Dir::NW => (coords.0 - jump, coords.1 - jump),
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

impl Mul<usize> for Dir {
    type Output = Self;

    fn mul(self, factor: usize) -> Self::Output {
        Self::from_quart(self.as_quart().wrapping_mul(factor))
    }
}

impl AddAssign<Dir> for Dir {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other
    }
}

impl SubAssign<Dir> for Dir {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other
    }
}

impl MulAssign<usize> for Dir {
    fn mul_assign(&mut self, other: usize) {
        *self = *self * other
    }
}
