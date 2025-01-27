use std::{fs, io::{self, BufRead}};
use rand::prelude::*;

use clap::{Args, Parser, Subcommand, ValueEnum};
use solver::Solver;

mod solver;
mod map;
mod game;
mod y2015;
mod y2016;
mod y2017;
mod y2018;
mod y2019;
mod y2023;
mod y2024;

macro_rules! default_file {
    ($year:expr, $day:expr) => { format!("examples/inputs/{}/{}", $year, $day) }
}

#[derive(Subcommand)]
enum Command {
    #[command(about = "Execute a specific day")]
    Run(Run),
    #[command(about = "Randomly choose the next available challenge")]
    Pick,
    #[command(about = "Display progress per year")]
    Progress,
}

#[derive(Args)]
struct Run {
    #[arg(help = "year of the exercise")]
    year: String,
    #[arg(help = "day of the year")]
    day: String,
    #[arg(help = "part one or two of the day", default_value_t=Part::One)]
    part: Part,
    #[arg(short, help = "input file (use - for stdin)", default_value_t=default_file!("{year}", "{day}"))]
    file: String,
}

#[derive(Parser)]
struct Cli {
    // #[arg(help = "year of the exercise")]
    // year: String,
    // #[arg(help = "day of the year")]
    // day: String,
    // #[arg(help = "part one or two of the day", default_value_t=Part::One)]
    // part: Part,
    // #[arg(short, help = "input file (use - for stdin)", default_value_t=default_file!("{year}", "{day}"))]
    // file: String,
    #[command(subcommand)]
    command: Command,
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

macro_rules! case {
    ($args:ident $t:literal -> ) => {
    };
    ($args:ident $t:literal -> $s1:literal-$s2:literal) => {
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
    };
}
macro_rules! available_years {
    ($([$t:literal $($s2:tt)?])*) => {
        available_years!([], $([$t $($s2)?])*)
    };
    ([$($year:literal,)*], ) => {
        [$($year,)*]
    };
    ([$($year:literal,)*], [$_:literal 25] $([$t:literal $($s2:tt)?])*) => {
        available_years!([$($year,)*], $([$t $($s2)?])*)
    };
    ([$($year:literal,)*], [$t:literal $($s2:tt)?] $([$ts:literal $($s2s:tt)?])*) => {
        available_years!([$($year,)* $t,], $([$ts $($s2s)?])*)
    };
}
macro_rules! cases {
    ($args:ident $($t:literal -> $($s1:literal-$s2:tt)?,)*) => (
        match $args.command {
            Command::Run(run) => match run.year.as_str() {
                $(
                    stringify!($t) => {
                        case!(run $t -> $($s1-$s2)?);
                    },
                )*
                _ => panic!("unknown year"),
            },
            Command::Pick => {
                let years = available_years!($([$t $($s2)?])*);
                let mut rng = rand::thread_rng();
                let year = years.choose(&mut rng).unwrap();
                println!("Go to https://adventofcode.com/20{}/day/{}", year, match year.to_string().as_str() {
                    $(
                        stringify!($t) => match stringify!($($s2)?).trim_start_matches('0') {
                            "" => "0",
                            e => e,
                        }.parse::<u8>().unwrap() + 1,
                    )*
                    _ => panic!("unknown year"),
                });
            },
            Command::Progress => {
                $(
                    let progress = match stringify!($($s2)?).trim_start_matches('0') {
                        "" => "0",
                        e => e,
                    }.parse::<usize>().unwrap()*200/50;
                    println!("20{}   {} {:3}%", $t, {
                        let p = progress*SIZE*(CODE.len())/100;
                        (0..SIZE).into_iter().map(|l| p.saturating_sub(l*CODE.len()).min(CODE.len() - 1)).map(|l| CODE[l as usize]).collect::<String>()
                    }, progress);
                )*
                let progress = [$(match stringify!($($s2)?).trim_start_matches('0') {
                    "" => "0",
                    e => e,
                }.parse::<usize>().unwrap(),)*];
                let len = progress.len();
                let progress = progress.iter().sum::<usize>()*4/len;
                println!("Global {} {:3}%", {
                    let p = progress*SIZE*CODE.len()/100;
                    (0..SIZE).into_iter().map(|l| p.saturating_sub(l*CODE.len()).min(CODE.len() - 1)).map(|l| CODE[l as usize]).collect::<String>()
                }, progress);
                println!("");
                (0..=100).for_each(|l| {
                    println!("{}", {
                        let p = l*SIZE*CODE.len()/100;
                        (0..SIZE).into_iter().map(|l| p.saturating_sub(l*CODE.len()).min(CODE.len() - 1)).map(|l| CODE[l as usize]).collect::<String>()
                    });    
                });
            }
        }
    );
}

const SIZE: usize = 30;
const CODE: [char; 7] = ['\u{28C0}', '\u{28C4}', '\u{28C6}',  '\u{28C7}', '\u{28E7}', '\u{28F7}', '\u{28FF}'];

fn main() -> anyhow::Result<()> {
    let args = Cli::parse();

    cases!(args
        15 -> 01-25,
        16 -> 01-04,
        17 -> 01-02,
        18 -> 01-03,
        19 -> 01-01,
        20 -> ,
        21 -> ,
        22 -> ,
        23 -> 01-01,
        24 -> 01-25,
    );

    Ok(())
}
