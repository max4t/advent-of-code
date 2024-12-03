use std::ops::ControlFlow;

pub fn basic_solve(s: &[String]) -> u64 {
    s.iter().filter(|&s| {
        s.chars().filter(|&c| c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u').count() >= 3
    }).filter(|&s| {
        s.chars().try_fold('\0', |acc, v| {
            if acc == v {
                ControlFlow::Break(())
            } else {
                ControlFlow::Continue(v)
            }
        }).is_break()
    }).filter(|&s| {
        !s.contains("ab") && !s.contains("cd") && !s.contains("pq") && !s.contains("xy")
    }).count().try_into().unwrap()
}
