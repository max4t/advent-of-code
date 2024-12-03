use std::collections::HashMap;

pub fn basic_solve(s: &str) -> i64 {
    let mut m = HashMap::new();
    m.entry((0, 0)).or_insert(1);

    s.chars().fold((0, 0), |(x, y), v| {
        let pos = match v {
            '<' => (x - 1, y),
            '^' => (x, y + 1),
            '>' => (x + 1, y),
            'v' => (x, y - 1),
            _ => panic!("invalid char"),
        };
        m.entry(pos).and_modify(|v| *v += 1).or_insert(1);
        pos
    });

    m.values().filter(|&&l| l > 0).count().try_into().unwrap()
}
