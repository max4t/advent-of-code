use std::{cmp::Ordering, collections::HashMap, io::{BufRead, Lines}};
use crate::solver;
use anyhow::{anyhow, Result};
use itertools::Itertools;

struct Room {
    name: String,
    sector_id: u64,
    checksum: String,
}

impl Room {
    fn compute_checksum(&self) -> String {
        let mut count = HashMap::new();
        for c in self.name.chars().filter(|&a| a != '-') {
            count.entry(c).and_modify(|e| *e += 1).or_insert(1);
        }
        count.into_iter().sorted_unstable_by(|(a, arank), (b, brank)| {
            match arank.cmp(brank).reverse() {
                Ordering::Equal => a.cmp(b),
                o => o,
            }
        }).take(5).map(|(a, _)| a).collect::<String>()
    }

    fn is_checksum_valid(&self) -> bool {
        self.compute_checksum() == self.checksum
    }

    fn decrypt_name(&self) -> String {
        let shift = (self.sector_id % 26) as u8;
        self.name.chars().map(|c| match c {
            '-' => ' ',
            c => {
                let p = c as u8 - 'a' as u8;
                let p = p + shift;
                let p = p % 26;
                let p = p + 'a' as u8;
                p as char
            }
        }).collect::<String>()
    }
}

impl TryFrom<String> for Room {
    type Error = anyhow::Error;

    fn try_from(value: String) -> std::result::Result<Self, Self::Error> {
        let value = value.strip_suffix("]").ok_or_else(|| anyhow!("expected checksum suffix"))?;
        let (value, checksum) = value.split_once("[").ok_or_else(|| anyhow!("expected checksum prefix"))?;
        let (name, sector_id) = value.rsplit_once("-").ok_or_else(|| anyhow!("expected sector id"))?;
        let name = name.to_string();
        let sector_id = sector_id.parse()?;
        let checksum = checksum.to_string();
        Ok(Self { name, sector_id, checksum })
    }
}

pub struct Problem(Vec<Room>);

impl<B: BufRead> TryFrom<Lines<B>> for Problem {
    type Error = anyhow::Error;

    fn try_from(value: Lines<B>) -> Result<Self, Self::Error> {
        Ok(Self(value.map(|l| l?.try_into()).collect::<Result<_, _>>()?))
    }
}

impl solver::Solver for Problem {
    fn part_one(self: &Self) -> impl std::fmt::Display {
        self.0.iter().filter_map(|room| {
            room.is_checksum_valid().then_some(room.sector_id)
        }).sum::<u64>()
    }

    fn part_two(self: &Self) -> impl std::fmt::Display {
        self.0.iter().find_map(|room| {
            let name = room.decrypt_name();
            ["north", "pole", "object"].into_iter().all(|pat| name.contains(pat)).then_some(room.sector_id)
        }).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use solver::Solver;
    use test_case::test_case;

    use super::*;

    #[test_case("aaaaa-bbb-z-y-x-123[abxyz]", "123")]
    #[test_case("a-b-c-d-e-f-g-h-987[abcde]
not-a-real-room-404[oarel]", "1391")]
    #[test_case("totally-real-room-200[decoy]", "0")]
    fn part_one(example: &str, result: &str) -> anyhow::Result<()> {
        let pb: Problem = example.as_bytes().lines().try_into()?;
        assert_eq!(result, format!("{}", pb.part_one()));
        Ok(())
    }
}
