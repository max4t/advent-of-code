use std::{cmp::Ordering, collections::HashSet};


pub fn basic_solve(deps: &[(u64, u64)], updates: &mut [Vec<u64>]) -> u64 {
    let h = HashSet::<&(u64, u64)>::from_iter(deps.iter());
    updates.iter_mut().filter(|update| {
        let mut update = update.as_slice();
        while let Some((&f, new_update)) = update.split_first() {
            if update.iter().any(|&v| h.contains(&(v, f))) {
                return true
            }
            update = new_update;
        }
        false
    })
    .map(|f| {
        f.sort_by(|&a, &b| {
            if h.contains(&(b, a)) { Ordering::Greater } else { Ordering::Less }
        });
        f
    })
    .map(|f| f[f.len()/2]).sum()
}
