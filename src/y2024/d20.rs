use std::{collections::{HashMap, HashSet}, io::Stdin, iter};
use crate::solver;
use anyhow::{anyhow, Result};

pub struct Problem(HashSet<(usize, usize)>, (usize, usize), (usize, usize), usize);


fn parse(lines: Vec<String>) -> Result<(HashSet<(usize, usize)>, (usize, usize), (usize, usize))> {
    let a = lines.into_iter()
        .enumerate().flat_map(|(y, row)| row.chars().enumerate().map(|(x, c)| (x, y, c)).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let start_point = a.iter().find_map(|&(x, y, c)| if c == 'S' { Some((x, y)) } else { None }).ok_or_else(|| anyhow!("unable to find starting point"))?;
    let end_point = a.iter().find_map(|&(x, y, c)| if c == 'E' { Some((x, y)) } else { None }).ok_or_else(|| anyhow!("unable to find end point"))?;
    Ok((HashSet::from_iter(a.into_iter().filter_map(|(x, y, c)| if c != '#' { Some((x, y)) } else { None })), start_point, end_point))
}

impl TryFrom<Stdin> for Problem
{
    type Error = anyhow::Error;

    fn try_from(value: Stdin) -> Result<Self, Self::Error> {
        let (a, start_point, end_point) = parse(value.lines().collect::<Result<Vec<_>, _>>()?)?;
        Ok(Self(a, start_point, end_point, 100))
    }
}

impl solver::Solver for Problem {
    fn part_one(self: &Self) -> impl std::fmt::Display {
        let seq = iter::successors(Some((self.1, self.1)), |&(prev, (x, y))| {
            if (x, y) == self.2 {
                return None;
            }
            Some(((x, y), *[(x-1, y), (x+1, y), (x, y-1), (x, y+1)].iter().find(|&&next| self.0.contains(&next) && next != prev).unwrap()))
        }).map(|(_, pos)| pos).collect::<Vec<_>>();

        let track = HashMap::<_, _>::from_iter(seq.into_iter().enumerate().map(|(i, pos)| (pos, i)));

        let cheats = track.iter().flat_map(|(&(x, y), &step)| {
            let mut reduc = vec![];
            if !track.contains_key(&(x+1, y)) {
                if let Some(&next_step) = track.get(&(x+2, y)) {
                    if next_step > step {
                        reduc.push(next_step - step - 2);
                    }
                }
            }
            if !track.contains_key(&(x, y+1)) {
                if let Some(&next_step) = track.get(&(x, y+2)) {
                    if next_step > step {
                        reduc.push(next_step - step - 2);
                    }
                }
            }
            if x > 1 && !track.contains_key(&(x-1, y)) {
                if let Some(&next_step) = track.get(&(x-2, y)) {
                    if next_step > step {
                        reduc.push(next_step - step - 2);
                    }
                }
            }
            if y > 1 && !track.contains_key(&(x, y-1)) {
                if let Some(&next_step) = track.get(&(x, y-2)) {
                    if next_step > step {
                        reduc.push(next_step - step - 2);
                    }
                }
            }
            reduc
        });
        let mut cheat_freqs = HashMap::new();
        cheats.for_each(|v| {
            cheat_freqs.entry(v).and_modify(|v| *v += 1).or_insert(1);
        });
        cheat_freqs.iter().filter_map(|(&reduc, &count)| if reduc >= self.3 { Some(count) } else { None }).sum::<usize>()
    }

    fn part_two(self: &Self) -> impl std::fmt::Display {
        let seq = &iter::successors(Some((self.1, self.1)), |&(prev, (x, y))| {
            if (x, y) == self.2 {
                return None;
            }
            Some(((x, y), *[(x-1, y), (x+1, y), (x, y-1), (x, y+1)].iter().find(|&&next| self.0.contains(&next) && next != prev).unwrap()))
        }).map(|(_, pos)| pos).collect::<Vec<_>>();

        let shortcuts = seq[..].iter().enumerate().flat_map(|(i, &start)| {
            seq[(i+1)..].iter().enumerate().filter_map(move |(j, &end)| {
                let j = j+i+1;
                Some(end.0.abs_diff(start.0) + end.1.abs_diff(start.1))
                    .filter(|&v| v <= 20)
                    .and_then(|v| (j-i).checked_sub(v))
                    .filter(|&v| v >= self.3)
            })
        });

        let mut binding = HashMap::<usize, usize>::new();

        let cheat_freqs = shortcuts.fold(&mut binding, |acc, cost| {
            acc.entry(cost).and_modify(|v| *v += 1).or_insert(1);
            acc
        });

        cheat_freqs.iter().filter_map(|(&reduc, &count)| if reduc >= self.3 { Some(count) } else { None }).sum::<usize>()
    }
}

#[cfg(test)]
mod tests {
    use solver::Solver;

    use super::*;

    #[test]
    fn part_one() {
        let race_track =
"###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";
        let (a, start, end) = parse(race_track.split("\n").map(|l| l.to_owned()).collect::<Vec<_>>()).unwrap();
        assert_eq!("8", format!("{}", Problem(a, start, end, 12).part_one()));
    }

    #[test]
    fn part_two() {
        let race_track =
"###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";
        let (a, start, end) = parse(race_track.split("\n").map(|l| l.to_owned()).collect::<Vec<_>>()).unwrap();
        assert_eq!("41", format!("{}", Problem(a, start, end, 70).part_two()));
    }
}