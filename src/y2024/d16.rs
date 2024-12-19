use std::{collections::HashSet, io::Stdin, ops::{Add, Sub}};
use crate::solver;
use anyhow::{anyhow, Result};

#[derive(PartialEq, Eq, Copy, Clone, Hash)]
enum Orientation {
    North,
    East,
    South,
    West,
}

impl Orientation {
    fn left(self) -> Self {
        match self {
            Self::North => Self::West,
            Self::East => Self::North,
            Self::South => Self::East,
            Self::West => Self::South,
        }
    }

    fn right(self) -> Self {
        match self {
            Self::North => Self::East,
            Self::East => Self::South,
            Self::South => Self::West,
            Self::West => Self::North,
        }
    }
}

impl<T> Add<Orientation> for (T, T)
where T: Sub<usize, Output = T> + Add<usize, Output = T>
{
    type Output = Self;

    fn add(self, rhs: Orientation) -> Self::Output {
        match rhs {
            Orientation::North => (self.0, self.1 - 1),
            Orientation::East => (self.0 + 1, self.1),
            Orientation::South => (self.0, self.1 + 1),
            Orientation::West => (self.0 - 1, self.1),
        }
    }
}

pub struct Problem(HashSet<(usize, usize)>, (usize, usize), (usize, usize));

impl TryFrom<Stdin> for Problem
{
    type Error = anyhow::Error;

    fn try_from(value: Stdin) -> Result<Self, Self::Error> {
        let a = value.lines()
            .map(|l| {
                let l = l?;
                anyhow::Ok(l.chars().collect::<Vec<_>>())
            })
            .collect::<Result<Vec<_>, _>>()?;
        let start_point = a.iter().enumerate()
            .find_map(|(y, row)| row.iter().enumerate().find_map(|(x, &c)| if c == 'S' { Some((x, y)) } else { None }))
            .ok_or_else(|| anyhow!("unable to find start"))?;
        let end_point = a.iter().enumerate()
            .find_map(|(y, row)| row.iter().enumerate().find_map(|(x, &c)| if c == 'E' { Some((x, y)) } else { None }))
            .ok_or_else(|| anyhow!("unable to find end"))?;
        let a = HashSet::from_iter(a.iter().enumerate()
            .flat_map(|(y, row)| row.iter().enumerate().filter_map(move |(x, &c)| if c == '.' || c == 'S' || c == 'E' { Some((x, y)) } else { None })));

        Ok(Self(a, start_point, end_point))
    }
}

impl solver::Solver for Problem {
    fn part_one(self: &Self) -> impl std::fmt::Display {
        let (_, cost) = pathfinding::directed::astar::astar(&(self.1, Orientation::East), |&(pos, orientation)| {
            let mut res = vec![];
            if self.0.contains(&(pos + orientation)) {
                res.push(((pos + orientation, orientation), 1));
            }
            if self.0.contains(&(pos + orientation.left())) {
                res.push(((pos, orientation.left()), 1000));
            }
            if self.0.contains(&(pos + orientation.right())) {
                res.push(((pos, orientation.right()), 1000));
            }
            res
        }, |&(pos, _)| self.2.0.abs_diff(pos.0) + self.2.1.abs_diff(pos.1), |&(pos, _)| pos == self.2).unwrap();
        cost
    }

    fn part_two(self: &Self) -> impl std::fmt::Display {
        let (paths, _) = pathfinding::directed::astar::astar_bag(&(self.1, Orientation::East), |&(pos, orientation)| {
            let mut res = vec![];
            if self.0.contains(&(pos + orientation)) {
                res.push(((pos + orientation, orientation), 1));
            }
            if self.0.contains(&(pos + orientation.left())) {
                res.push(((pos, orientation.left()), 1000));
            }
            if self.0.contains(&(pos + orientation.right())) {
                res.push(((pos, orientation.right()), 1000));
            }
            res
        }, |&(pos, _)| self.2.0.abs_diff(pos.0) + self.2.1.abs_diff(pos.1), |&(pos, _)| pos == self.2).unwrap();
        let mut tiles = paths.flatten().map(|(pos, _)| pos).collect::<Vec<_>>();
        tiles.sort();
        tiles.dedup();
        tiles.len()
    }
}

#[cfg(test)]
mod tests {
    use solver::Solver;

    use super::*;

