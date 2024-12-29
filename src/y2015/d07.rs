use std::{collections::HashMap, io::{BufRead, Lines}};
use crate::solver;
use anyhow::{anyhow, bail, Result};

#[derive(Clone)]
enum Signal {
    Value(u16),
    Wire(String),
}
impl TryFrom<&str> for Signal {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        match value.parse() {
            Ok(v) => Ok(Self::Value(v)),
            Err(_) => Ok(Self::Wire(value.to_owned())),
        }
    }
}
impl Signal {
    fn compute(&self, signals: &mut Circuit) -> u16 {
        match self {
            Self::Value(v) => *v,
            Self::Wire(w) => signals.compute(w),
        }
    }
}

#[derive(Clone)]
enum Gate {
    And(Signal, Signal),
    Or(Signal, Signal),
    LShift(Signal, Signal),
    RShift(Signal, Signal),
    Not(Signal),
}
impl Gate {
    fn compute(&self, signals: &mut Circuit) -> u16 {
        match self {
            Gate::And(signal, signal1) => signal.compute(signals) & signal1.compute(signals),
            Gate::Or(signal, signal1) => signal.compute(signals) | signal1.compute(signals),
            Gate::LShift(signal, signal1) => signal.compute(signals) << signal1.compute(signals),
            Gate::RShift(signal, signal1) => signal.compute(signals) >> signal1.compute(signals),
            Gate::Not(signal) => !signal.compute(signals),
        }
    }
}
impl TryFrom<&str> for Gate {
    type Error = anyhow::Error;
    
    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        let ops = value.split_whitespace().collect::<Vec<_>>();
        match ops.len() {
            2 if ops[0] == "NOT" => Ok(Self::Not(ops[1].try_into()?)),
            3 if ops[1] == "AND" => Ok(Self::And(ops[0].try_into()?, ops[2].try_into()?)),
            3 if ops[1] == "OR" => Ok(Self::Or(ops[0].try_into()?, ops[2].try_into()?)),
            3 if ops[1] == "LSHIFT" => Ok(Self::LShift(ops[0].try_into()?, ops[2].try_into()?)),
            3 if ops[1] == "RSHIFT" => Ok(Self::RShift(ops[0].try_into()?, ops[2].try_into()?)),
            _ => bail!("invalid instruction ({value})"),
        }
    }
}

#[derive(Clone)]
enum Wire {
    Signal(Signal),
    Gate(Gate),
}
impl Wire {
    fn compute(&self, signals: &mut Circuit) -> u16 {
        match self {
            Wire::Signal(signal) => signal.compute(signals),
            Wire::Gate(gate) => gate.compute(signals),
        }
    }
}
impl TryFrom<&str> for Wire {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        if value.contains(" ") {
            Ok(Self::Gate(value.try_into()?))
        } else {
            Ok(Self::Signal(value.try_into()?))
        }
    }
}

// struct Wired {
//     wire: Wire,
//     value: Option<u16>,
// }
// impl Wired {
//     fn compute(&self, signals: &mut Circuit) -> u16 {
//         *self.value.get_or_insert_with(|| self.wire.compute(signals))
//     }
//     fn set(&mut self, v: Option<u16>) {
//         self.value = v;
//     }
// }

#[derive(Clone)]
struct Circuit {
    wires: HashMap<String, Wire>,
    cache: HashMap<String, u16>,
}
impl Circuit {
    fn new(wires: HashMap<String, Wire>) -> Self {
        Self {
            wires,
            cache: HashMap::new(),
        }
    }

    fn reset(&mut self) {
        self.cache.clear();
    }

    fn set(&mut self, name: &str, value: u16) {
        self.cache.insert(name.to_string(), value);
    }

    fn compute(&mut self, name: &str) -> u16 {
        match self.cache.get(name) {
            Some(v) => *v,
            None => {
                let res = self.wires.get(name).unwrap().clone().compute(self);
                self.cache.insert(name.to_string(), res);
                res
            },
        }
    }
}

pub struct Problem(HashMap<String, Wire>);

impl<B: BufRead> TryFrom<Lines<B>> for Problem {
    type Error = anyhow::Error;

    fn try_from(value: Lines<B>) -> Result<Self, Self::Error> {
        Ok(Self(HashMap::from_iter(value.map(|l| {
            let l = l?;
            let (gate, name) = l.split_once(" -> ").ok_or_else(|| anyhow!("missing arrow of the instruction"))?;
            anyhow::Ok((name.to_owned(), gate.try_into()?))
        }).collect::<Result<Vec<_>, _>>()?.into_iter())))
    }
}

impl solver::Solver for Problem {
    fn part_one(self: &Self) -> impl std::fmt::Display {
        let mut board = Circuit::new(self.0.clone());
        board.compute("a")
    }

    fn part_two(self: &Self) -> impl std::fmt::Display {
        let mut board = Circuit::new(self.0.clone());
        let val = board.compute("a");
        board.reset();
        board.set("b", val);
        board.compute("a")
    }
}

#[cfg(test)]
mod tests {
}
