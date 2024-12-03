pub fn basic_solve(s: &str) -> i32 {
    s.chars().map(|i| {
        match i {
            '(' => 1,
            ')' => -1,
            _ => 0,
        }
    }).sum()
}
