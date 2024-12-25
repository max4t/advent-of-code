use core::panic;
use std::{io::Stdin, iter};
use crate::solver;
use anyhow::Result;
use itertools::Itertools;

pub struct Problem(Vec<String>);

impl TryFrom<Stdin> for Problem
{
    type Error = anyhow::Error;

    fn try_from(value: Stdin) -> Result<Self, Self::Error> {
        Ok(Self(value.lines().collect::<Result<Vec<_>, _>>()?))
    }
}

#[memoize::memoize]
fn path_between(a: (usize, usize), b: (usize, usize), exclude: (usize, usize)) -> Vec<String> {
    if a == b {
        return vec!["A".to_owned()];
    }
    let binding = Some(b.1.cmp(&a.1)).and_then(|v| match v {
        std::cmp::Ordering::Less => Some(((a.0, a.1-1), '^')),
        std::cmp::Ordering::Equal => None,
        std::cmp::Ordering::Greater => Some(((a.0, a.1+1), 'v')),
    });
    let next = Some(b.0.cmp(&a.0)).and_then(|v| match v {
        std::cmp::Ordering::Less => Some(((a.0-1, a.1), '<')),
        std::cmp::Ordering::Equal => None,
        std::cmp::Ordering::Greater => Some(((a.0+1, a.1), '>')),
    }).into_iter().chain(binding.into_iter());
    next.into_iter().filter(move |&(pos, _)| pos != exclude).flat_map(move |(pos, c)| {
        path_between(pos, b, exclude).into_iter().map(move |s| String::from(c) + s.as_str()).collect::<Vec<_>>()
    }).collect::<Vec<_>>()
}
fn num_keypad_pos(a: char) -> (usize, usize) {
    match a {
        '7' => (0, 0),
        '8' => (1, 0),
        '9' => (2, 0),
        '4' => (0, 1),
        '5' => (1, 1),
        '6' => (2, 1),
        '1' => (0, 2),
        '2' => (1, 2),
        '3' => (2, 2),
        '0' => (1, 3),
        'A' => (2, 3),
        _ => panic!("invalid num key"),
    }
}
fn dir_keypad_pos(a: char) -> (usize, usize) {
    match a {
        '^' => (1, 0),
        'A' => (2, 0),
        '<' => (0, 1),
        'v' => (1, 1),
        '>' => (2, 1),
        _ => panic!("invalid dir key"),
    }
}
fn num_keypad_path(a: char, b: char) -> Vec<String> {
    path_between(num_keypad_pos(a), num_keypad_pos(b), (0, 3)).into_iter().filter(|s| s.len() <= 3 || s.chars().dedup().count() <= 3).collect::<Vec<_>>()
}
fn dir_keypad_path(a: char, b: char) -> Vec<String> {
    path_between(dir_keypad_pos(a), dir_keypad_pos(b), (0, 0)).into_iter().filter(|s| s.len() <= 2 || s.chars().dedup().count() <= 3).collect::<Vec<_>>()
}
fn split_path(s: &str) -> Vec<(char, char)> {
    s.chars().scan('A', |state, c| {
        let next = (*state, c);
        *state = c;
        Some(next)
    }).collect::<Vec<_>>()
}

#[memoize::memoize]
fn compute_cost(s: String, lvl: usize) -> usize {
    if lvl == 0 {
        s.len()
    } else {
        split_path(&s).into_iter()
            .map(|(a, b)|
                dir_keypad_path(a, b).into_iter().map(|s| compute_cost(s, lvl-1)).min().unwrap_or(0)
            )
            .sum()
    }
}

impl solver::Solver for Problem {
    fn part_one(self: &Self) -> impl std::fmt::Display { // beurk that's slow...
        self.0.iter().map(|s| {
            let okok = iter::once(s).flat_map(|s| split_path(&s).into_iter()
                .map(|(a, b)| num_keypad_path(a, b))
                .reduce(|acc, next| {
                    acc.into_iter().cartesian_product(next).map(|(s1, s2)| s1 + &s2).collect::<Vec<_>>()
                })
                .unwrap_or_default()
            ).min_set_by_key(|s| s.len());

            okok.iter().map(|m| compute_cost(m.to_owned(), 2)).min().unwrap_or(0) * s.trim_end_matches("A").parse::<usize>().unwrap_or(0)
        }).sum::<usize>()
    }

    fn part_two(self: &Self) -> impl std::fmt::Display {
        self.0.iter().map(|s| {
            let okok = iter::once(s).flat_map(|s| split_path(&s).into_iter()
                .map(|(a, b)| num_keypad_path(a, b))
                .reduce(|acc, next| {
                    acc.into_iter().cartesian_product(next).map(|(s1, s2)| s1 + &s2).collect::<Vec<_>>()
                })
                .unwrap_or_default()
            ).min_set_by_key(|s| s.len());

            okok.into_iter().map(|m| compute_cost(m, 25)).min().unwrap_or(0) * s.trim_end_matches("A").parse::<usize>().unwrap_or(0)
        }).sum::<usize>()
    }
}

#[cfg(test)]
mod tests {
    use solver::Solver;

    use super::*;

    #[test]
    fn part_one() {
        assert_eq!("126384", format!("{}", Problem(vec![
            "029A".to_owned(),
            "980A".to_owned(),
            "179A".to_owned(),
            "456A".to_owned(),
            "379A".to_owned(),
        ]).part_one()));
    }

    #[test]
    fn part_two() {
        // assert_eq!("126384", format!("{}", Problem(vec![
        //     "029A".to_owned(),
        //     "980A".to_owned(),
        //     "179A".to_owned(),
        //     "456A".to_owned(),
        //     "379A".to_owned(),
        // ]).part_two()));
    }
}
