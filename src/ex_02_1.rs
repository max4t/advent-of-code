
pub fn basic_solve(a: &[&[u32]]) -> u32 {
    a.iter().filter(|&&l| {
        assert!(l.len() > 1);
        let diffs = l[..l.len()-1].iter().zip(l[1..].iter()).map::<i64, _>(|(&i, &j)| Into::<i64>::into(i) - Into::<i64>::into(j));
        let (sigs, diffs): (Vec<i64>, Vec<i64>) = diffs.map(|d| (d.signum(), d.abs())).unzip();
        
        diffs.iter().all(|&diff| diff >= 1 && diff <= 3) && (sigs.iter().all(|&l| l == 1) || sigs.iter().all(|&l| l == -1))
    }).count().try_into().unwrap()
}
