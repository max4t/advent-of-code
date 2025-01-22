use std::io::{BufRead, Lines};
use crate::{game::{Game, Warrior, Wizard}, solver};
use anyhow::{anyhow, Result};

pub struct Problem(Warrior);

impl<B: BufRead> TryFrom<Lines<B>> for Problem {
    type Error = anyhow::Error;

    fn try_from(mut value: Lines<B>) -> Result<Self, Self::Error> {
        Ok(Self(Warrior::new(
            value.next().ok_or_else(|| anyhow!("missing hp"))??.strip_prefix("Hit Points: ").ok_or_else(|| anyhow!("invalid hp"))?.parse::<u64>()?,
            value.next().ok_or_else(|| anyhow!("missing damage"))??.strip_prefix("Damage: ").ok_or_else(|| anyhow!("invalid damage"))?.parse::<u64>()?,
        )))
    }
}

impl solver::Solver for Problem {
    fn part_one(self: &Self) -> impl std::fmt::Display {
        let game = Game {
            player: Wizard::new(50, 500),
            enemy: self.0.clone(),
        };
        game.process()
    }

    fn part_two(self: &Self) -> impl std::fmt::Display {
        0
    }
}

#[cfg(test)]
mod tests {
}
