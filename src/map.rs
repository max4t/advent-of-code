use std::ops::{Add, Bound, Index, IndexMut, Range};
use std::hash::Hash;
use anyhow::bail;

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
pub struct Pt<T>(pub T, pub T);

impl Pt<usize> {
    pub fn saturating_add(self, dir: Direction, within: Self) -> Self {
        let Pt(x, y) = self;
        match dir {
            Direction::N if y == 0 => self,
            Direction::E if x == within.0 - 1 => self,
            Direction::S if y == within.1 - 1 => self,
            Direction::W if x == 0 => self,
            _ => self + dir,
        }
    }
}

impl<T> From<(T, T)> for Pt<T> {
    fn from((a, b): (T, T)) -> Self {
        Pt(a, b)
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Side {
    Left,
    Right,
}

#[derive(Clone, Copy, Debug)]
pub enum Direction {
    N,
    E,
    S,
    W,
}

impl Add<Side> for Direction {
    type Output = Direction;

    fn add(self, rhs: Side) -> Self::Output {
        match self {
            Direction::N => match rhs {
                Side::Left => Direction::W,
                Side::Right => Direction::E,
            },
            Direction::E => match rhs {
                Side::Left => Direction::N,
                Side::Right => Direction::S,
            },
            Direction::S => match rhs {
                Side::Left => Direction::E,
                Side::Right => Direction::W,
            },
            Direction::W => match rhs {
                Side::Left => Direction::S,
                Side::Right => Direction::N,
            },
        }
    }
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
    ($($t:ty, unsigned = $ut:ty)*) => ($(
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
        impl Add<Self> for Pt<$t> {
            type Output = Self;

            #[inline]
            fn add(self, rhs: Self) -> Self::Output {
                let Self(x, y) = self;
                let Self(x1, y1) = rhs;
                Self(x+x1, y+y1)
            }
        }

        impl Pt<$t> {
            #[allow(dead_code)]
            pub fn manhattan_distance(self, other: Self) -> $ut {
                self.0.abs_diff(other.0) + self.1.abs_diff(other.1)
            }
        }
    )*)
}

add_impl! { usize, unsigned = usize u8, unsigned = u8 u16, unsigned = u16 u32, unsigned = u32 u64, unsigned = u64 u128, unsigned = u128 isize, unsigned = usize i8, unsigned = u8 i16, unsigned = u16 i32, unsigned = u32 i64, unsigned = u64 i128, unsigned = u128 }

#[derive(Clone, Debug)]
pub struct Grid<T> {
    size: Pt<usize>,
    map: Vec<Vec<T>>,
}

impl<T> Grid<T> {
    pub fn size(&self) -> (usize, usize) {
        (self.size.0, self.size.1)
    }
}

impl<T: Copy> Grid<T> {
    pub fn new<const W: usize, const H: usize>(init: T) -> Self {
        Self::new_dyn(W, H, init)
    }
    pub fn new_dyn(w: usize, h: usize, init: T) -> Self {
        Self {
            size: Pt(w, h),
            map: vec![vec![init; w]; h],
        }
    }
}

impl<T, II: IntoIterator<Item = T> + Clone> FromIterator<II> for Grid<T> {
    fn from_iter<I: IntoIterator<Item = II>>(iter: I) -> Self {
        let mut iter = iter.into_iter().peekable();
        let peek: Option<&II> = iter.peek();
        if let Some(peek) = peek {
            let l = peek.clone().into_iter().count();
            let res = iter.map(|r: II| r.into_iter().collect::<Vec<_>>()).collect::<Vec<_>>();
            Self {
                size: Pt(l, res.len()),
                map: res,
            }
        } else {
            Self {
                size: Pt(0, 0),
                map: Vec::new(),
            }
        }
    }
}

// inspired by std::slice::into_range
fn into_range(
    len: usize,
    (start, end): (Bound<usize>, Bound<usize>),
) -> Range<usize> {
    let start = match start {
        Bound::Included(start) => start,
        Bound::Excluded(start) => start + 1,
        Bound::Unbounded => 0,
    }.max(0);

    let end = match end {
        Bound::Included(end) => end + 1,
        Bound::Excluded(end) => end,
        Bound::Unbounded => len,
    }.min(len);

    // Don't bother with checking `start < end` and `end <= len`
    // since these checks are handled by `Range` impls

    start..end
}

impl<T> Grid<T> {
    fn saturating_move(&self, pt: Pt<usize>, dir: Direction) -> Pt<usize> {
        pt.saturating_add(dir, self.size)
    }

    pub fn neigbours(&self, pos: Pt<usize>) -> impl Iterator<Item = &T> {
        self.neighbour_positions(pos).map(|Pt(x, y)| &self.map[y][x])
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.items(Pt(0, 0)..self.size)
    }
    // pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
    //     self.items_mut(Pt(0, 0)..self.size)
    // }
    pub fn positions(&self, pts: impl std::ops::RangeBounds<Pt<usize>>) -> impl Iterator<Item = Pt<usize>> {
        let rows = ((pts.start_bound().cloned().map(|b| b.1)), (pts.end_bound().cloned().map(|b| b.1)));
        let cols = ((pts.start_bound().cloned().map(|b| b.0)), (pts.end_bound().cloned().map(|b| b.0)));
        let rows = into_range(self.size.1, rows);
        let cols = into_range(self.size.0, cols);
        rows.flat_map(move |y| cols.clone().map(move |x| Pt(x, y)))
    }
    pub fn all_positions(&self) -> impl Iterator<Item = Pt<usize>> {
        self.positions(Pt(0, 0)..self.size)
    }
    pub fn neighbour_positions(&self, pos: Pt<usize>) -> impl Iterator<Item = Pt<usize>> {
        let top = self.saturating_move(pos, Direction::N);
        let bottom = self.saturating_move(pos, Direction::S);
        self.positions(self.saturating_move(top, Direction::W)..=self.saturating_move(bottom, Direction::E))
            .filter(move |&pt| pt != pos)
    }
    pub fn items(&self, pts: impl std::ops::RangeBounds<Pt<usize>>) -> impl Iterator<Item = &T> {
        self.positions(pts).map(|Pt(x, y)| &self.map[y][x])
    }
    pub fn items_mut(&mut self, pts: impl std::ops::RangeBounds<Pt<usize>>) -> impl Iterator<Item = &mut T> {
        // self.positions(pts).map(|Pt(x, y)| self.map.get_mut(y).unwrap().get_mut(x).unwrap())

        let rows = ((pts.start_bound().cloned().map(|b| b.1)), (pts.end_bound().cloned().map(|b| b.1)));
        let cols = ((pts.start_bound().cloned().map(|b| b.0)), (pts.end_bound().cloned().map(|b| b.0)));
        self.map[rows].iter_mut()
            .flat_map(move |row| row[cols].iter_mut())
    }
}

impl<T> Index<Pt<usize>> for Grid<T> {
    type Output = T;

    fn index(&self, Pt(x, y): Pt<usize>) -> &Self::Output {
        &self.map[y][x]
    }
}

impl<T> IndexMut<Pt<usize>> for Grid<T> {
    fn index_mut(&mut self, Pt(x, y): Pt<usize>) -> &mut Self::Output {
        &mut self.map[y][x]
    }
}
