use std::collections::HashMap;

pub fn basic_solve(s: &[String]) -> u64 {
    s.iter().filter(|&s| {
        let ok = s.chars().collect::<Vec<_>>();
        let pairs = ok.iter().zip(ok.iter().skip(1)).map::<String, _>(|(&i, &j)| i.to_string() + &j.to_string()).collect::<Vec<_>>();
        pairs.iter().enumerate().fold(HashMap::<String, Vec<usize>>::new(), |mut acc, (i, p)| {
            acc.entry(p.to_owned()).and_modify(|l| l.push(i)).or_insert(vec![i]);
            acc
        }).values().filter(|idxs| idxs.len() > 1).any(|idxs| idxs.last().unwrap() - idxs.first().unwrap() > 1)
    }).filter(|&s| {
        let idxs = s.chars().enumerate().fold(HashMap::<char, Vec<usize>>::new(), |mut acc, (i, c)| {
            acc.entry(c).and_modify(|l| l.push(i)).or_insert(vec![i]);
            acc
        });
        idxs.values()
            .filter(|idxs| idxs.len() > 1)
            .any(|idxs| (0..(idxs.len()-1)).any(|i| idxs[i].abs_diff(idxs[i+1]) == 2)) ||
            idxs.values()
                .filter(|idxs| idxs.len() > 2)
                .any(|idxs| (0..(idxs.len()-2)).any(|i| idxs[i].abs_diff(idxs[i+2]) == 2))
    }).count().try_into().unwrap()
}
