use std::io::Stdin;
use crate::solver;
use anyhow::{anyhow, bail, ensure, Result};

#[derive(Clone, Copy, Debug)]
struct Machine {
    a: u64,
    b: u64,
    c: u64,

    ptr: usize,
}

trait Operand {
    fn value(self, machine: &Machine) -> u64;
}

#[derive(Debug)]
struct LiteralOperand(u8);

impl TryFrom<u8> for LiteralOperand {
    type Error = anyhow::Error;

    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        ensure!(value < 8, "invalid literal operand");
        Ok(Self(value))
    }
}

impl Operand for LiteralOperand {
    fn value(self, _machine: &Machine) -> u64 {
        self.0.into()
    }
}

#[derive(Debug)]
enum ComboOperand {
    Literal(u8),
    RegA,
    RegB,
    RegC,
    Invalid,
}

impl TryFrom<u8> for ComboOperand {
    type Error = anyhow::Error;

    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        Ok(match value {
            0 | 1 | 2 | 3 => Self::Literal(value),
            4 => Self::RegA,
            5 => Self::RegB,
            6 => Self::RegC,
            7 => Self::Invalid,
            _ => bail!("invalid combo operand"),
        })
    }
}

impl Operand for ComboOperand {
    fn value(self, machine: &Machine) -> u64 {
        match self {
            Self::Literal(v) => v.into(),
            Self::RegA => machine.a,
            Self::RegB => machine.b,
            Self::RegC => machine.c,
            Self::Invalid => panic!("using reserved combo operand"),
        }
    }
}

#[derive(Debug)]
enum Instruction {
    Adv(ComboOperand),
    Bxl(LiteralOperand),
    Bst(ComboOperand),
    Jnz(LiteralOperand),
    Bxc,
    Out(ComboOperand),
    Bdv(ComboOperand),
    Cdv(ComboOperand),
}

impl Instruction {
    fn process(self: Self, state: &mut Machine) -> Option<u64> {
        match self {
            Self::Adv(operand) => state.a >>= operand.value(state),
            Self::Bxl(operand) => state.b ^= operand.value(state),
            Self::Bst(operand) => state.b = operand.value(state) & 7,
            Self::Jnz(operand) => if state.a != 0 {
                state.ptr = operand.value(state).try_into().unwrap();
                return None;
            },
            Self::Bxc => state.b ^= state.c,
            Self::Out(operand) => {
                let res = operand.value(state) & 7;
                state.ptr += 2;
                return Some(res);
            },
            Self::Bdv(operand) => state.b = state.a >> operand.value(state),
            Self::Cdv(operand) => state.c = state.a >> operand.value(state),
        };
        state.ptr += 2;
        None
    }
}

impl TryFrom<(u8, u8)> for Instruction {
    type Error = anyhow::Error;

    fn try_from(value: (u8, u8)) -> Result<Self, Self::Error> {
        match value.0 {
            0 => Ok(Self::Adv(value.1.try_into()?)),
            1 => Ok(Self::Bxl(value.1.try_into()?)),
            2 => Ok(Self::Bst(value.1.try_into()?)),
            3 => Ok(Self::Jnz(value.1.try_into()?)),
            4 => Ok(Self::Bxc),
            5 => Ok(Self::Out(value.1.try_into()?)),
            6 => Ok(Self::Bdv(value.1.try_into()?)),
            7 => Ok(Self::Cdv(value.1.try_into()?)),
            _ => anyhow::bail!("unknown value"),
        }
    }
}

pub struct Problem(Machine, Vec<u8>);

impl TryFrom<Stdin> for Problem
{
    type Error = anyhow::Error;

    fn try_from(value: Stdin) -> Result<Self, Self::Error> {
        let mut a = value.lines();

        let reg_a = a.next().ok_or_else(|| anyhow!("unable to parse registry A"))??.trim_start_matches("Register A: ").parse::<u64>()?;
        let reg_b = a.next().ok_or_else(|| anyhow!("unable to parse registry B"))??.trim_start_matches("Register B: ").parse::<u64>()?;
        let reg_c = a.next().ok_or_else(|| anyhow!("unable to parse registry C"))??.trim_start_matches("Register C: ").parse::<u64>()?;
        a.next();
        let prog = a.next().ok_or_else(|| anyhow!("unable to parse program"))??.trim_start_matches("Program: ").split(",").map(|v| v.parse::<u8>()).collect::<Result<Vec<_>, _>>()?;

        Ok(Self(Machine { a: reg_a, b: reg_b, c: reg_c, ptr: 0 }, prog))
    }
}

impl solver::Solver for Problem {
    fn part_one(self: &Self) -> impl std::fmt::Display {
        let mut res = vec![];
        let state = &mut self.0.clone();

        while state.ptr < self.1.len() {
            let instruct: Instruction = (self.1[state.ptr], self.1[state.ptr+1]).try_into().unwrap();
            instruct.process(state).iter().for_each(|&v| { res.push(v); });
        }

        res.iter().map(|&v| format!("{}", v)).collect::<Vec<_>>().join(",")
    }

    fn part_two(self: &Self) -> impl std::fmt::Display {
        fn run(machine: &Machine, expected: &[u64], prog: &[u8], a: u64) -> Option<u64> {
            for i in a..(a+16) {
                let mut res = vec![];
                let state = &mut machine.clone();
                state.a = i;
        
                while state.ptr < prog.len() {
                    let instruct: Instruction = (prog[state.ptr], prog[state.ptr+1]).try_into().unwrap();
                    instruct.process(state).iter().for_each(|&v| { res.push(v); });
                }

                // println!("{} {}", i, &res.iter().map(|l| format!("{}", l)).collect::<Vec<_>>().join(","));
                if res[..] == expected[(expected.len()-res.len())..] {
                    if res.len() == expected.len() {
                        return Some(i);
                    }
                    if let Some(r) = run(machine, expected, prog, i*8) {
                        return Some(r);
                    }
                }
            }
            None
        }

        run(&self.0.clone(), &self.1.iter().map(|&l| l as u64).collect::<Vec<_>>(), &self.1, 1).unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use solver::Solver;

    use super::*;

    #[test]
    fn part_one() {
        assert_eq!("4,6,3,5,6,3,5,2,1,0", format!("{}", Problem(Machine { a: 729, b: 0, c: 0, ptr: 0 }, vec![0,1,5,4,3,0]).part_one()));
    }

    #[test]
    fn part_two() {
        assert_eq!("117440", format!("{}", Problem(Machine { a: 2024, b: 0, c: 0, ptr: 0 }, vec![0,3,5,4,3,0]).part_two()));
    }
}
