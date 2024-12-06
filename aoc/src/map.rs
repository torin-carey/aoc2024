use crate::types::Dir;
use nom::{IResult, InputIter, InputLength, InputTakeAtPosition, AsChar, Slice};
use nom::error::{ParseError, FromExternalError, ErrorKind};
use nom::character::complete::newline;
use nom::bytes::complete::take_while1;
use nom::combinator::iterator;
use nom::sequence::terminated;
use smallvec::SmallVec;

use std::fmt;
use std::num::NonZeroUsize;
use std::ops::{Index, IndexMut, RangeFrom};

pub trait ParseTile: Copy + Eq {
    fn from_char(ch: char) -> Option<Self>;
}

pub trait DisplayTile: Copy + Eq {
    fn to_char(self) -> char;
}

impl ParseTile for char {
    fn from_char(ch: char) -> Option<Self> {
        match ch {
            '\n' => None,
            c => Some(c),
        }
    }
}

impl DisplayTile for char {
    fn to_char(self) -> char {
        self
    }
}

pub type Coords = (usize, usize);

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Map<T> {
    _width: Option<NonZeroUsize>,
    pub tiles: Vec<T>,
}

impl<T: Clone> Map<T> {
    pub fn new(width: usize, height: usize, tile: T) -> Self {
        Self {
            _width: NonZeroUsize::new(width),
            tiles: vec![tile; width*height],
        }
    }
}

impl<T> Map<T> {
    pub fn width(&self) -> usize {
        self._width.unwrap().get()
    }

    pub fn height(&self) -> usize {
        self.tiles.len() / self.width()
    }

    pub fn valid(&self, (x, y): Coords) -> bool {
        x < self.width() && y < self.height()
    }

    pub fn add(&self, (x, y): Coords, dir: Dir) -> Option<Coords> {
        let raw_coords = match dir {
            Dir::N  => (x                , y.wrapping_sub(1)),
            Dir::NE => (x.wrapping_add(1), y.wrapping_sub(1)),
            Dir::E  => (x.wrapping_add(1), y),
            Dir::SE => (x.wrapping_add(1), y.wrapping_add(1)),
            Dir::S  => (x                , y.wrapping_add(1)),
            Dir::SW => (x.wrapping_sub(1), y.wrapping_add(1)),
            Dir::W  => (x.wrapping_sub(1), y),
            Dir::NW => (x.wrapping_sub(1), y.wrapping_sub(1)),
        };
        if self.valid(raw_coords) {
            Some(raw_coords)
        } else {
            None
        }
    }

    pub fn idx(&self, (x, y): Coords) -> usize {
        x + y*self.width()
    }

    pub fn coords(&self, idx: usize) -> Coords {
        (idx % self.width(), idx / self.width())
    }

    pub fn neigh(&self, (x, y): Coords, adj: bool, diag: bool) -> SmallVec<[(Coords, Dir); 8]> {
        assert!(self.valid((x, y)));
        let mut vec = SmallVec::new();
        if adj && y > 0 {
            vec.push(((x, y-1), Dir::N));
        }
        if diag && y > 0 && x < self.width()-1 {
            vec.push(((x+1, y-1), Dir::NE));
        }
        if adj && x < self.width()-1 {
            vec.push(((x+1, y), Dir::E));
        }
        if diag && y < self.height()-1 && x < self.width()-1 {
            vec.push(((x+1, y+1), Dir::SE));
        }
        if adj && y < self.height()-1 {
            vec.push(((x, y+1), Dir::S));
        }
        if diag && y < self.height()-1 && x > 0 {
            vec.push(((x-1, y+1), Dir::SW));
        }
        if adj && x > 0 {
            vec.push(((x-1, y), Dir::W));
        }
        if diag && y > 0 && x > 0 {
            vec.push(((x-1, y-1), Dir::NW));
        }
        vec
    }

    pub fn iter(&self) -> MapIterator<'_, T> {
        MapIterator {
            map: self,
            idx: 0,
        }
    }
}

pub struct MapIterator<'a, T> {
    map: &'a Map<T>,
    idx: usize,
}

impl<'a, T: 'a> Iterator for MapIterator<'a, T> {
    type Item = (Coords, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(tile) = self.map.tiles.get(self.idx) {
            let coords = self.map.coords(self.idx);
            self.idx += 1;
            Some((coords, tile))
        } else {
            None
        }
    }
}

impl<'a, T> IntoIterator for &'a Map<T> {
    type Item = (Coords, &'a T);
    type IntoIter = MapIterator<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<T> Default for Map<T> {
    fn default() -> Self {
        Self {
            _width: None,
            tiles: Vec::new(),
        }
    }
}

impl<T> Index<usize> for Map<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.tiles[index]
    }
}

impl<T> IndexMut<usize> for Map<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.tiles[index]
    }
}

impl<T> Index<Coords> for Map<T> {
    type Output = T;

    fn index(&self, index: Coords) -> &Self::Output {
        &self.tiles[self.idx(index)]
    }
}

impl<T> IndexMut<Coords> for Map<T> {
    fn index_mut(&mut self, index: Coords) -> &mut Self::Output {
        let idx = self.idx(index);
        &mut self.tiles[idx]
    }
}

impl<T: DisplayTile> fmt::Display for Map<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..self.height() {
            if y > 0 {
                write!(f, "\n")?;
            }
            for x in 0..self.width() {
                write!(f, "{}", T::to_char(self[(x, y)]))?;
            }
        }
        Ok(())
    }
}

#[derive(thiserror::Error, Debug)]
#[error("expected a row of length {0}, got {1} characters")]
pub struct InvalidLength(NonZeroUsize, usize);

impl<T: ParseTile> Map<T> {
    pub fn push_row<I>(&mut self, input: I) -> Result<(), InvalidLength>
    where
        I: InputIter,
        <I as InputIter>::Item: AsChar,
    {
        let len = input.iter_elements().count();
        if let Some(exp) = self._width {
            if len != exp.get() {
                return Err(InvalidLength(exp, len));
            }
        } else {
            self._width = NonZeroUsize::new(len);
        }

        self.tiles.reserve(len);
        self.tiles.extend(input.iter_elements()
            .map(|e| T::from_char(e.as_char()).unwrap()));
        Ok(())
    }

    pub fn parse<I, E>(input: I) -> IResult<I, Self, E>
    where
        I: InputIter + InputLength + Clone,
        I: InputTakeAtPosition<Item = char> + Slice<RangeFrom<usize>>,
        <I as InputIter>::Item: AsChar,
        E: ParseError<I>,
        E: FromExternalError<I, InvalidLength>,
    {
        let mut map = Self::default();
        let mut iter = iterator(input,
            terminated(take_while1(|ch| T::from_char(ch).is_some()), newline));
        for row in &mut iter {
            map.push_row(row.clone())
                .map_err(|e| nom::Err::Error(E::from_external_error(
                    row, ErrorKind::TakeWhile1, e
                )))?
        }
        Ok((iter.finish()?.0, map))
    }

    pub fn parse_nom<I>(input: I) -> IResult<I, Self>
    where
        I: InputIter + InputLength + Clone,
        I: InputTakeAtPosition<Item = char> + Slice<RangeFrom<usize>>,
        <I as InputIter>::Item: AsChar,
        //I: nom::UnspecializedInput + nom::InputTake + nom::InputLength,
        //I: nom::Slice<std::ops::RangeFrom<usize>> + Clone,
        //I: nom::InputIter + InputLength,
    {
        Self::parse(input)
    }
}
