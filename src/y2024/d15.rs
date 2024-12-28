use std::{collections::{HashSet, VecDeque}, io::{BufRead, Lines}, ops::{Add, Sub}};
use crate::solver;
use anyhow::Result;
use std::hash::Hash;

trait MapPiece {
    fn is_movable(self) -> bool;
    fn positions(self) -> impl Iterator<Item = (isize, isize)>;
}

#[derive(PartialEq, Copy, Clone)]
enum Element {
    // Empty,
    Wall,
    Box,
    Robot,
}

impl MapPiece for Element {
    fn is_movable(self) -> bool {
        match self {
            Element::Wall => false,
            Element::Box | Element::Robot => true,
        }
    }

    fn positions(self) -> impl Iterator<Item = (isize, isize)> {
        [(0, 0)].into_iter()
    }
}

#[derive(PartialEq, Copy, Clone, Debug)]
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
            Move::Up => (self.0 - 1, self.1),
            Move::Right => (self.0, self.1 + 1),
            Move::Down => (self.0 + 1, self.1),
            Move::Left => (self.0, self.1 - 1),
        }
    }
}

#[derive(PartialEq, Copy, Clone)]
enum ScaledElement {
    // Empty,
    Wall,
    BoxLeft,
    BoxRight,
    Robot,
}

impl MapPiece for ScaledElement {
    fn is_movable(self) -> bool {
        match self {
            ScaledElement::Wall => false,
            ScaledElement::BoxLeft | ScaledElement::BoxRight | ScaledElement::Robot => true,
        }
    }

    fn positions(self) -> impl Iterator<Item = (isize, isize)> {
        match self {
            ScaledElement::Wall | ScaledElement::Robot => vec![(0, 0)].into_iter(),
            ScaledElement::BoxLeft => vec![(0, 0), (0, 1)].into_iter(),
            ScaledElement::BoxRight => vec![(0, -1), (0, 0)].into_iter(),
        }
        
    }
}

struct Map<T: PartialEq> {
    grid: Vec<Vec<Option<T>>>,
    pos: (usize, usize),
}

impl From<Map<Element>> for Map<ScaledElement> {
    fn from(Map { grid, pos: (x, y) }: Map<Element>) -> Self {
        Self {
            grid: grid.into_iter().map(|row| row.into_iter().flat_map(|e| match e {
                None => [None, None],
                Some(Element::Wall) => [Some(ScaledElement::Wall), Some(ScaledElement::Wall)],
                Some(Element::Box) => [Some(ScaledElement::BoxLeft), Some(ScaledElement::BoxRight)],
                Some(Element::Robot) => [Some(ScaledElement::Robot), None],
            }).collect()).collect(),
            pos: (x, y << 1),
        }
    }
}

trait Graph {
    type Item;

    fn next(&self, n: Self::Item) -> impl Iterator<Item = Self::Item>;
}

struct BFSIterator<N: Hash + Eq, G: Graph<Item = N>> {
    queue: VecDeque<N>,
    visited: HashSet<N>,
    graph: G,
}

impl<N: Hash + Eq + Copy, G: Graph<Item = N>> BFSIterator<N, G> {
    fn new(start: G::Item, graph: G) -> Self {
        Self {
            queue: vec![start].into(),
            visited: HashSet::from([start]),
            graph,
        }
    }
}

impl<N: Hash + Eq + Copy, G: Graph<Item = N>> Iterator for BFSIterator<N, G> {
    type Item = G::Item;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(n) = self.queue.pop_front() {
            self.queue.extend(self.graph.next(n).filter(|&i| self.visited.insert(i)));
            Some(n)
        } else {
            None
        }
    }
}

