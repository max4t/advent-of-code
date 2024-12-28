use std::{io, path::PathBuf};

use clap::{Parser, ValueEnum};
use solver::Solver;

mod ex_2015_01;
mod ex_2015_01_2;
mod ex_2015_02;
mod ex_2015_02_2;
mod ex_2015_03;
mod ex_2015_03_2;
mod ex_2015_04;
mod ex_2015_04_2;
mod ex_2015_05;
mod ex_2015_05_2;
mod ex_2015_06;
mod ex_2015_06_2;
mod ex_2015_07;
mod ex_2015_07_2;
mod solver;
mod y2024;

#[derive(Parser, Debug)]
struct Cli {
    #[arg(help = "year of the exercise")]
    year: String,
    #[arg(help = "day of the year")]
    day: String,
    #[arg(help = "part one or two of the day")]
    part: Part,
    #[arg(short, help = "input file", value_parser = clap::value_parser!(PathBuf))]
    file: String
}

#[derive(ValueEnum, Clone, Debug)]
enum Part {
    #[value(help = "compute part one")]
    One,
    #[value(help = "compute part two")]
    Two,
}

impl Default for Part {
    fn default() -> Self { Part::One }
}

macro_rules! cases {
    ($args:ident $($t:literal -> $s1:literal-$s2:literal)*) => (
        match $args.year.as_str() {
            $(
                stringify!($t) => {
                    seq_macro::seq!(N in $s1..=$s2 {
                        match $args.day.as_str() {
                            #(
                                stringify!(N) => {
                                    paste::paste! {
                                        let ex: [<y20 $t>]::d~N::Problem = io::stdin().lines().try_into()?;
                                    }
                                    match $args.part {
                                        Part::One => println!("Result: {}", ex.part_one()),
                                        Part::Two => println!("Result: {}", ex.part_two()),
                                    };
                                },
                            )*
                            _ => panic!("unknown day"),
                        }
                    })
                },
            )*
            _ => panic!("unknown year"),
        }
    );
}
  
