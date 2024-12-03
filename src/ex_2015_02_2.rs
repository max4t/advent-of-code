pub fn basic_solve(s: &[(i64, i64, i64)]) -> i64 {
    s.iter().map(|&(l, w, h)| {
        let length = [l, w, h];
        (length.iter().sum::<i64>() - length.iter().max().unwrap())*2 + length.iter().product::<i64>()
    }).sum()
}
