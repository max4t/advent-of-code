
pub fn basic_solve(a: &str) -> u64 {
    let reg = regex::Regex::new(r"(?<typt>don)'t\(\)|(?<typ>do)\(\)|(?<typm>mul)\((?<x>\d{1,3}),(?<y>\d{1,3})\)").unwrap();
    let it = reg.captures_iter(a);
    it.scan(true, |state, capt| {
        if let Some(_) = capt.name("typm") {
            if *state {
                Some(capt["x"].parse::<u64>().unwrap() * capt["y"].parse::<u64>().unwrap())
            } else {
                Some(0)
            }
        } else if let Some(_) = capt.name("typt") {
            *state = false;
            Some(0)
        } else if let Some(_) = capt.name("typ") {
            *state = true;
            Some(0)
        } else {
            panic!("unknown command")
        }
    }).sum()
    // .map(|capt| capt["x"].parse::<u64>().unwrap() * capt["y"].parse::<u64>().unwrap()).sum()
}
