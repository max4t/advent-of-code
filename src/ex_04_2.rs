
pub fn basic_solve(a: &[&[char]]) -> usize {
    let xs = a.iter().enumerate().skip(1).take(a.len()-2).flat_map(|(x, &aa)| {
        aa.iter().enumerate().skip(1).take(aa.len()-2).filter_map(move |(y, &aaa)| if aaa == 'A' { Some((x, y)) } else { None })
    }).collect::<Vec<_>>();
    xs.iter().filter(|&&(x, y)| {
        ((a[x-1][y-1] == 'M' && a[x+1][y+1] == 'S') || (a[x-1][y-1] == 'S' && a[x+1][y+1] == 'M')) &&
            ((a[x-1][y+1] == 'M' && a[x+1][y-1] == 'S') || (a[x-1][y+1] == 'S' && a[x+1][y-1] == 'M'))
    }).count()
}
