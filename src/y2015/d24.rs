use std::{collections::HashSet, io::{BufRead, Lines}, ops::ControlFlow, u64, vec};
use crate::solver;
use anyhow::{Ok, Result};

pub struct Problem(Vec<u64>);

impl<B: BufRead> TryFrom<Lines<B>> for Problem {
    type Error = anyhow::Error;

    fn try_from(value: Lines<B>) -> Result<Self, Self::Error> {
        Ok(Self(value.collect::<Result<Vec<_>, _>>()?.into_iter().map(|s| s.parse()).collect::<Result<Vec<_>, _>>()?))
    }
}

// assume `set` is ordered
fn find_subset_sum(set: &[u64], sum: u64, subset: &[u64]) -> Vec<Vec<u64>> {
    if sum == 0 {
        return vec![subset.to_vec()];
    }
    if set.is_empty() {
        return vec![];
    }

    match (0..set.len()).try_fold(vec![], |mut acc, i| {
        if set[i] > sum {
            return ControlFlow::Break(acc);
        }

        let mut subset = subset.to_vec();
        subset.push(set[i]);
        acc.extend(find_subset_sum(&set[(i+1)..], sum - set[i], &subset));
        ControlFlow::Continue(acc)
    }) {
        ControlFlow::Continue(v) => v,
        ControlFlow::Break(v) => v,
    }
}

#[derive(PartialEq, Eq)]
struct SubsetGroup {
    values: HashSet<u64>,
    product: u64,
}

impl From<HashSet<u64>> for SubsetGroup {
    fn from(value: HashSet<u64>) -> Self {
        Self {
            product: value.iter().fold(Some(1_u64), |acc, &i| acc.and_then(|acc| acc.checked_mul(i))).unwrap_or(u64::MAX),
            values: value,
        }
    }
}
impl FromIterator<u64> for SubsetGroup {
    fn from_iter<T: IntoIterator<Item = u64>>(iter: T) -> Self {
        iter.into_iter().collect::<HashSet<_>>().into()
    }
}

impl PartialOrd for SubsetGroup {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for SubsetGroup {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.values.len().cmp(&other.values.len()) {
            core::cmp::Ordering::Equal => self.product.cmp(&other.product),
            ord => ord,
        }
    }
}

impl solver::Solver for Problem {
    fn part_one(self: &Self) -> impl std::fmt::Display {
        let group_sum = self.0.iter().sum::<u64>()/3;
        let mut values = self.0.clone();
        values.sort();
        let mut groups = find_subset_sum(&values, group_sum, &[]).into_iter().map(|l| l.into_iter().collect::<SubsetGroup>()).collect::<Vec<_>>();
        groups.sort();

        groups.iter().find(|l| groups.iter().any(|ll| ll.values.intersection(&l.values).next().is_none())).unwrap().product
    }

    fn part_two(self: &Self) -> impl std::fmt::Display {
        let group_sum = self.0.iter().sum::<u64>()/4;
        let mut values = self.0.clone();
        values.sort();
        let mut groups = find_subset_sum(&values, group_sum, &[]).into_iter().map(|l| l.into_iter().collect::<SubsetGroup>()).collect::<Vec<_>>();
        groups.sort();

        groups.iter().find(|l| groups.iter().any(|ll| ll.values.intersection(&l.values).next().is_none() && groups.iter().any(|lll| lll.values.intersection(&l.values).next().is_none() && lll.values.intersection(&ll.values).next().is_none()))).unwrap().product
    }
}

#[cfg(test)]
mod tests {
    use solver::Solver;

    use super::*;

    #[test]
    fn part_one() -> anyhow::Result<()> {
        let pb: Problem = "1
2
3
4
5
7
8
9
10
11".as_bytes().lines().try_into()?;
        assert_eq!("99", format!("{}", pb.part_one()));
        Ok(())
    }
}
