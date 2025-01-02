use std::io::{BufRead, Lines};
use crate::{map::{Grid, Pt}, solver};
use anyhow::{anyhow, Result};


pub struct Problem(Grid<bool>, usize);

impl<B: BufRead> TryFrom<Lines<B>> for Problem {
    type Error = anyhow::Error;

    fn try_from(value: Lines<B>) -> Result<Self, Self::Error> {
        (value, 100).try_into()
    }
}

impl<B: BufRead> TryFrom<(Lines<B>, usize)> for Problem {
    type Error = anyhow::Error;

    fn try_from(value: (Lines<B>, usize)) -> Result<Self, Self::Error> {
        let grid = value.0
            .collect::<Result<Vec<_>, _>>()?.into_iter()
            .map(|s| {
                anyhow::Ok(s.chars().map(|c| (c == '#').then(|| true).or_else(|| (c == '.').then(|| false)).ok_or_else(|| anyhow!("unknown element"))).collect::<Result<Vec<_>, _>>()?)
            })
            .collect::<Result<Grid<_>, _>>()?;
        Ok(Self(grid, value.1))
    }
}

impl solver::Solver for Problem {
    fn part_one(self: &Self) -> impl std::fmt::Display {
        let mut grid = self.0.clone();
        let mut tmp_grid = Grid::new_dyn(grid.size().0, grid.size().1, false);
        (0..self.1).for_each(|_| {
            grid.all_positions().for_each(|pt| {
                let neigh = grid.neigbours(pt).filter(|&&v| v).count();
                tmp_grid[pt] = if grid[pt] {
                    neigh == 2 || neigh == 3
                } else {
                    neigh == 3
                };
            });
            std::mem::swap(&mut grid, &mut tmp_grid);
        });
        grid.iter().filter(|&&v| v).count()
    }

    fn part_two(self: &Self) -> impl std::fmt::Display {
        let mut grid = self.0.clone();
        let (w, h) = grid.size();
        let mut tmp_grid = Grid::new_dyn(w, h, false);
        let broken_lights = [Pt(0, 0), Pt(w-1, 0), Pt(0, h-1), Pt(w-1, h-1)];
        broken_lights.iter().for_each(|&p| { grid[p] = true; });
        (0..self.1).for_each(|_| {
            grid.all_positions().for_each(|pt| {
                let neigh = grid.neigbours(pt).filter(|&&v| v).count();
                tmp_grid[pt] = if grid[pt] {
                    neigh == 2 || neigh == 3
                } else {
                    neigh == 3
                };
            });
            std::mem::swap(&mut grid, &mut tmp_grid);
            broken_lights.iter().for_each(|&p| { grid[p] = true; });
        });
        grid.iter().filter(|&&v| v).count()
    }
}

#[cfg(test)]
mod tests {
    use solver::Solver;

    use super::*;

    #[test]
    fn part_one() -> anyhow::Result<()> {
        let pb: Problem = (".#.#.#
...##.
#....#
..#...
#.#..#
####..".as_bytes().lines(), 4).try_into()?;
        assert_eq!("4", format!("{}", pb.part_one()));
        Ok(())
    }

    #[test]
    fn part_two() -> anyhow::Result<()> {
        let pb: Problem = (".#.#.#
...##.
#....#
..#...
#.#..#
####..".as_bytes().lines(), 5).try_into()?;
        assert_eq!("17", format!("{}", pb.part_two()));
        Ok(())
    }
}
