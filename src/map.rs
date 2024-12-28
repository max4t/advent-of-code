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
