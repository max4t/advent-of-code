
pub fn basic_solve(a: &str) -> u64 {
    let reg = regex::Regex::new(r"mul\((?<x>\d{1,3}),(?<y>\d{1,3})\)").unwrap();
    reg.captures_iter(a).map(|capt| capt["x"].parse::<u64>().unwrap() * capt["y"].parse::<u64>().unwrap()).sum()
}