impl<T: PartialEq + Copy + MapPiece> Map<T> {
    fn movable(&self, dir: Move) -> Vec<(usize, usize)> {
        let mut ok = BFSIterator::new(self.pos, (&self.grid, dir))
            .map(|(x, y)| if self.grid[x][y].is_some_and(|l| !l.is_movable()) { Err(FixedItemError) } else { Ok((x, y)) })
            .collect::<Result<Vec<_>, _>>().unwrap_or_else(|_| vec![]);
        ok.sort_by_key(|&(x, y)| -> isize {
            let v: isize = match dir {
                Move::Up | Move::Down => x.try_into().unwrap(),
                Move::Right | Move::Left => y.try_into().unwrap(),
            };
            v * (match dir {
                Move::Up | Move::Left => 1_isize,
                Move::Right | Move::Down => -1,
            })
        });
        ok

    }

    fn step(&mut self, dir: Move) {
        let i = self.movable(dir);
        if !i.is_empty() {
            self.pos = self.pos + dir
        }
        for (x, y) in i {
            let (next_x, next_y) = (x, y) + dir;
            self.grid[next_x][next_y] = self.grid[x][y];
            self.grid[x][y] = None;
        }
    }
}

struct FixedItemError;

impl<T: Copy + PartialEq + MapPiece> Graph for (&Vec<Vec<Option<T>>>, Move) {
    type Item = (usize, usize);

    fn next(&self, (x, y): Self::Item) -> impl Iterator<Item = Self::Item> {
        let &(map, dir) = self;
        if map[x][y].as_ref().is_none_or(|l| !l.is_movable()) {
            return vec![].into_iter();
        }

        let next = (x, y) + dir;
        if let Some(el) = map[next.0][next.1] {
            el.positions()
                .map(|(dx, dy)| (next.0.saturating_add_signed(dx), next.1.saturating_add_signed(dy)))
                .collect::<Vec<_>>().into_iter()
        } else {
            vec![].into_iter()
        }
    }
}

pub struct Problem(Vec<Vec<Option<Element>>>, Vec<Move>);

impl<B: BufRead> TryFrom<Lines<B>> for Problem {
    type Error = anyhow::Error;

