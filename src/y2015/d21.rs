use std::{io::{BufRead, Lines}, iter::Sum};
use crate::solver;
use anyhow::{anyhow, Result};
use itertools::{iproduct, Itertools};

#[derive(Clone)]
struct Stats {
    hp: u64,
    damage: u64,
    armor: u64,
}

impl Stats {
    fn attacks(&self, other: &mut Self) -> bool {
        let damage = self.damage.saturating_sub(other.armor).max(1);
        other.hp = other.hp.saturating_sub(damage);
        other.hp == 0
    }
}

impl<B: BufRead> TryFrom<Lines<B>> for Stats {
    type Error = anyhow::Error;

    fn try_from(mut value: Lines<B>) -> Result<Self, Self::Error> {
        Ok(Self {
            hp: value.next().ok_or_else(|| anyhow!("missing hp"))??.strip_prefix("Hit Points: ").ok_or_else(|| anyhow!("invalid hp"))?.parse::<u64>()?,
            damage: value.next().ok_or_else(|| anyhow!("missing damage"))??.strip_prefix("Damage: ").ok_or_else(|| anyhow!("invalid damage"))?.parse::<u64>()?,
            armor: value.next().ok_or_else(|| anyhow!("missing armor"))??.strip_prefix("Armor: ").ok_or_else(|| anyhow!("invalid armor"))?.parse::<u64>()?,
        })
    }
}

pub struct Problem(Stats);

#[derive(Debug)]
struct Equipment {
    cost: u64,
    damage: u64,
    armor: u64,
}

impl<'a> Sum<&'a Equipment> for Equipment {
    fn sum<I: Iterator<Item = &'a Equipment>>(iter: I) -> Self {
        iter.fold(Equipment { cost: 0, damage: 0, armor: 0 }, |mut acc, n| {
            acc.cost += n.cost;
            acc.damage += n.damage;
            acc.armor += n.armor;
            acc
        })
    }
}

impl<B: BufRead> TryFrom<Lines<B>> for Problem {
    type Error = anyhow::Error;

    fn try_from(value: Lines<B>) -> Result<Self, Self::Error> {
        Ok(Self(value.try_into()?))
    }
}

impl solver::Solver for Problem {
    fn part_one(self: &Self) -> impl std::fmt::Display {
        let weapons = vec![
            Equipment { cost: 8, damage: 4, armor: 0 },
            Equipment { cost: 10, damage: 5, armor: 0 },
            Equipment { cost: 25, damage: 6, armor: 0 },
            Equipment { cost: 40, damage: 7, armor: 0 },
            Equipment { cost: 74, damage: 8, armor: 0 },
        ];
        let armors = vec![
            Equipment { cost: 13, damage: 0, armor: 1 },
            Equipment { cost: 31, damage: 0, armor: 2 },
            Equipment { cost: 53, damage: 0, armor: 3 },
            Equipment { cost: 75, damage: 0, armor: 4 },
            Equipment { cost: 102, damage: 0, armor: 5 },
        ];
        let rings = vec![
            Equipment { cost: 25, damage: 1, armor: 0 },
            Equipment { cost: 50, damage: 2, armor: 0 },
            Equipment { cost: 100, damage: 3, armor: 0 },
            Equipment { cost: 20, damage: 0, armor: 1 },
            Equipment { cost: 40, damage: 0, armor: 2 },
            Equipment { cost: 80, damage: 0, armor: 3 },
        ];
        iproduct!(
            weapons.iter().powerset().filter(|s| s.len() == 1),
            armors.iter().powerset().filter(|s| s.len() <= 1),
            rings.iter().powerset().filter(|s| s.len() <= 2),
        )
            .map(|(a, b, c)| [a, b, c].concat().into_iter().sum::<Equipment>())
            .sorted_by_key(|eqs| eqs.cost)
            .find(|eqs| {
                let mut player = Stats {
                    hp: 100,
                    damage: eqs.damage,
                    armor: eqs.armor,
                };
                let mut boss = self.0.clone();
                loop {
                    if player.attacks(&mut boss) {
                        return true;
                    }

                    if boss.attacks(&mut player) {
                        return false;
                    }
                }
            }).map(|eqs| eqs.cost).unwrap()
    }

    fn part_two(self: &Self) -> impl std::fmt::Display {
        let weapons = vec![
            Equipment { cost: 8, damage: 4, armor: 0 },
            Equipment { cost: 10, damage: 5, armor: 0 },
            Equipment { cost: 25, damage: 6, armor: 0 },
            Equipment { cost: 40, damage: 7, armor: 0 },
            Equipment { cost: 74, damage: 8, armor: 0 },
        ];
        let armors = vec![
            Equipment { cost: 13, damage: 0, armor: 1 },
            Equipment { cost: 31, damage: 0, armor: 2 },
            Equipment { cost: 53, damage: 0, armor: 3 },
            Equipment { cost: 75, damage: 0, armor: 4 },
            Equipment { cost: 102, damage: 0, armor: 5 },
        ];
        let rings = vec![
            Equipment { cost: 25, damage: 1, armor: 0 },
            Equipment { cost: 50, damage: 2, armor: 0 },
            Equipment { cost: 100, damage: 3, armor: 0 },
            Equipment { cost: 20, damage: 0, armor: 1 },
            Equipment { cost: 40, damage: 0, armor: 2 },
            Equipment { cost: 80, damage: 0, armor: 3 },
        ];
        iproduct!(
            weapons.iter().powerset().filter(|s| s.len() == 1),
            armors.iter().powerset().filter(|s| s.len() <= 1),
            rings.iter().powerset().filter(|s| s.len() <= 2),
        )
            .map(|(a, b, c)| [a, b, c].concat().into_iter().sum::<Equipment>())
            .sorted_by_key(|eqs| eqs.cost)
            .rev()
            .find(|eqs| {
                let mut player = Stats {
                    hp: 100,
                    damage: eqs.damage,
                    armor: eqs.armor,
                };
                let mut boss = self.0.clone();
                loop {
                    if player.attacks(&mut boss) {
                        return false;
                    }

                    if boss.attacks(&mut player) {
                        return true;
                    }
                }
            }).map(|eqs| eqs.cost).unwrap()
    }
}

#[cfg(test)]
mod tests {
}
