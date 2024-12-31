use std::{collections::HashMap, io::{BufRead, Lines}, iter::{self, Sum}, ops::{Add, Mul}};
use crate::solver;
use anyhow::{anyhow, Result};
use itertools::Itertools;

#[derive(Clone, Copy, Debug)]
struct Properties {
    capacity: i64,
    durability: i64,
    flavor: i64,
    texture: i64,
    calories: i64,
}

impl Mul<i64> for Properties {
    type Output = Self;

    fn mul(self, rhs: i64) -> Self::Output {
        Self {
            capacity: self.capacity*rhs,
            durability: self.durability*rhs,
            flavor: self.flavor*rhs,
            texture: self.texture*rhs,
            calories: self.calories*rhs,
        }
    }
}

impl Add for Properties {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            capacity: self.capacity + rhs.capacity,
            durability: self.durability + rhs.durability,
            flavor: self.flavor + rhs.flavor,
            texture: self.texture + rhs.texture,
            calories: self.calories + rhs.calories,
        }
    }
}

impl Sum for Properties {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self {
            capacity: 0,
            durability: 0,
            flavor: 0,
            texture: 0,
            calories: 0,
        }, |acc, n| {
            acc + n
        })
    }
}

impl TryFrom<&str> for Properties {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        let props = value.split(", ")
            .map(|s| s.split_once(" ").unwrap())
            .map(|(name, value)| anyhow::Ok((name.to_string(), value.parse::<i64>()?)))
            .collect::<Result<HashMap<_, _>, _>>()?;
        Ok(Self {
            texture: props["texture"],
            flavor: props["flavor"],
            durability: props["durability"],
            capacity: props["capacity"],
            calories: props["calories"],
        })
    }
}

struct Ingredient {
    props: Properties,
}

impl TryFrom<&str> for Ingredient {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        Ok(Self {
            props: value.try_into()?,
        })
    }
}

pub struct Problem(HashMap<String, Ingredient>);

impl<B: BufRead> TryFrom<Lines<B>> for Problem {
    type Error = anyhow::Error;

    fn try_from(value: Lines<B>) -> Result<Self, Self::Error> {
        Ok(Self(value
            .collect::<Result<Vec<_>, _>>()?.into_iter()
            .map(|s| {
                let (name, c) = s.split_once(": ").ok_or_else(|| anyhow!("missing characteristics"))?;
                anyhow::Ok((name.to_string(), c.try_into()?))
            })
            .collect::<Result<_, _>>()?))
    }
}

impl solver::Solver for Problem {
    fn part_one(self: &Self) -> impl std::fmt::Display { // TODO a bit long, can be optimized?
        let ingrerdients = self.0.values().collect::<Vec<_>>();
        iter::repeat_n(0..=100_i64, self.0.len())
            .multi_cartesian_product()
            .filter(|comb| comb.iter().sum::<i64>() == 100)
            .map(|comb| {
                let props = comb.iter().zip_eq(ingrerdients.iter())
                    .map(|(&c, &i)| i.props*c)
                    .sum::<Properties>();
                props.texture.max(0) * props.flavor.max(0) * props.durability.max(0) * props.capacity.max(0)
            }).max().unwrap()
    }

    fn part_two(self: &Self) -> impl std::fmt::Display { // TODO same as part one
        let ingrerdients = self.0.values().collect::<Vec<_>>();
        iter::repeat_n(0..=100_i64, self.0.len())
            .multi_cartesian_product()
            .filter(|comb| comb.iter().sum::<i64>() == 100)
            .filter_map(|comb| {
                let props = comb.iter().zip_eq(ingrerdients.iter())
                    .map(|(&c, &i)| i.props*c)
                    .sum::<Properties>();
                if props.calories > 500 {
                    return None;
                }
                Some(props.texture.max(0) * props.flavor.max(0) * props.durability.max(0) * props.capacity.max(0))
            }).max().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use solver::Solver;

    use super::*;

    #[test]
    fn part_one() -> anyhow::Result<()> {
        let pb: Problem = "Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3".as_bytes().lines().try_into()?;
        assert_eq!("62842880", format!("{}", pb.part_one()));
        Ok(())
    }

    #[test]
    fn part_two() -> anyhow::Result<()> {
        let pb: Problem = "Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3".as_bytes().lines().try_into()?;
        assert_eq!("57600000", format!("{}", pb.part_two()));
        Ok(())
    }
}
