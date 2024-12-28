use std::{fs, io::{self, BufRead}};

use clap::{Parser, ValueEnum};
use solver::Solver;

mod ex_2015_06;
mod ex_2015_06_2;
mod ex_2015_07;
mod ex_2015_07_2;
mod solver;
mod map;
mod y2015;
mod y2024;

macro_rules! default_file {
    ($year:expr, $day:expr) => { format!("examples/inputs/{}/{}", $year, $day) }
}

#[derive(Parser, Debug)]
struct Cli {
    #[arg(help = "year of the exercise")]
    year: String,
    #[arg(help = "day of the year")]
    day: String,
    #[arg(help = "part one or two of the day", default_value_t=Part::One)]
    part: Part,
    #[arg(short, help = "input file (use - for stdin)", default_value_t=default_file!("{year}", "{day}"))]
    file: String,
}

#[derive(ValueEnum, Clone, Debug)]
enum Part {
    #[value(help = "compute part one")]
    One,
    #[value(help = "compute part two")]
    Two,
}

impl ToString for Part {
    fn to_string(&self) -> String {
        match self {
            Part::One => "one",
            Part::Two => "two",
        }.to_string()
    }
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
                                        let ex: [<y20 $t>]::d~N::Problem = if $args.file == "-" {
                                            io::stdin().lines().try_into()?
                                        } else {
                                            let input = if $args.file == default_file!("{year}", "{day}") {
                                                default_file!($args.year, $args.day)
                                            } else {
                                                $args.file
                                            };
                                            fs::read(input)?.lines().try_into()?
                                        };
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
        15 -> 01-05
        24 -> 01-25
    );

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
