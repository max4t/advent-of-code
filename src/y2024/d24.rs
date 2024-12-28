use std::{collections::HashMap, io::{BufRead, Lines}};
use crate::solver;
use anyhow::{anyhow, bail, Result};

#[derive(Clone, Debug)]
enum Gate {
    And(String, String),
    Or(String, String),
    Xor(String, String),
}

pub struct Problem(HashMap<String, bool>, HashMap<String, Gate>);

impl<B: BufRead> TryFrom<Lines<B>> for Problem {
    type Error = anyhow::Error;

    fn try_from(value: Lines<B>) -> Result<Self, Self::Error> {
        let mut a = value;
        let init= HashMap::from_iter(a.by_ref()
            .take_while(|l| {
                if let Ok(s) = l {
                    !s.is_empty()
                } else {
                    true
                }
            })
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .map(|s| {
                let (name, val) = s.split_once(": ").ok_or_else(|| anyhow!("missing colon delim"))?;
                anyhow::Ok((name.to_owned(), val.parse::<u32>()? == 1))
            })
            .collect::<Result<Vec<_>, _>>()?);
        let ops = HashMap::from_iter(a.collect::<Result<Vec<_>, _>>()?
            .iter()
            .map(|s| {
                let (op, var) = s.split_once(" -> ").ok_or_else(|| anyhow!("missing arrow delim"))?;
                let op = op.split_whitespace().collect::<Vec<_>>();
                let (op1, op, op2) = (op[0], op[1], op[2]);
                anyhow::Ok((var.to_owned(), match op {
                    "AND" => Gate::And(op1.to_string(), op2.to_string()),
                    "OR" => Gate::Or(op1.to_string(), op2.to_string()),
                    "XOR" => Gate::Xor(op1.to_string(), op2.to_string()),
                    _ => bail!("unknown gate"),
                }))
            })
            .collect::<Result<Vec<_>, _>>()?);

        Ok(Self(init, ops))
    }
}

fn compute(cache: &mut HashMap<String, bool>, ops: &HashMap<String, Gate>, op: String) -> bool {
    if let Some(&res) = cache.get(&op) {
        return res;
    }
    let gate = ops.get(&op).unwrap();
    let res = match gate {
        Gate::And(op1, op2) => compute(cache, ops, op1.clone()) & compute(cache, ops, op2.clone()),
        Gate::Or(op1, op2) => compute(cache, ops, op1.clone()) | compute(cache, ops, op2.clone()),
        Gate::Xor(op1, op2) => compute(cache, ops, op1.clone()) ^ compute(cache, ops, op2.clone()),
    };
    cache.insert(op, res);
    res
}

fn build_number(state: &HashMap<String, bool>, name: &str) -> u64 {
    state.iter()
            .filter(|&(l, _)| l.starts_with(name))
            .map(|(k, &v)| (if v { 1 } else { 0 }, k.trim_start_matches(name).parse::<usize>().unwrap()))
            .map(|(a, b)| a << b)
            .sum::<u64>()
}

fn compute_result(state: &mut HashMap<String, bool>, ops: &HashMap<String, Gate>) -> u64 {
    ops.keys().filter(|&l| l.starts_with("z")).for_each(|l| {
        compute(state, ops, l.clone());
    });

    build_number(state, "z")
}

impl solver::Solver for Problem {
    fn part_one(self: &Self) -> impl std::fmt::Display {
        compute_result(&mut self.0.clone(), &self.1)
    }

    // TODO interactive version??
    fn part_two(self: &Self) -> impl std::fmt::Display {
        let expected = build_number(&self.0, "x") + build_number(&self.0, "y");
        fn swap(state: &mut HashMap<String, Gate>, k1: String, k2: String) {
            let old = state.insert(k2, state.get(&k1).unwrap().clone());
            state.insert(k1, old.unwrap());
        }

        let diff = !(dbg!(expected) ^ dbg!(compute_result(&mut self.0.clone(), &self.1)));
        let test_diff = diff - ((1 << 39) + (1 << 38) + (1 << 37) + (1 << 36) + (1 << 35) + (1 << 34) + (1 << 15) + (1 << 14) + (1 << 13) + (1 << 12) + (1 << 8) + (1 << 7));
        println!("{diff} {test_diff}");

        let ops = &mut self.1.clone();
        swap(ops, "rts".to_string(), "z07".to_string());
        swap(ops, "z12".to_string(), "jpj".to_string());
        swap(ops, "kgj".to_string(), "z26".to_string());
        swap(ops, "chv".to_string(), "vvw".to_string()); //chv,jpj,kgj,rts,vvw,z07,z12,z26 // this is the answer !!!
        let diff = !(expected ^ dbg!(compute_result(&mut self.0.clone(), ops)));
        let false_val = diff.count_ones();
        println!("{diff} {false_val}");

        0
    }
}

