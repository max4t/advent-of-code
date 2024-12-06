
pub fn basic_solve(a: &[&[char]]) -> usize {
    let xs = a.iter().enumerate().flat_map(|(x, &aa)| {
        aa.iter().enumerate().filter_map(move |(y, &aaa)| if aaa == 'X' { Some((x, y)) } else { None })
    }).collect::<Vec<_>>();
    xs.iter().filter(|&&(x, y)| x+3 < a.len() && a[x+1][y] == 'M' && a[x+2][y] == 'A' && a[x+3][y] == 'S').count();
    xs.iter().filter(|&&(x, y)| x >= 3 && a[x-1][y] == 'M' && a[x-2][y] == 'A' && a[x-3][y] == 'S').count();
    xs.iter().filter(|&&(x, y)| y+3 < a[x].len() && a[x][y+1] == 'M' && a[x][y+2] == 'A' && a[x][y+3] == 'S').count();
    xs.iter().filter(|&&(x, y)| y >= 3 && a[x][y-1] == 'M' && a[x][y-2] == 'A' && a[x][y-3] == 'S').count();

    xs.iter().filter(|&&(x, y)| y+3 < a[x].len() && a[x][y+1] == 'M' && a[x][y+2] == 'A' && a[x][y+3] == 'S').count();
    xs.iter().filter(|&&(x, y)| y >= 3 && a[x][y-1] == 'M' && a[x][y-2] == 'A' && a[x][y-3] == 'S').count();

    xs.iter().flat_map(|&(x, y)| [(-1_isize, -1_isize), (-1, 0), (-1, 1), (0, 1), (1, 1), (1, 0), (1, -1), (0, -1)].iter().filter(move |(i, j)| {
        x.checked_add_signed(3*i).is_some_and(|l| l < a.len()) && y.checked_add_signed(3*j).is_some_and(|l| l < a[x].len())
    }).filter(move |&&(i, j)| {
        *a.get(x.checked_add_signed(i).unwrap()).and_then(|r| r.get(y.checked_add_signed(j).unwrap())).unwrap() == 'M' &&
            *a.get(x.checked_add_signed(2*i).unwrap()).and_then(|r| r.get(y.checked_add_signed(2*j).unwrap())).unwrap() == 'A' &&
            *a.get(x.checked_add_signed(3*i).unwrap()).and_then(|r| r.get(y.checked_add_signed(3*j).unwrap())).unwrap() == 'S'
    })).count()
}
