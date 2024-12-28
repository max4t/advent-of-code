use std::{collections::{HashMap, HashSet}, io::{BufRead, Lines}, iter};
use crate::solver;
use anyhow::{anyhow, Result};
use itertools::Itertools;

pub struct Problem(Vec<(String, String)>);

impl<B: BufRead> TryFrom<Lines<B>> for Problem {
    type Error = anyhow::Error;

    fn try_from(value: Lines<B>) -> Result<Self, Self::Error> {
        Ok(Self(value.map(|l| {
            let l = l?;
            let (a, b) = l.split_once("-").ok_or_else(|| anyhow!("missing separator"))?;
            anyhow::Ok((a.to_string(), b.to_string()))
        }).collect::<Result<Vec<_>, _>>()?))
    }
}

fn bron_kerbosch<'a, F>(r: HashSet<&'a String>, mut p: HashSet<&'a String>, mut x: HashSet<&'a String>, n: &F) -> Vec<HashSet<&'a String>>
where
    F: Fn(&'a String) -> &'a HashSet<&'a String>
{
    let mut res = Vec::new();
    if p.is_empty() && x.is_empty() {
        res.extend([r]);
        return res;
    }
    let &u = p.union(&x).next().unwrap();
    let nu = n(u);
    while let Some(&v) = p.difference(nu).next() {
        let nv = n(v);
        res.extend(bron_kerbosch(
            r.iter().cloned().chain(iter::once(v)).collect(),
            p.intersection(nv).cloned().collect(),
            x.intersection(nv).cloned().collect(),
            n,
        ));
        p.remove(v);
        x.insert(v);
    }
    res
}

impl solver::Solver for Problem {
    fn part_one(self: &Self) -> impl std::fmt::Display {
        let mut directs = HashMap::<&String, HashSet<&String>>::new();
        for (a, b) in self.0.iter() {
            directs.entry(a).and_modify(|s| { s.insert(b); }).or_insert_with(|| HashSet::from([b]));
            directs.entry(b).and_modify(|s| { s.insert(a); }).or_insert_with(|| HashSet::from([a]));
        }

        let res = HashSet::<_>::from_iter(directs.iter().flat_map(|(&cmp, nexts)|
            nexts.into_iter()
                .filter_map(|&next| directs.get(next).map(|s| (next, s)))
                .flat_map(move |(next, other_nexts)|
                    other_nexts.intersection(&nexts).map(move |&third| {
                        let mut els = [cmp, next, third];
                        els.sort();
                        els
                    })
                )
        ));
        res.into_iter().filter(|&els| els.into_iter().any(|el| el.starts_with("t"))).count()
    }

    fn part_two(self: &Self) -> impl std::fmt::Display {
        let mut directs = HashMap::<&String, HashSet<&String>>::new();
        for (a, b) in self.0.iter() {
            directs.entry(a).and_modify(|s| { s.insert(b); }).or_insert_with(|| HashSet::from([b]));
            directs.entry(b).and_modify(|s| { s.insert(a); }).or_insert_with(|| HashSet::from([a]));
        }

        let empty = HashSet::new();
        let res = bron_kerbosch(HashSet::new(), directs.keys().cloned().collect(), HashSet::new(), &|s| directs.get(s).unwrap_or(&empty));
        res.into_iter().max_by_key(|l| l.len()).unwrap().iter().sorted().join(",")
    }
}

#[cfg(test)]
mod tests {
    use solver::Solver;

    use super::*;

    #[test]
    fn part_one() {
        assert_eq!("7", format!("{}", Problem(vec![
            ("kh".to_owned(), "tc".to_owned()),
            ("qp".to_owned(), "kh".to_owned()),
            ("de".to_owned(), "cg".to_owned()),
            ("ka".to_owned(), "co".to_owned()),
            ("yn".to_owned(), "aq".to_owned()),
            ("qp".to_owned(), "ub".to_owned()),
            ("cg".to_owned(), "tb".to_owned()),
            ("vc".to_owned(), "aq".to_owned()),
            ("tb".to_owned(), "ka".to_owned()),
            ("wh".to_owned(), "tc".to_owned()),
            ("yn".to_owned(), "cg".to_owned()),
            ("kh".to_owned(), "ub".to_owned()),
            ("ta".to_owned(), "co".to_owned()),
            ("de".to_owned(), "co".to_owned()),
            ("tc".to_owned(), "td".to_owned()),
            ("tb".to_owned(), "wq".to_owned()),
            ("wh".to_owned(), "td".to_owned()),
            ("ta".to_owned(), "ka".to_owned()),
            ("td".to_owned(), "qp".to_owned()),
            ("aq".to_owned(), "cg".to_owned()),
            ("wq".to_owned(), "ub".to_owned()),
            ("ub".to_owned(), "vc".to_owned()),
            ("de".to_owned(), "ta".to_owned()),
            ("wq".to_owned(), "aq".to_owned()),
            ("wq".to_owned(), "vc".to_owned()),
            ("wh".to_owned(), "yn".to_owned()),
            ("ka".to_owned(), "de".to_owned()),
            ("kh".to_owned(), "ta".to_owned()),
            ("co".to_owned(), "tc".to_owned()),
            ("wh".to_owned(), "qp".to_owned()),
            ("tb".to_owned(), "vc".to_owned()),
            ("td".to_owned(), "yn".to_owned()),
        ]).part_one()));
    }

    #[test]
    fn part_two() {
        assert_eq!("co,de,ka,ta", format!("{}", Problem(vec![
            ("kh".to_owned(), "tc".to_owned()),
            ("qp".to_owned(), "kh".to_owned()),
            ("de".to_owned(), "cg".to_owned()),
            ("ka".to_owned(), "co".to_owned()),
            ("yn".to_owned(), "aq".to_owned()),
            ("qp".to_owned(), "ub".to_owned()),
            ("cg".to_owned(), "tb".to_owned()),
            ("vc".to_owned(), "aq".to_owned()),
            ("tb".to_owned(), "ka".to_owned()),
            ("wh".to_owned(), "tc".to_owned()),
            ("yn".to_owned(), "cg".to_owned()),
            ("kh".to_owned(), "ub".to_owned()),
            ("ta".to_owned(), "co".to_owned()),
            ("de".to_owned(), "co".to_owned()),
            ("tc".to_owned(), "td".to_owned()),
            ("tb".to_owned(), "wq".to_owned()),
            ("wh".to_owned(), "td".to_owned()),
            ("ta".to_owned(), "ka".to_owned()),
            ("td".to_owned(), "qp".to_owned()),
            ("aq".to_owned(), "cg".to_owned()),
            ("wq".to_owned(), "ub".to_owned()),
            ("ub".to_owned(), "vc".to_owned()),
            ("de".to_owned(), "ta".to_owned()),
            ("wq".to_owned(), "aq".to_owned()),
            ("wq".to_owned(), "vc".to_owned()),
            ("wh".to_owned(), "yn".to_owned()),
            ("ka".to_owned(), "de".to_owned()),
            ("kh".to_owned(), "ta".to_owned()),
            ("co".to_owned(), "tc".to_owned()),
            ("wh".to_owned(), "qp".to_owned()),
            ("tb".to_owned(), "vc".to_owned()),
            ("td".to_owned(), "yn".to_owned()),
        ]).part_two()));
    }
}