#[cfg(test)]
mod tests {
    use solver::Solver;

    use super::*;

    #[test]
    fn part_one() {
        assert_eq!("2024", format!("{}", Problem(HashMap::from([
            ("x00".to_owned(), true),
            ("x01".to_owned(), false),
            ("x02".to_owned(), true),
            ("x03".to_owned(), true),
            ("x04".to_owned(), false),
            ("y00".to_owned(), true),
            ("y01".to_owned(), true),
            ("y02".to_owned(), true),
            ("y03".to_owned(), true),
            ("y04".to_owned(), true),
        ]), HashMap::from([
            ("mjb".to_owned(), Gate::Xor("ntg".to_owned(), "fgs".to_owned())),
            ("tnw".to_owned(), Gate::Or("y02".to_owned(), "x01".to_owned())),
            ("z05".to_owned(), Gate::Or("kwq".to_owned(), "kpj".to_owned())),
            ("fst".to_owned(), Gate::Or("x00".to_owned(), "x03".to_owned())),
            ("z01".to_owned(), Gate::Xor("tgd".to_owned(), "rvg".to_owned())),
            ("bfw".to_owned(), Gate::Or("vdt".to_owned(), "tnw".to_owned())),
            ("z10".to_owned(), Gate::And("bfw".to_owned(), "frj".to_owned())),
            ("bqk".to_owned(), Gate::Or("ffh".to_owned(), "nrd".to_owned())),
            ("djm".to_owned(), Gate::And("y00".to_owned(), "y03".to_owned())),
            ("psh".to_owned(), Gate::Or("y03".to_owned(), "y00".to_owned())),
            ("z08".to_owned(), Gate::Or("bqk".to_owned(), "frj".to_owned())),
            ("frj".to_owned(), Gate::Or("tnw".to_owned(), "fst".to_owned())),
            ("z11".to_owned(), Gate::And("gnj".to_owned(), "tgd".to_owned())),
            ("z00".to_owned(), Gate::Xor("bfw".to_owned(), "mjb".to_owned())),
            ("vdt".to_owned(), Gate::Or("x03".to_owned(), "x00".to_owned())),
            ("z02".to_owned(), Gate::And("gnj".to_owned(), "wpb".to_owned())),
            ("kjc".to_owned(), Gate::And("x04".to_owned(), "y00".to_owned())),
            ("qhw".to_owned(), Gate::Or("djm".to_owned(), "pbm".to_owned())),
            ("hwm".to_owned(), Gate::And("nrd".to_owned(), "vdt".to_owned())),
            ("rvg".to_owned(), Gate::And("kjc".to_owned(), "fst".to_owned())),
            ("fgs".to_owned(), Gate::Or("y04".to_owned(), "y02".to_owned())),
            ("pbm".to_owned(), Gate::And("y01".to_owned(), "x02".to_owned())),
            ("kwq".to_owned(), Gate::Or("ntg".to_owned(), "kjc".to_owned())),
            ("tgd".to_owned(), Gate::Xor("psh".to_owned(), "fgs".to_owned())),
            ("z09".to_owned(), Gate::Xor("qhw".to_owned(), "tgd".to_owned())),
            ("kpj".to_owned(), Gate::Or("pbm".to_owned(), "djm".to_owned())),
            ("ffh".to_owned(), Gate::Xor("x03".to_owned(), "y03".to_owned())),
            ("ntg".to_owned(), Gate::Xor("x00".to_owned(), "y04".to_owned())),
            ("z06".to_owned(), Gate::Or("bfw".to_owned(), "bqk".to_owned())),
            ("wpb".to_owned(), Gate::Xor("nrd".to_owned(), "fgs".to_owned())),
            ("z04".to_owned(), Gate::Xor("frj".to_owned(), "qhw".to_owned())),
            ("z07".to_owned(), Gate::Or("bqk".to_owned(), "frj".to_owned())),
            ("nrd".to_owned(), Gate::Or("y03".to_owned(), "x01".to_owned())),
            ("z03".to_owned(), Gate::And("hwm".to_owned(), "bqk".to_owned())),
            ("z12".to_owned(), Gate::Xor("tgd".to_owned(), "rvg".to_owned())),
            ("gnj".to_owned(), Gate::Or("tnw".to_owned(), "pbm".to_owned())),
        ])).part_one()));
    }