    #[test]
    fn part_one() {
        assert_eq!("7036", format!("{}", Problem(HashSet::from([
            (1, 1 ), (2, 1 ), (3, 1 ), (4, 1 ), (5, 1 ), (6, 1 ), (7, 1 ),          (9, 1 ), (10, 1 ), (11, 1 ), (12, 1 ), (13, 1 ),
            (1, 2 ),          (3, 2 ),                            (7, 2 ),          (9, 2 ),                               (13, 2 ),
            (1, 3 ), (2, 3 ), (3, 3 ), (4, 3 ), (5, 3 ),          (7, 3 ),          (9, 3 ), (10, 3 ), (11, 3 ),           (13, 3 ),
            (1, 4 ),                            (5, 4 ),                                               (11, 4 ),           (13, 4 ),
            (1, 5 ),          (3, 5 ),          (5, 5 ), (6, 5 ), (7, 5 ), (8, 5 ), (9, 5 ), (10, 5 ), (11, 5 ),           (13, 5 ),
            (1, 6 ),          (3, 6 ),                                              (9, 6 ),                               (13, 6 ),
            (1, 7 ), (2, 7 ), (3, 7 ), (4, 7 ), (5, 7 ), (6, 7 ), (7, 7 ), (8, 7 ), (9, 7 ), (10, 7 ), (11, 7 ),           (13, 7 ),
                              (3, 8 ),          (5, 8 ),                                               (11, 8 ),           (13, 8 ),
            (1, 9 ), (2, 9 ), (3, 9 ),          (5, 9 ), (6, 9 ), (7, 9 ), (8, 9 ), (9, 9 ),           (11, 9 ),           (13, 9 ),
            (1, 10),          (3, 10),          (5, 10),                            (9, 10),           (11, 10),           (13, 10),
            (1, 11), (2, 11), (3, 11), (4, 11), (5, 11),          (7, 11), (8, 11), (9, 11),           (11, 11),           (13, 11),
            (1, 12),                            (5, 12),          (7, 12),          (9, 12),           (11, 12),           (13, 12),
            (1, 13), (2, 13), (3, 13),          (5, 13), (6, 13), (7, 13), (8, 13), (9, 13),           (11, 13), (12, 13), (13, 13),
        ]), (13, 1), (1, 13)).part_one()));
    }

    #[test]
    fn part_two() {
        assert_eq!("45", format!("{}", Problem(HashSet::from([
            (1, 1 ), (2, 1 ), (3, 1 ), (4, 1 ), (5, 1 ), (6, 1 ), (7, 1 ),          (9, 1 ), (10, 1 ), (11, 1 ), (12, 1 ), (13, 1 ),
            (1, 2 ),          (3, 2 ),                            (7, 2 ),          (9, 2 ),                               (13, 2 ),
            (1, 3 ), (2, 3 ), (3, 3 ), (4, 3 ), (5, 3 ),          (7, 3 ),          (9, 3 ), (10, 3 ), (11, 3 ),           (13, 3 ),
            (1, 4 ),                            (5, 4 ),                                               (11, 4 ),           (13, 4 ),
            (1, 5 ),          (3, 5 ),          (5, 5 ), (6, 5 ), (7, 5 ), (8, 5 ), (9, 5 ), (10, 5 ), (11, 5 ),           (13, 5 ),
            (1, 6 ),          (3, 6 ),                                              (9, 6 ),                               (13, 6 ),
            (1, 7 ), (2, 7 ), (3, 7 ), (4, 7 ), (5, 7 ), (6, 7 ), (7, 7 ), (8, 7 ), (9, 7 ), (10, 7 ), (11, 7 ),           (13, 7 ),
                              (3, 8 ),          (5, 8 ),                                               (11, 8 ),           (13, 8 ),
            (1, 9 ), (2, 9 ), (3, 9 ),          (5, 9 ), (6, 9 ), (7, 9 ), (8, 9 ), (9, 9 ),           (11, 9 ),           (13, 9 ),
            (1, 10),          (3, 10),          (5, 10),                            (9, 10),           (11, 10),           (13, 10),
            (1, 11), (2, 11), (3, 11), (4, 11), (5, 11),          (7, 11), (8, 11), (9, 11),           (11, 11),           (13, 11),
            (1, 12),                            (5, 12),          (7, 12),          (9, 12),           (11, 12),           (13, 12),
            (1, 13), (2, 13), (3, 13),          (5, 13), (6, 13), (7, 13), (8, 13), (9, 13),           (11, 13), (12, 13), (13, 13),
        ]), (13, 1), (1, 13)).part_two()));
    }
}
