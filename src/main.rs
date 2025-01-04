use std::{fs, io::{self, BufRead}};

use clap::{Parser, ValueEnum};
use solver::Solver;

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
        15 -> 01-21
        24 -> 01-25
    );

    Ok(())
}
