use std::iter::zip;

pub fn basic_solve(a: &mut [u32], b: &mut [u32]) -> u32 {
    assert!(a.len() == b.len(), "lists have different lengths (a = {}, b = {})", a.len(), b.len());
    a.sort();
    b.sort();
    zip(a, b)
        .map(|(o, t)| o.abs_diff(*t))
        .sum()
}
