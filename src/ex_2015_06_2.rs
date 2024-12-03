pub fn basic_solve(s: &[(String, (u32, u32), (u32, u32))]) -> u64 {
    s.iter().fold(vec![vec![0_u64; 1000]; 1000], |mut acc, (action, (x1, y1), (x2, y2))| {
        match action.as_str() {
            "toggle" => {
                for i in *x1..=*x2 {
                    for j in *y1..=*y2 {
                        let i = TryInto::<usize>::try_into(i).unwrap();
                        let j = TryInto::<usize>::try_into(j).unwrap();
                        let okok = acc.get_mut(i).unwrap().get_mut(j).unwrap();
                        *okok += 2;
                    }
                }
            },
            "turn on" => {
                for i in *x1..=*x2 {
                    for j in *y1..=*y2 {
                        let i = TryInto::<usize>::try_into(i).unwrap();
                        let j = TryInto::<usize>::try_into(j).unwrap();
                        let okok = acc.get_mut(i).unwrap().get_mut(j).unwrap();
                        *okok += 1;
                    }
                }
            },
            "turn off" => {
                for i in *x1..=*x2 {
                    for j in *y1..=*y2 {
                        let i = TryInto::<usize>::try_into(i).unwrap();
                        let j = TryInto::<usize>::try_into(j).unwrap();
                        let okok = acc.get_mut(i).unwrap().get_mut(j).unwrap();
                        if *okok > 0 {
                            *okok -= 1;
                        }
                    }
                }
            },
            _ => panic!("unknown action"),
        }
        acc
    }).iter().map(|v| v.iter().sum::<u64>()).sum::<u64>()
}
