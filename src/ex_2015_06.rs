pub fn basic_solve(s: &[(String, (u32, u32), (u32, u32))]) -> u64 {
    s.iter().fold(vec![vec![false; 1000]; 1000], |mut acc, (action, (x1, y1), (x2, y2))| {
        match action.as_str() {
            "toggle" => {
                for i in *x1..=*x2 {
                    for j in *y1..=*y2 {
                        let i = TryInto::<usize>::try_into(i).unwrap();
                        let j = TryInto::<usize>::try_into(j).unwrap();
                        let okok = acc.get_mut(i).unwrap().get_mut(j).unwrap();
                        *okok = !*okok;
                    }
                }
            },
            "turn on" => {
                for i in *x1..=*x2 {
                    for j in *y1..=*y2 {
                        let i = TryInto::<usize>::try_into(i).unwrap();
                        let j = TryInto::<usize>::try_into(j).unwrap();
                        let okok = acc.get_mut(i).unwrap().get_mut(j).unwrap();
                        *okok = true;
                    }
                }
            },
            "turn off" => {
                for i in *x1..=*x2 {
                    for j in *y1..=*y2 {
                        let i = TryInto::<usize>::try_into(i).unwrap();
                        let j = TryInto::<usize>::try_into(j).unwrap();
                        let okok = acc.get_mut(i).unwrap().get_mut(j).unwrap();
                        *okok = false;
                    }
                }
            },
            _ => panic!("unknown action"),
        }
        acc
    }).iter().map(|v| v.iter().filter(|&&l| l).count()).sum::<usize>().try_into().unwrap()
}
