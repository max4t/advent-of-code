use std::collections::HashSet;


pub fn basic_solve(deps: &[(u64, u64)], updates: &mut [Vec<u64>]) -> u64 {
    let h = HashSet::<&(u64, u64)>::from_iter(deps.iter());
    updates.iter().filter(|update| {
        let mut update = update.as_slice();
        while let Some((&f, new_update)) = update.split_first() {
            if update.iter().any(|&v| h.contains(&(v, f))) {
                return false
            }
            update = new_update;
        }
        true
    }).map(|f| f[f.len()/2]).sum()
}
