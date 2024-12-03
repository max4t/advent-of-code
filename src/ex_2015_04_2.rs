use md5;

pub fn basic_solve(s: &str) -> u64 {
    let mut i = 1_u64;
    loop {
        let digest = md5::compute(s.to_owned() + &i.to_string());
        let hash = format!("{:x}", digest);
        if hash.starts_with("000000") {
            return i
        }
        i += 1;
    }
}
