pub fn basic_solve(s: &[(i64, i64, i64)]) -> i64 {
    s.iter().map(|&(l, w, h)| {
        let sides = [l*w, l*h, w*h];
        sides.iter().sum::<i64>()*2 + sides.iter().min().unwrap()
    }).sum()
}
