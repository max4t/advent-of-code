use std::{collections::HashMap, io::{BufRead, Lines}};
use crate::solver;
use anyhow::{anyhow, Result};

#[derive(Clone, Copy, Debug, Default)]
struct Sue {
    children: Option<u64>,
    cats: Option<u64>,
    samoyeds: Option<u64>,
    pomeranians: Option<u64>,
    akitas: Option<u64>,
    vizslas: Option<u64>,
    goldfish: Option<u64>,
    trees: Option<u64>,
    cars: Option<u64>,
    perfumes: Option<u64>,
}

impl Sue {
    fn can_be(&self, other: Sue) -> bool {
        self.children.zip(other.children).is_none_or(|(a, b)| a == b) &&
        self.cats.zip(other.cats).is_none_or(|(a, b)| a == b) &&
        self.samoyeds.zip(other.samoyeds).is_none_or(|(a, b)| a == b) &&
        self.pomeranians.zip(other.pomeranians).is_none_or(|(a, b)| a == b) &&
        self.akitas.zip(other.akitas).is_none_or(|(a, b)| a == b) &&
        self.vizslas.zip(other.vizslas).is_none_or(|(a, b)| a == b) &&
        self.goldfish.zip(other.goldfish).is_none_or(|(a, b)| a == b) &&
        self.trees.zip(other.trees).is_none_or(|(a, b)| a == b) &&
        self.cars.zip(other.cars).is_none_or(|(a, b)| a == b) &&
        self.perfumes.zip(other.perfumes).is_none_or(|(a, b)| a == b)
    }
    fn can_be_range(&self, other: Sue) -> bool {
        self.children.zip(other.children).is_none_or(|(a, b)| a == b) &&
        self.cats.zip(other.cats).is_none_or(|(a, b)| a > b) &&
        self.samoyeds.zip(other.samoyeds).is_none_or(|(a, b)| a == b) &&
        self.pomeranians.zip(other.pomeranians).is_none_or(|(a, b)| a < b) &&
        self.akitas.zip(other.akitas).is_none_or(|(a, b)| a == b) &&
        self.vizslas.zip(other.vizslas).is_none_or(|(a, b)| a == b) &&
        self.goldfish.zip(other.goldfish).is_none_or(|(a, b)| a < b) &&
        self.trees.zip(other.trees).is_none_or(|(a, b)| a > b) &&
        self.cars.zip(other.cars).is_none_or(|(a, b)| a == b) &&
        self.perfumes.zip(other.perfumes).is_none_or(|(a, b)| a == b)
    }
}

impl TryFrom<&str> for Sue {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        let props = value.split(", ")
            .map(|s| s.split_once(": ").unwrap())
            .map(|(name, value)| anyhow::Ok((name.to_string(), value.parse::<u64>()?)))
            .collect::<Result<HashMap<_, _>, _>>()?;
        Ok(Self {
            children: props.get("children").cloned(),
            cats: props.get("cats").cloned(),
            samoyeds: props.get("samoyeds").cloned(),
            pomeranians: props.get("pomeranians").cloned(),
            akitas: props.get("akitas").cloned(),
            vizslas: props.get("vizslas").cloned(),
            goldfish: props.get("goldfish").cloned(),
            trees: props.get("trees").cloned(),
            cars: props.get("cars").cloned(),
            perfumes: props.get("perfumes").cloned(),
        })
    }
}

pub struct Problem(HashMap<usize, Sue>);

impl<B: BufRead> TryFrom<Lines<B>> for Problem {
    type Error = anyhow::Error;

    fn try_from(value: Lines<B>) -> Result<Self, Self::Error> {
        Ok(Self(value
            .collect::<Result<Vec<_>, _>>()?.into_iter()
            .map(|s| {
                let (name, c) = s.split_once(": ").ok_or_else(|| anyhow!("missing characteristics"))?;
                anyhow::Ok((name.strip_prefix("Sue ").ok_or_else(|| anyhow!("expected a Sue"))?.parse::<usize>()?, c.try_into()?))
            })
            .collect::<Result<_, _>>()?))
    }
}

impl solver::Solver for Problem {
    fn part_one(self: &Self) -> impl std::fmt::Display {
        self.0.iter().find_map(|(&id, sue)| {
            if sue.can_be(Sue {
                children: Some(3),
                cats: Some(7),
                samoyeds: Some(2),
                pomeranians: Some(3),
                akitas: Some(0),
                vizslas: Some(0),
                goldfish: Some(5),
                trees: Some(3),
                cars: Some(2),
                perfumes: Some(1),
            }) {
                Some(id)
            } else {
                None
            }
        }).unwrap()
    }

    fn part_two(self: &Self) -> impl std::fmt::Display {
        self.0.iter().find_map(|(&id, sue)| {
            if sue.can_be_range(Sue {
                children: Some(3),
                cats: Some(7),
                samoyeds: Some(2),
                pomeranians: Some(3),
                akitas: Some(0),
                vizslas: Some(0),
                goldfish: Some(5),
                trees: Some(3),
                cars: Some(2),
                perfumes: Some(1),
            }) {
                Some(id)
            } else {
                None
            }
        }).unwrap()
    }
}

#[cfg(test)]
mod tests {
}
