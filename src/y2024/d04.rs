use std::io::Stdin;
use crate::solver;
use anyhow::Result;

pub struct Problem(Vec<Vec<char>>);

impl TryFrom<Stdin> for Problem
{
    type Error = anyhow::Error;

    fn try_from(value: Stdin) -> Result<Self, Self::Error> {
        let a = value.lines()
            .map::<Result<_, Self::Error>, _>(|s| Ok(s?.chars().collect::<Vec<_>>())).collect::<Result<Vec<_>, _>>()?;
        Ok(Self(a))
    }
}

impl solver::Solver for Problem {
    fn part_one(self: &Self) -> impl std::fmt::Display {
        let xs = self.0.iter().enumerate().flat_map(|(x, aa)| {
            aa.iter().enumerate().filter_map(move |(y, &aaa)| if aaa == 'X' { Some((x, y)) } else { None })
        }).collect::<Vec<_>>();
    
        xs.iter().flat_map(|&(x, y)| [(-1_isize, -1_isize), (-1, 0), (-1, 1), (0, 1), (1, 1), (1, 0), (1, -1), (0, -1)].iter().filter(move |(i, j)| {
            x.checked_add_signed(3*i).is_some_and(|l| l < self.0.len()) && y.checked_add_signed(3*j).is_some_and(|l| l < self.0[x].len())
        }).filter(move |&&(i, j)| {
            *self.0.get(x.checked_add_signed(i).unwrap()).and_then(|r| r.get(y.checked_add_signed(j).unwrap())).unwrap() == 'M' &&
                *self.0.get(x.checked_add_signed(2*i).unwrap()).and_then(|r| r.get(y.checked_add_signed(2*j).unwrap())).unwrap() == 'A' &&
                *self.0.get(x.checked_add_signed(3*i).unwrap()).and_then(|r| r.get(y.checked_add_signed(3*j).unwrap())).unwrap() == 'S'
        })).count()
    }

    fn part_two(self: &Self) -> impl std::fmt::Display {
        let xs = self.0.iter().enumerate().skip(1).take(self.0.len()-2).flat_map(|(x, aa)| {
            aa.iter().enumerate().skip(1).take(aa.len()-2).filter_map(move |(y, &aaa)| if aaa == 'A' { Some((x, y)) } else { None })
        }).collect::<Vec<_>>();
        xs.iter().filter(|&&(x, y)| {
            ((self.0[x-1][y-1] == 'M' && self.0[x+1][y+1] == 'S') || (self.0[x-1][y-1] == 'S' && self.0[x+1][y+1] == 'M')) &&
                ((self.0[x-1][y+1] == 'M' && self.0[x+1][y-1] == 'S') || (self.0[x-1][y+1] == 'S' && self.0[x+1][y-1] == 'M'))
        }).count()    
    }
}

#[cfg(test)]
mod tests {
    use solver::Solver;

    use super::*;

    #[test]
    fn part_one() {
        assert_eq!("18", format!("{}", Problem(vec![
            vec!['M', 'M', 'M', 'S', 'X', 'X', 'M', 'A', 'S', 'M'],
            vec!['M', 'S', 'A', 'M', 'X', 'M', 'S', 'M', 'S', 'A'],
            vec!['A', 'M', 'X', 'S', 'X', 'M', 'A', 'A', 'M', 'M'],
            vec!['M', 'S', 'A', 'M', 'A', 'S', 'M', 'S', 'M', 'X'],
            vec!['X', 'M', 'A', 'S', 'A', 'M', 'X', 'A', 'M', 'M'],
            vec!['X', 'X', 'A', 'M', 'M', 'X', 'X', 'A', 'M', 'A'],
            vec!['S', 'M', 'S', 'M', 'S', 'A', 'S', 'X', 'S', 'S'],
            vec!['S', 'A', 'X', 'A', 'M', 'A', 'S', 'A', 'A', 'A'],
            vec!['M', 'A', 'M', 'M', 'M', 'X', 'M', 'M', 'M', 'M'],
            vec!['M', 'X', 'M', 'X', 'A', 'X', 'M', 'A', 'S', 'X'],
        ]).part_one()));
    }

    #[test]
    fn part_two() {
        assert_eq!("9", format!("{}", Problem(vec![
            vec!['M', 'M', 'M', 'S', 'X', 'X', 'M', 'A', 'S', 'M'],
            vec!['M', 'S', 'A', 'M', 'X', 'M', 'S', 'M', 'S', 'A'],
            vec!['A', 'M', 'X', 'S', 'X', 'M', 'A', 'A', 'M', 'M'],
            vec!['M', 'S', 'A', 'M', 'A', 'S', 'M', 'S', 'M', 'X'],
            vec!['X', 'M', 'A', 'S', 'A', 'M', 'X', 'A', 'M', 'M'],
            vec!['X', 'X', 'A', 'M', 'M', 'X', 'X', 'A', 'M', 'A'],
            vec!['S', 'M', 'S', 'M', 'S', 'A', 'S', 'X', 'S', 'S'],
            vec!['S', 'A', 'X', 'A', 'M', 'A', 'S', 'A', 'A', 'A'],
            vec!['M', 'A', 'M', 'M', 'M', 'X', 'M', 'M', 'M', 'M'],
            vec!['M', 'X', 'M', 'X', 'A', 'X', 'M', 'A', 'S', 'X'],
        ]).part_two()));
    }
}
