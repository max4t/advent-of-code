use std::io::{BufRead, Lines};
use crate::solver;
use anyhow::Result;
use itertools::Itertools;

pub struct Problem(String);

impl<B: BufRead> TryFrom<Lines<B>> for Problem {
    type Error = anyhow::Error;

    fn try_from(value: Lines<B>) -> Result<Self, Self::Error> {
        Ok(Self(value.collect::<Result<String, _>>()?))
    }
}

fn process(value: &serde_json::Value) -> i64 {
    match value {
        serde_json::Value::Null => 0,
        serde_json::Value::Bool(_) => 0,
        serde_json::Value::Number(number) => number.as_i64().unwrap_or(0),
        serde_json::Value::String(_) => 0,
        serde_json::Value::Array(vec) => vec.iter().map(|v| process(v)).sum(),
        serde_json::Value::Object(map) => {
            if map.values().any(|v| v.as_str() == Some("red")) {
                0
            } else {
                map.values().map(|v| process(v)).sum()
            }
        },
    }
}

impl solver::Solver for Problem {
    fn part_one(self: &Self) -> impl std::fmt::Display {
        self.0.chars().peekable().batching(|it| {
            it.peeking_take_while(|&c| !c.is_digit(10) && c != '-').for_each(|_| {});
            let n = it.peeking_take_while(|&c| c.is_digit(10) || c == '-').collect::<String>();
            (n.len() > 0).then(|| n.parse::<i64>().unwrap())
        }).sum::<i64>()
    }

    fn part_two(self: &Self) -> impl std::fmt::Display {
        process(&serde_json::from_str(&self.0).unwrap())
    }
}

#[cfg(test)]
mod tests {
    use solver::Solver;
    use test_case::test_case;

    use super::*;

    #[test_case("[1,2,3]", "6")]
    #[test_case("{\"a\":2,\"b\":4}", "6")]
    #[test_case("[[[3]]]", "3")]
    #[test_case("{\"a\":{\"b\":4},\"c\":-1}", "3")]
    #[test_case("{\"a\":[-1,1]}", "0")]
    #[test_case("[-1,{\"a\":1}]", "0")]
    #[test_case("[]", "0" ; "obj")]
    #[test_case("{}", "0" ; "arr")]
    fn part_one(example: &str, result: &str) -> anyhow::Result<()> {
        let pb: Problem = example.as_bytes().lines().try_into()?;
        assert_eq!(result, format!("{}", pb.part_one()));
        Ok(())
    }

    #[test_case("[1,2,3]", "6")]
    #[test_case("[1,{\"c\":\"red\",\"b\":2},3]", "4")]
    #[test_case("{\"d\":\"red\",\"e\":[1,2,3,4],\"f\":5}", "0")]
    #[test_case("[1,\"red\",5]", "6")]
    fn part_two(example: &str, result: &str) -> anyhow::Result<()> {
        let pb: Problem = example.as_bytes().lines().try_into()?;
        assert_eq!(result, format!("{}", pb.part_two()));
        Ok(())
    }
}