fn main() -> anyhow::Result<()> {
    let args = Cli::parse();
    cases!(args
        24 -> 01-25
    );
    // let args: Vec<_> = env::args().collect();
    // assert!(args.len() == 2, "requires one argument but had {}", args.len());
    // let day = args[1].as_str();
    // match day {
    //     // case!(04/01),
    //     "01-1" => {
    //         let ex: y2024::d01::Problem = io::stdin().try_into()?;
    //         let res = ex.part_one();
    //         println!("Result: {}", res);
    //     },
    //     "01-2" => {
    //         let ex: y2024::d01::Problem = io::stdin().try_into().unwrap();
    //         let res = ex.part_two();
    //         println!("Result: {}", res);
    //     },
    //     "02-1" => {
    //         let ex: y2024::d02::Problem = io::stdin().try_into().unwrap();
    //         let res = ex.part_one();
    //         println!("Result: {}", res);
    //     },
    //     "02-2" => {
    //         let ex: y2024::d02::Problem = io::stdin().try_into().unwrap();
    //         let res = ex.part_two();
    //         println!("Result: {}", res);
    //     },
    //     "03-1" => {
    //         let ex: y2024::d03::Problem = io::stdin().try_into().unwrap();
    //         let res = ex.part_one();
    //         println!("Result: {}", res);
    //     },
    //     "03-2" => {
    //         let ex: y2024::d03::Problem = io::stdin().try_into().unwrap();
    //         let res = ex.part_two();
    //         println!("Result: {}", res);
    //     },
    //     "04-1" => {
    //         let ex: y2024::d04::Problem = io::stdin().try_into().unwrap();
    //         let res = ex.part_one();
    //         println!("Result: {}", res);
    //     },
    //     "04-2" => {
    //         let ex: y2024::d04::Problem = io::stdin().try_into().unwrap();
    //         let res = ex.part_two();
    //         println!("Result: {}", res);
    //     },
    //     "05-1" => {
    //         let ex: y2024::d05::Problem = io::stdin().try_into().unwrap();
    //         let res = ex.part_one();
    //         println!("Result: {}", res);
    //     },
    //     "05-2" => {
    //         let ex: y2024::d05::Problem = io::stdin().try_into().unwrap();
    //         let res = ex.part_two();
    //         println!("Result: {}", res);
    //     },
    //     "2015-01" => {
    //         let a = io::stdin().lines()
    //             .map(|s| s.unwrap()).collect::<Vec<_>>().concat();
    //         let res = ex_2015_01::basic_solve(&a);
    //         println!("Result: {}", res);
    //     },
    //     "2015-01-2" => {
    //         let a = io::stdin().lines()
    //             .map(|s| s.unwrap()).collect::<Vec<_>>().concat();
    //         let res = ex_2015_01_2::basic_solve(&a);
    //         println!("Result: {}", res);
    //     },
    //     "2015-02" => {
    //         let lines = io::stdin().lines()
    //             .map(|s| s.unwrap()).collect::<Vec<_>>();
    //         let a = lines.iter().map(|r| r.split('x').map(|f| f.parse::<i64>().unwrap()).collect::<Vec<_>>()).map(|f| (f[0], f[1], f[2])).collect::<Vec<_>>();
    //         let res = ex_2015_02::basic_solve(&a);
    //         println!("Result: {}", res);
    //     },
    //     "2015-02-2" => {
    //         let lines = io::stdin().lines()
    //             .map(|s| s.unwrap()).collect::<Vec<_>>();
    //         let a = lines.iter().map(|r| r.split('x').map(|f| f.parse::<i64>().unwrap()).collect::<Vec<_>>()).map(|f| (f[0], f[1], f[2])).collect::<Vec<_>>();
    //         let res = ex_2015_02_2::basic_solve(&a);
    //         println!("Result: {}", res);
    //     },
    //     "2015-03" => {
    //         let a = io::stdin().lines()
    //             .map(|s| s.unwrap()).collect::<Vec<_>>().concat();
    //         let res = ex_2015_03::basic_solve(&a);
    //         println!("Result: {}", res);
    //     },
    //     "2015-03-2" => {
    //         let a = io::stdin().lines()
    //             .map(|s| s.unwrap()).collect::<Vec<_>>().concat();
    //         let res = ex_2015_03_2::basic_solve(&a);
    //         println!("Result: {}", res);
    //     },
    //     "2015-04" => {
    //         let a = io::stdin().lines()
    //             .map(|s| s.unwrap()).collect::<Vec<_>>().concat();
    //         let res = ex_2015_04::basic_solve(&a.trim());
    //         println!("Result: {}", res);
    //     },
    //     "2015-04-2" => {
    //         let a = io::stdin().lines()
    //             .map(|s| s.unwrap()).collect::<Vec<_>>().concat();
    //         let res = ex_2015_04_2::basic_solve(&a.trim());
    //         println!("Result: {}", res);
    //     },
    //     "2015-05" => {
    //         let a = io::stdin().lines()
    //             .map(|s| s.unwrap()).collect::<Vec<_>>();
    //         let res = ex_2015_05::basic_solve(&a);
    //         println!("Result: {}", res);
    //     },
    //     "2015-05-2" => {
    //         let a = io::stdin().lines()
    //             .map(|s| s.unwrap()).collect::<Vec<_>>();
    //         let res = ex_2015_05_2::basic_solve(&a);
    //         println!("Result: {}", res);
    //     },
    //     "2015-06" => {
    //         let lines = io::stdin().lines()
    //             .map(|s| s.unwrap()).collect::<Vec<_>>();
    //         let reg = regex::Regex::new(r"(?<typ>[a-z ]+) (?<x1>\d+),(?<y1>\d+) through (?<x2>\d+),(?<y2>\d+)").unwrap();
    //         let a = lines.iter().map(|s| {
    //             let capt = reg.captures(s).unwrap();
    //             (capt["typ"].to_owned(), (capt["x1"].parse::<u32>().unwrap(), capt["y1"].parse::<u32>().unwrap()), (capt["x2"].parse::<u32>().unwrap(), capt["y2"].parse::<u32>().unwrap()))
    //         }).collect::<Vec<_>>();
    //         let res = ex_2015_06::basic_solve(&a);
    //         println!("Result: {}", res);
    //     },
    //     "2015-06-2" => {
    //         let lines = io::stdin().lines()
    //             .map(|s| s.unwrap()).collect::<Vec<_>>();
    //         let reg = regex::Regex::new(r"(?<typ>[a-z ]+) (?<x1>\d+),(?<y1>\d+) through (?<x2>\d+),(?<y2>\d+)").unwrap();
    //         let a = lines.iter().map(|s| {
    //             let capt = reg.captures(s).unwrap();
    //             (capt["typ"].to_owned(), (capt["x1"].parse::<u32>().unwrap(), capt["y1"].parse::<u32>().unwrap()), (capt["x2"].parse::<u32>().unwrap(), capt["y2"].parse::<u32>().unwrap()))
    //         }).collect::<Vec<_>>();
    //         let res = ex_2015_06_2::basic_solve(&a);
    //         println!("Result: {}", res);
    //     },
    //     "2015-07" => {
    //         let lines = io::stdin().lines()
    //             .map(|s| s.unwrap()).collect::<Vec<_>>();
    //         let a = lines.iter().map(|l| l.split_whitespace().map(|s| s.trim()).collect::<Vec<_>>()).collect::<Vec<_>>();
    //         let a = a.iter().map(|l| l.as_slice()).collect::<Vec<_>>();
    //         let res = ex_2015_07::basic_solve(&a);
    //         println!("Result: {}", res);
    //     },
    //     "2015-07-2" => {
    //         let lines = io::stdin().lines()
    //             .map(|s| s.unwrap()).collect::<Vec<_>>();
    //         let a = lines.iter().map(|l| l.split_whitespace().map(|s| s.trim()).collect::<Vec<_>>()).collect::<Vec<_>>();
    //         let a = a.iter().map(|l| l.as_slice()).collect::<Vec<_>>();
    //         let res = ex_2015_07_2::basic_solve(&a);
    //         println!("Result: {}", res);
    //     },
    //     _ => panic!("unknown example"),
    // };
    Ok(())
}
