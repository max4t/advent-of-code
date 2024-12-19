use std::{collections::HashMap, io::Stdin, ops::{Add, Sub}};
use crate::solver;
use anyhow::Result;

#[derive(PartialEq, Copy, Clone)]
enum Element {
    Empty,
    Wall,
    Box,
    Robot,
}
#[derive(PartialEq, Copy, Clone)]
enum Move {
    Up,
    Right,
    Down,
    Left,
}

impl<T> Add<Move> for (T, T)
where T: Sub<usize, Output = T> + Add<usize, Output = T>
{
    type Output = Self;

    fn add(self, rhs: Move) -> Self::Output {
        match rhs {
            Move::Up => (self.0, self.1 - 1),
            Move::Right => (self.0 + 1, self.1),
            Move::Down => (self.0, self.1 + 1),
            Move::Left => (self.0 - 1, self.1),
        }
    }
}

pub struct Problem(Vec<Vec<Element>>, Vec<Move>);

impl TryFrom<Stdin> for Problem
{
    type Error = anyhow::Error;

    fn try_from(value: Stdin) -> Result<Self, Self::Error> {
        let mut lines = value.lines();
        let a = lines.by_ref().take_while(|p| p.as_ref().is_ok_and(|s| !s.is_empty()))
            .map(|l| {
                let l = l?;
                anyhow::Ok(l.chars().map(|l| Ok(match l {
                    '#' => Element::Wall,
                    '.' => Element::Empty,
                    'O' => Element::Box,
                    '@' => Element::Robot,
                    _ => anyhow::bail!("unknown char"),
                })).collect::<Result<Vec<_>, _>>())?
            })
            .collect::<Result<Vec<_>, _>>()?;
        let b = lines
            .map(|l| {
                let l = l?;
                anyhow::Ok(l.chars().map(|l| Ok(match l {
                    '^' => Move::Up,
                    '>' => Move::Right,
                    'v' => Move::Down,
                    '<' => Move::Left,
                    _ => anyhow::bail!("unknown char"),
                })).collect::<Result<Vec<_>, _>>())?
            })
            .collect::<Result<Vec<_>, _>>()?
            .into_iter().flatten().collect::<Vec<_>>();
        Ok(Self(a, b))
    }
}

impl solver::Solver for Problem {
    fn part_one(self: &Self) -> impl std::fmt::Display {
        let Some(pos) = self.0.iter().enumerate().find_map(|(y, row)| row.iter().enumerate().find_map(|(x, c)| if *c == Element::Robot { Some((x, y)) } else { None })) else {
            panic!("unable to find robot");
        };

        let mut grid = HashMap::<_, _>::from_iter(self.0.iter().enumerate().flat_map(|(y, row)| row.iter().enumerate().map(move |(x, &el)| ((x, y), el))));
        let res = self.1.iter().fold((&mut grid, pos), |(grid, pos), &mov| {
            let mut poss = vec![pos];
            let mut next_pos = pos + mov;
            while grid.get(&next_pos).is_some_and(|&v| v == Element::Box) {
                poss.push(next_pos);
                next_pos = next_pos + mov;
            }

            if grid.get(&next_pos).is_some_and(|&v| v != Element::Empty) {
                return (grid, pos);
            }

            while let Some(pos) = poss.pop() {
                grid.insert(pos + mov, *grid.get(&pos).unwrap());
            }

            grid.insert(pos, Element::Empty);

            (grid, pos + mov)
        }).0;
        // for i in 0..10 {
        //     for j in 0..10 {
        //         print!("{}", match res.get(&(j, i)) {
        //             Some(Element::Empty) => '.',
        //             Some(Element::Wall) => '#',
        //             Some(Element::Box) => 'O',
        //             Some(Element::Robot) => '@',
        //             None => ' ',
        //         });
        //     }
        //     print!("\n");
        // }
        res.iter().filter_map(|(&(x, y), &el)| if el == Element::Box { Some(x + 100*y) } else { None }).sum::<usize>()
    }

    fn part_two(self: &Self) -> impl std::fmt::Display {
        0
    }
}

#[cfg(test)]
mod tests {
    use solver::Solver;

    use super::*;

    #[test]
    fn part_one() {
        assert_eq!("2028", format!("{}", Problem(vec![
            vec![Element::Wall, Element::Wall, Element::Wall, Element::Wall, Element::Wall, Element::Wall, Element::Wall, Element::Wall],
            vec![Element::Wall, Element::Empty, Element::Empty, Element::Box, Element::Empty, Element::Box, Element::Empty, Element::Wall],
            vec![Element::Wall, Element::Wall, Element::Robot, Element::Empty, Element::Box, Element::Empty, Element::Empty, Element::Wall],
            vec![Element::Wall, Element::Empty, Element::Empty, Element::Empty, Element::Box, Element::Empty, Element::Empty, Element::Wall],
            vec![Element::Wall, Element::Empty, Element::Wall, Element::Empty, Element::Box, Element::Empty, Element::Empty, Element::Wall],
            vec![Element::Wall, Element::Empty, Element::Empty, Element::Empty, Element::Box, Element::Empty, Element::Empty, Element::Wall],
            vec![Element::Wall, Element::Empty, Element::Empty, Element::Empty, Element::Empty, Element::Empty, Element::Empty, Element::Wall],
            vec![Element::Wall, Element::Wall, Element::Wall, Element::Wall, Element::Wall, Element::Wall, Element::Wall, Element::Wall],
        ], vec![
            Move::Left, Move::Up, Move::Up, Move::Right, Move::Right, Move::Right, Move::Down, Move::Down, Move::Left, Move::Down, Move::Right,
            Move::Right, Move::Down, Move::Left, Move::Left,
        ]).part_one()));
    }

    #[test]
    fn part_two() {
    }
}