    #[test]
    fn part_two() {
        assert_eq!("co,de,ka,ta", format!("{}", Problem(HashMap::from([
            ("x00".to_owned(), true),
            ("x01".to_owned(), false),
            ("x02".to_owned(), true),
            ("x03".to_owned(), true),
            ("x04".to_owned(), false),
            ("y00".to_owned(), true),
            ("y01".to_owned(), true),
            ("y02".to_owned(), true),
            ("y03".to_owned(), true),
            ("y04".to_owned(), true),
        ]), HashMap::from([
            ("mjb".to_owned(), Gate::Xor("ntg".to_owned(), "fgs".to_owned())),
            ("tnw".to_owned(), Gate::Or("y02".to_owned(), "x01".to_owned())),
            ("z05".to_owned(), Gate::Or("kwq".to_owned(), "kpj".to_owned())),
            ("fst".to_owned(), Gate::Or("x00".to_owned(), "x03".to_owned())),
            ("z01".to_owned(), Gate::Xor("tgd".to_owned(), "rvg".to_owned())),
            ("bfw".to_owned(), Gate::Or("vdt".to_owned(), "tnw".to_owned())),
            ("z10".to_owned(), Gate::And("bfw".to_owned(), "frj".to_owned())),
            ("bqk".to_owned(), Gate::Or("ffh".to_owned(), "nrd".to_owned())),
            ("djm".to_owned(), Gate::And("y00".to_owned(), "y03".to_owned())),
            ("psh".to_owned(), Gate::Or("y03".to_owned(), "y00".to_owned())),
            ("z08".to_owned(), Gate::Or("bqk".to_owned(), "frj".to_owned())),
            ("frj".to_owned(), Gate::Or("tnw".to_owned(), "fst".to_owned())),
            ("z11".to_owned(), Gate::And("gnj".to_owned(), "tgd".to_owned())),
            ("z00".to_owned(), Gate::Xor("bfw".to_owned(), "mjb".to_owned())),
            ("vdt".to_owned(), Gate::Or("x03".to_owned(), "x00".to_owned())),
            ("z02".to_owned(), Gate::And("gnj".to_owned(), "wpb".to_owned())),
            ("kjc".to_owned(), Gate::And("x04".to_owned(), "y00".to_owned())),
            ("qhw".to_owned(), Gate::Or("djm".to_owned(), "pbm".to_owned())),
            ("hwm".to_owned(), Gate::And("nrd".to_owned(), "vdt".to_owned())),
            ("rvg".to_owned(), Gate::And("kjc".to_owned(), "fst".to_owned())),
            ("fgs".to_owned(), Gate::Or("y04".to_owned(), "y02".to_owned())),
            ("pbm".to_owned(), Gate::And("y01".to_owned(), "x02".to_owned())),
            ("kwq".to_owned(), Gate::Or("ntg".to_owned(), "kjc".to_owned())),
            ("tgd".to_owned(), Gate::Xor("psh".to_owned(), "fgs".to_owned())),
            ("z09".to_owned(), Gate::Xor("qhw".to_owned(), "tgd".to_owned())),
            ("kpj".to_owned(), Gate::Or("pbm".to_owned(), "djm".to_owned())),
            ("ffh".to_owned(), Gate::Xor("x03".to_owned(), "y03".to_owned())),
            ("ntg".to_owned(), Gate::Xor("x00".to_owned(), "y04".to_owned())),
            ("z06".to_owned(), Gate::Or("bfw".to_owned(), "bqk".to_owned())),
            ("wpb".to_owned(), Gate::Xor("nrd".to_owned(), "fgs".to_owned())),
            ("z04".to_owned(), Gate::Xor("frj".to_owned(), "qhw".to_owned())),
            ("z07".to_owned(), Gate::Or("bqk".to_owned(), "frj".to_owned())),
            ("nrd".to_owned(), Gate::Or("y03".to_owned(), "x01".to_owned())),
            ("z03".to_owned(), Gate::And("hwm".to_owned(), "bqk".to_owned())),
            ("z12".to_owned(), Gate::Xor("tgd".to_owned(), "rvg".to_owned())),
            ("gnj".to_owned(), Gate::Or("tnw".to_owned(), "pbm".to_owned())),
        ])).part_two()));
    }
}
