use std::{collections::HashMap, io::{BufRead, Lines}};
use crate::solver;
use anyhow::{bail, Ok, Result};

#[derive(PartialEq, Eq, Hash, Debug)]
enum Registry {
    A,
    B,
}

impl TryFrom<&str> for Registry {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "a" => Ok(Self::A),
            "b" => Ok(Self::B),
            e => bail!("invalid registry {e}"),
        }
    }
}

#[derive(Debug)]
enum Instruction {
    Hlf(Registry),
    Tpl(Registry),
    Inc(Registry),
    Jmp(isize),
    Jie(Registry, isize),
    Jio(Registry, isize),
}

impl TryFrom<&str> for Instruction {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let Some((instruction, args)) = value.split_once(" ") else {
            bail!("no instruction");
        };
        Ok(match instruction {
            "hlf" => Instruction::Hlf(args.try_into()?),
            "tpl" => Instruction::Tpl(args.try_into()?),
            "inc" => Instruction::Inc(args.try_into()?),
            "jmp" => Instruction::Jmp(args.parse()?),
            "jie" => {
                let Some((reg, jump)) = args.split_once(", ") else {
                    bail!("invalid arguments (expected 2 args)");
                };
                Instruction::Jie(reg.try_into()?, jump.parse()?)
            },
            "jio" => {
                let Some((reg, jump)) = args.split_once(", ") else {
                    bail!("invalid arguments (expected 2 args)");
                };
                Instruction::Jio(reg.try_into()?, jump.parse()?)
            },
            e => bail!("invalid instruction {e}"),
        })
    }
}

pub struct Problem(Vec<Instruction>, Registry);

impl<B: BufRead> TryFrom<Lines<B>> for Problem {
    type Error = anyhow::Error;

    fn try_from(value: Lines<B>) -> Result<Self, Self::Error> {
        (value, Registry::B).try_into()
    }
}

impl<B: BufRead> TryFrom<(Lines<B>, Registry)> for Problem {
    type Error = anyhow::Error;

    fn try_from((value, reg): (Lines<B>, Registry)) -> Result<Self, Self::Error> {
        Ok(Self(value.collect::<Result<Vec<_>, _>>()?.into_iter().map(|s| s.as_str().try_into()).collect::<Result<Vec<_>, _>>()?, reg))
    }
}

impl Problem {
    fn process_instructions(&self, registries: &mut HashMap<Registry, u64>) {
        let mut i = 0;
        while let Some(instruction) = self.0.get(i) {
            match instruction {
                Instruction::Hlf(registry) => {
                    *registries.get_mut(registry).unwrap() /= 2;
                    i += 1;
                },
                Instruction::Tpl(registry) => {
                    *registries.get_mut(registry).unwrap() *= 3;
                    i += 1;
                },
                Instruction::Inc(registry) => {
                    *registries.get_mut(registry).unwrap() += 1;
                    i += 1;
                },
                Instruction::Jmp(offset) => {
                    if let Some(offset) = i.checked_add_signed(*offset) {
                        i = offset;
                    } else {
                        break;
                    }
                },
                Instruction::Jie(registry, offset) => {
                    if registries[registry] % 2 == 0 {
                        if let Some(offset) = i.checked_add_signed(*offset) {
                            i = offset;
                        } else {
                            break;
                        }
                    } else {
                        i += 1;
                    }
                },
                Instruction::Jio(registry, offset) => {
                    if registries[registry] == 1 {
                        if let Some(offset) = i.checked_add_signed(*offset) {
                            i = offset;
                        } else {
                            break;
                        }
                    } else {
                        i += 1;
                    }
                },
            }
        }
    }
}

impl solver::Solver for Problem {
    fn part_one(self: &Self) -> impl std::fmt::Display {
        let mut registries = HashMap::from([(Registry::A, 0_u64), (Registry::B, 0)]);
        self.process_instructions(&mut registries);
        registries[&self.1]
    }

    fn part_two(self: &Self) -> impl std::fmt::Display {
        let mut registries = HashMap::from([(Registry::A, 1_u64), (Registry::B, 0)]);
        self.process_instructions(&mut registries);
        registries[&self.1]
    }
}

#[cfg(test)]
mod tests {
    use solver::Solver;

    use super::*;

    #[test]
    fn part_one() -> anyhow::Result<()> {
        let pb: Problem = ("inc a
jio a, +2
tpl a
inc a".as_bytes().lines(), Registry::A).try_into()?;
        assert_eq!("2", format!("{}", pb.part_one()));
        Ok(())
    }
}
