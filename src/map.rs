use std::ops::Add;
use std::hash::Hash;
use anyhow::bail;

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
pub struct Pt<T>(pub T, pub T);

impl<T> From<(T, T)> for Pt<T> {
    fn from((a, b): (T, T)) -> Self {
        Pt(a, b)
    }
}

#[derive(Clone, Copy)]
pub enum Direction {
    N,
    E,
    S,
    W,
}

impl TryFrom<char> for Direction {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            '^' => Self::N,
            '>' => Self::E,
            'v' => Self::S,
            '<' => Self::W,
            _ => bail!("unknown direction"),
        })
    }
}

macro_rules! add_impl {
    ($($t:ty)*) => ($(
        impl Add<Direction> for Pt<$t> {
            type Output = Self;

            #[inline]
            fn add(self, rhs: Direction) -> Self::Output {
                let Self(x, y) = self;
                match rhs {
                    Direction::N => Self(x, y-1),
                    Direction::E => Self(x+1, y),
                    Direction::S => Self(x, y+1),
                    Direction::W => Self(x-1, y),
                }
            }
        }
    )*)
}

add_impl! { usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 }

pub struct Grid<T> {
    size: Pt<usize>,
    map: Vec<Vec<T>>,
}

impl<T: Copy> Grid<T> {
    pub fn new<const W: usize, const H: usize>(init: T) -> Self {
        Self {
            size: Pt(W, H),
            map: vec![vec![init; W]; H],
        }
    }
}

impl<T> Grid<T> {
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.items(Pt(0, 0)..self.size)
    }
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.items_mut(Pt(0, 0)..self.size)
    }
    pub fn items(&self, pts: impl std::ops::RangeBounds<Pt<usize>>) -> impl Iterator<Item = &T> {
        let rows = ((pts.start_bound().cloned().map(|b| b.1)), (pts.end_bound().cloned().map(|b| b.1)));
        let cols = ((pts.start_bound().cloned().map(|b| b.0)), (pts.end_bound().cloned().map(|b| b.0)));
        self.map[rows].iter()
            .flat_map(move |row| row[cols].iter())
    }
    pub fn items_mut(&mut self, pts: impl std::ops::RangeBounds<Pt<usize>>) -> impl Iterator<Item = &mut T> {
        let rows = ((pts.start_bound().cloned().map(|b| b.1)), (pts.end_bound().cloned().map(|b| b.1)));
        let cols = ((pts.start_bound().cloned().map(|b| b.0)), (pts.end_bound().cloned().map(|b| b.0)));
        self.map[rows].iter_mut()
            .flat_map(move |row| row[cols].iter_mut())
    }
}