    fn try_from(value: Lines<B>) -> Result<Self, Self::Error> {
        let mut lines = value;
        let a = lines.by_ref().take_while(|p| p.as_ref().is_ok_and(|s| !s.is_empty()))
            .map(|l| {
                let l = l?;
                anyhow::Ok(l.chars().map(|l| Ok(match l {
                    '#' => Some(Element::Wall),
                    '.' => None,
                    'O' => Some(Element::Box),
                    '@' => Some(Element::Robot),
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
        let Some(pos) = self.0.iter().enumerate().find_map(|(y, row)| row.iter().enumerate().find_map(|(x, c)| if *c == Some(Element::Robot) { Some((x, y)) } else { None })) else {
            panic!("unable to find robot");
        };

        let mut map = Map {
            grid: self.0.clone(),
            pos,
        };
        self.1.iter().for_each(|&m| {
            map.step(m);
            // println!("{:?}", m);
            // for i in 0..8 {
            //     for j in 0..8 {
            //         print!("{}", match map.grid[i][j] {
            //             None => '.',
            //             Some(Element::Wall) => '#',
            //             Some(Element::Box) => 'O',
            //             Some(Element::Robot) => '@',
            //         });
            //     }
            //     print!("\n");
            // }
        });
        map.grid.iter().enumerate().flat_map(|(y, el)| el.iter().enumerate().filter_map(move |(x, el)| if el == &Some(Element::Box) { Some(x + 100*y) } else { None })).sum::<usize>()
    }

    fn part_two(self: &Self) -> impl std::fmt::Display {
        let Some(pos) = self.0.iter().enumerate().find_map(|(x, row)| row.iter().enumerate().find_map(|(y, c)| if *c == Some(Element::Robot) { Some((x, y)) } else { None })) else {
            panic!("unable to find robot");
        };

        let mut map: Map<ScaledElement> = Map {
            grid: self.0.clone(),
            pos,
        }.into();
        // for i in 0..7 {
        //     for j in 0..14 {
        //         print!("{}", match map.grid[i][j] {
        //             None => '.',
        //             Some(ScaledElement::Wall) => '#',
        //             Some(ScaledElement::BoxLeft) => '[',
        //             Some(ScaledElement::BoxRight) => ']',
        //             Some(ScaledElement::Robot) => '@',
        //         });
        //     }
        //     print!("\n");
        // }

        self.1.iter().for_each(|&m| {
            map.step(m);
            // println!("{:?}", m);
            // for i in 0..7 {
            //     for j in 0..14 {
            //         print!("{}", match map.grid[i][j] {
            //             None => '.',
            //             Some(ScaledElement::Wall) => '#',
            //             Some(ScaledElement::BoxLeft) => '[',
            //             Some(ScaledElement::BoxRight) => ']',
            //             Some(ScaledElement::Robot) => '@',
            //         });
            //     }
            //     print!("\n");
            // }
        });
        map.grid.iter().enumerate().flat_map(|(y, el)| el.iter().enumerate().filter_map(move |(x, el)| if el == &Some(ScaledElement::BoxLeft) { Some(x + 100*y) } else { None })).sum::<usize>()
    }
}

#[cfg(test)]
mod tests {
    use solver::Solver;

    use super::*;

    #[test]
    fn part_one() {
        assert_eq!("2028", format!("{}", Problem(vec![
            vec![Some(Element::Wall), Some(Element::Wall), Some(Element::Wall), Some(Element::Wall), Some(Element::Wall), Some(Element::Wall), Some(Element::Wall), Some(Element::Wall)],
            vec![Some(Element::Wall), None, None, Some(Element::Box), None, Some(Element::Box), None, Some(Element::Wall)],
            vec![Some(Element::Wall), Some(Element::Wall), Some(Element::Robot), None, Some(Element::Box), None, None, Some(Element::Wall)],
            vec![Some(Element::Wall), None, None, None, Some(Element::Box), None, None, Some(Element::Wall)],
            vec![Some(Element::Wall), None, Some(Element::Wall), None, Some(Element::Box), None, None, Some(Element::Wall)],
            vec![Some(Element::Wall), None, None, None, Some(Element::Box), None, None, Some(Element::Wall)],
            vec![Some(Element::Wall), None, None, None, None, None, None, Some(Element::Wall)],
            vec![Some(Element::Wall), Some(Element::Wall), Some(Element::Wall), Some(Element::Wall), Some(Element::Wall), Some(Element::Wall), Some(Element::Wall), Some(Element::Wall)],
        ], vec![
            Move::Left, Move::Up, Move::Up, Move::Right, Move::Right, Move::Right, Move::Down, Move::Down, Move::Left, Move::Down, Move::Right,
            Move::Right, Move::Down, Move::Left, Move::Left,
        ]).part_one()));
    }

    #[test]
    fn part_two() {
        assert_eq!("2028", format!("{}", Problem(vec![
            vec![Some(Element::Wall), Some(Element::Wall), Some(Element::Wall), Some(Element::Wall), Some(Element::Wall), Some(Element::Wall), Some(Element::Wall), Some(Element::Wall)],
            vec![Some(Element::Wall), None, None, Some(Element::Box), None, Some(Element::Box), None, Some(Element::Wall)],
            vec![Some(Element::Wall), Some(Element::Wall), Some(Element::Robot), None, Some(Element::Box), None, None, Some(Element::Wall)],
            vec![Some(Element::Wall), None, None, None, Some(Element::Box), None, None, Some(Element::Wall)],
            vec![Some(Element::Wall), None, Some(Element::Wall), None, Some(Element::Box), None, None, Some(Element::Wall)],
            vec![Some(Element::Wall), None, None, None, Some(Element::Box), None, None, Some(Element::Wall)],
            vec![Some(Element::Wall), None, None, None, None, None, None, Some(Element::Wall)],
            vec![Some(Element::Wall), Some(Element::Wall), Some(Element::Wall), Some(Element::Wall), Some(Element::Wall), Some(Element::Wall), Some(Element::Wall), Some(Element::Wall)],
        ], vec![
            Move::Left, Move::Up, Move::Up, Move::Right, Move::Right, Move::Right, Move::Down, Move::Down, Move::Left, Move::Down, Move::Right,
            Move::Right, Move::Down, Move::Left, Move::Left,
        ]).part_two()));
    }
}
