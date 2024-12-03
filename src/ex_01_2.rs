use std::collections::HashMap;

pub fn basic_solve(a: &[u32], b: &[u32]) -> u32 {
    assert!(a.len() == b.len(), "lists have different lengths (a = {}, b = {})", a.len(), b.len());
    let mut counters = HashMap::new();
    for &el in b {
        counters.entry(el).and_modify(|e| *e += 1).or_insert(1);
    }
    a.iter().map(|e| e * counters.get(e).unwrap_or(&0)).sum()
}
