use std::{env, io};

mod ex_01_1;
mod ex_01_2;
mod ex_02_1;
mod ex_02_2;
mod ex_2015_01;
mod ex_2015_01_2;
mod ex_2015_02;
mod ex_2015_02_2;
mod ex_2015_03;
mod ex_2015_03_2;
mod ex_2015_04;
mod ex_2015_04_2;

fn main() {
    let args: Vec<_> = env::args().collect();
    assert!(args.len() == 2, "requires one argument but had {}", args.len());
    let day = args[1].as_str();
    match day {
        "01-1" => {
            let (mut a, mut b): (Vec<_>, Vec<_>) = io::stdin().lines()
                .map(|s| s.unwrap())
                .map(|s| s.trim().to_owned())
                .map(|s| {
                    let res= s.split_whitespace().collect::<Vec<_>>();
                    assert!(res.len() == 2);
                    (res[0].parse::<u32>().unwrap(), res[1].parse::<u32>().unwrap())
                })
                .unzip();
            let res = ex_01_1::basic_solve(&mut a, &mut b);
            println!("Result: {}", res);
        },
        "01-2" => {
            let (mut a, mut b): (Vec<_>, Vec<_>) = io::stdin().lines()
                .map(|s| s.unwrap())
                .map(|s| s.trim().to_owned())
                .map(|s| {
                    let res= s.split_whitespace().collect::<Vec<_>>();
                    assert!(res.len() == 2);
                    (res[0].parse::<u32>().unwrap(), res[1].parse::<u32>().unwrap())
                })
                .unzip();
            let res = ex_01_2::basic_solve(&mut a, &mut b);
            println!("Result: {}", res);
        },
        "02-1" => {
            let lines = io::stdin().lines()
                .map(|s| s.unwrap()).collect::<Vec<_>>();
            let a = lines.iter()
                .map(|s| s.trim())
                .map(|s| {
                    s.split_whitespace().map(|s| s.parse::<u32>().unwrap()).collect::<Vec<_>>()
                }).collect::<Vec<_>>();
            let a = a.iter().map(|s| s.as_slice()).collect::<Vec<_>>();
            let res = ex_02_1::basic_solve(&a);
            println!("Result: {}", res);
        },
        "02-2" => {
            let lines = io::stdin().lines()
                .map(|s| s.unwrap()).collect::<Vec<_>>();
            let a = lines.iter()
                .map(|s| s.trim())
                .map(|s| {
                    s.split_whitespace().map(|s| s.parse::<u32>().unwrap()).collect::<Vec<_>>()
                }).collect::<Vec<_>>();
            let a = a.iter().map(|s| s.as_slice()).collect::<Vec<_>>();
            let res = ex_02_2::basic_solve(&a);
            println!("Result: {}", res);
        },
        "2015-01" => {
            let a = io::stdin().lines()
                .map(|s| s.unwrap()).collect::<Vec<_>>().concat();
            let res = ex_2015_01::basic_solve(&a);
            println!("Result: {}", res);
        },
        "2015-01-2" => {
            let a = io::stdin().lines()
                .map(|s| s.unwrap()).collect::<Vec<_>>().concat();
            let res = ex_2015_01_2::basic_solve(&a);
            println!("Result: {}", res);
        },
        "2015-02" => {
            let lines = io::stdin().lines()
                .map(|s| s.unwrap()).collect::<Vec<_>>();
            let a = lines.iter().map(|r| r.split('x').map(|f| f.parse::<i64>().unwrap()).collect::<Vec<_>>()).map(|f| (f[0], f[1], f[2])).collect::<Vec<_>>();
            let res = ex_2015_02::basic_solve(&a);
            println!("Result: {}", res);
        },
        "2015-02-2" => {
            let lines = io::stdin().lines()
                .map(|s| s.unwrap()).collect::<Vec<_>>();
            let a = lines.iter().map(|r| r.split('x').map(|f| f.parse::<i64>().unwrap()).collect::<Vec<_>>()).map(|f| (f[0], f[1], f[2])).collect::<Vec<_>>();
            let res = ex_2015_02_2::basic_solve(&a);
            println!("Result: {}", res);
        },
        "2015-03" => {
            let a = io::stdin().lines()
                .map(|s| s.unwrap()).collect::<Vec<_>>().concat();
            let res = ex_2015_03::basic_solve(&a);
            println!("Result: {}", res);
        },
        "2015-03-2" => {
            let a = io::stdin().lines()
                .map(|s| s.unwrap()).collect::<Vec<_>>().concat();
            let res = ex_2015_03_2::basic_solve(&a);
            println!("Result: {}", res);
        },
        "2015-04" => {
            let a = io::stdin().lines()
                .map(|s| s.unwrap()).collect::<Vec<_>>().concat();
            let res = ex_2015_04::basic_solve(&a.trim());
            println!("Result: {}", res);
        },
        "2015-04-2" => {
            let a = io::stdin().lines()
                .map(|s| s.unwrap()).collect::<Vec<_>>().concat();
            let res = ex_2015_04_2::basic_solve(&a.trim());
            println!("Result: {}", res);
        },
        _ => panic!("unknown example"),
    }
}
