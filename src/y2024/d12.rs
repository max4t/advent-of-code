use std::{collections::{HashMap, HashSet}, io::Stdin, usize};
use crate::solver;
use anyhow::Result;

pub struct Problem(Vec<Vec<char>>);

impl TryFrom<Stdin> for Problem
{
    type Error = anyhow::Error;

    fn try_from(value: Stdin) -> Result<Self, Self::Error> {
        let a = value.lines()
            .map(|l| {
                let chars = l?.chars().collect::<Vec<_>>();
                anyhow::Ok(chars)
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self(a))
    }
}

fn visit_zone(grid: &mut HashMap<(usize, usize), char>, zone: &mut HashSet<(usize, usize)>, pos: (usize, usize), n: char) {
    if !grid.get(&pos).is_some_and(|&c| c == n) {
        return;
    }
    grid.remove(&pos);
    zone.insert(pos);

    if pos.0 > 0 {
        visit_zone(grid, zone, (pos.0-1, pos.1), n);
    }
    visit_zone(grid, zone, (pos.0+1, pos.1), n);
    if pos.1 > 0 {
        visit_zone(grid, zone, (pos.0, pos.1-1), n);
    }
    visit_zone(grid, zone, (pos.0, pos.1+1), n);
}

impl solver::Solver for Problem {
    fn part_one(self: &Self) -> impl std::fmt::Display {
        let zones = &mut HashMap::<char, Vec<HashSet<(usize, usize)>>>::new();

        let grid = &mut HashMap::from_iter(self.0.iter()
            .enumerate()
            .flat_map(|(x, row)| row.iter().enumerate().map(move |(y, &c)| ((x, y), c))));
        for x in 0..self.0.len() {
            for y in 0..self.0[x].len() {
                let Some(&c) = grid.get(&(x, y)) else {
                    continue;
                };

                let mut zone = &mut HashSet::<(usize, usize)>::new();
                visit_zone(grid, &mut zone, (x, y), c);
                zones.entry(c).and_modify(|zs| { zs.push(zone.clone()); }).or_insert_with(|| vec![zone.clone()]);
            }
        }

        zones.iter().flat_map(|(c, zones)| zones.iter().map(|zone| {
            let perimeter = zone.iter().map(|&loc| {
                let mut c = 0;
                if loc.0 == 0 || !zone.contains(&(loc.0-1, loc.1)) { c += 1 }
                if loc.1 == 0 || !zone.contains(&(loc.0, loc.1-1)) { c += 1 }
                if !zone.contains(&(loc.0+1, loc.1)) { c += 1 }
                if !zone.contains(&(loc.0, loc.1+1)) { c += 1 }
                c
            }).sum::<usize>();
            let area = zone.len();
            dbg!(*c, perimeter, area);
            perimeter*area
        })).sum::<usize>()
    }

    fn part_two(self: &Self) -> impl std::fmt::Display {
        let zones = &mut HashMap::<char, Vec<HashSet<(usize, usize)>>>::new();

        let grid = &mut HashMap::from_iter(self.0.iter()
            .enumerate()
            .flat_map(|(x, row)| row.iter().enumerate().map(move |(y, &c)| ((x, y), c))));
        for x in 0..self.0.len() {
            for y in 0..self.0[x].len() {
                let Some(&c) = grid.get(&(x, y)) else {
                    continue;
                };

                let mut zone = &mut HashSet::<(usize, usize)>::new();
                visit_zone(grid, &mut zone, (x, y), c);
                zones.entry(c).and_modify(|zs| { zs.push(zone.clone()); }).or_insert_with(|| vec![zone.clone()]);
            }
        }

        zones.iter().flat_map(|(c, zones)| zones.iter().map(|zone| {
            let sides = zone.iter().map(|&loc| {
                let mut c = 0;
                let has_top = loc.0 == 0 || !zone.contains(&(loc.0-1, loc.1));
                let has_left = loc.1 == 0 || !zone.contains(&(loc.0, loc.1-1));
                let has_bottom = !zone.contains(&(loc.0+1, loc.1));
                let has_right = !zone.contains(&(loc.0, loc.1+1));
                if has_top && has_left { c += 1 }
                if has_top && has_right { c += 1 }
                if has_bottom && has_left { c += 1 }
                if has_bottom && has_right { c += 1 }
                if !has_top && !has_left && !zone.contains(&(loc.0-1, loc.1-1)) { c += 1 }
                if !has_top && !has_right && !zone.contains(&(loc.0-1, loc.1+1)) { c += 1 }
                if !has_bottom && !has_left && !zone.contains(&(loc.0+1, loc.1-1)) { c += 1 }
                if !has_bottom && !has_right && !zone.contains(&(loc.0+1, loc.1+1)) { c += 1 }
                c
            }).sum::<usize>();
            let area = zone.len();
            dbg!(*c, sides, area);
            sides*area
        })).sum::<usize>()
    }
}

#[cfg(test)]
mod tests {
    use solver::Solver;

    use super::*;

    #[test]
    fn part_one() {
        assert_eq!("1930", format!("{}", Problem(vec![
            vec!['R', 'R', 'R', 'R', 'I', 'I', 'C', 'C', 'F', 'F'],
            vec!['R', 'R', 'R', 'R', 'I', 'I', 'C', 'C', 'C', 'F'],
            vec!['V', 'V', 'R', 'R', 'R', 'C', 'C', 'F', 'F', 'F'],
            vec!['V', 'V', 'R', 'C', 'C', 'C', 'J', 'F', 'F', 'F'],
            vec!['V', 'V', 'V', 'V', 'C', 'J', 'J', 'C', 'F', 'E'],
            vec!['V', 'V', 'I', 'V', 'C', 'C', 'J', 'J', 'E', 'E'],
            vec!['V', 'V', 'I', 'I', 'I', 'C', 'J', 'J', 'E', 'E'],
            vec!['M', 'I', 'I', 'I', 'I', 'I', 'J', 'J', 'E', 'E'],
            vec!['M', 'I', 'I', 'I', 'S', 'I', 'J', 'E', 'E', 'E'],
            vec!['M', 'M', 'M', 'I', 'S', 'S', 'J', 'E', 'E', 'E'],
        ]).part_one()));
    }

    #[test]
    fn part_two() {
        assert_eq!("1206", format!("{}", Problem(vec![
            vec!['R', 'R', 'R', 'R', 'I', 'I', 'C', 'C', 'F', 'F'],
            vec!['R', 'R', 'R', 'R', 'I', 'I', 'C', 'C', 'C', 'F'],
            vec!['V', 'V', 'R', 'R', 'R', 'C', 'C', 'F', 'F', 'F'],
            vec!['V', 'V', 'R', 'C', 'C', 'C', 'J', 'F', 'F', 'F'],
            vec!['V', 'V', 'V', 'V', 'C', 'J', 'J', 'C', 'F', 'E'],
            vec!['V', 'V', 'I', 'V', 'C', 'C', 'J', 'J', 'E', 'E'],
            vec!['V', 'V', 'I', 'I', 'I', 'C', 'J', 'J', 'E', 'E'],
            vec!['M', 'I', 'I', 'I', 'I', 'I', 'J', 'J', 'E', 'E'],
            vec!['M', 'I', 'I', 'I', 'S', 'I', 'J', 'E', 'E', 'E'],
            vec!['M', 'M', 'M', 'I', 'S', 'S', 'J', 'E', 'E', 'E'],
        ]).part_two()));
    }
}
