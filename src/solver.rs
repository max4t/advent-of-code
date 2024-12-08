use std::fmt::Display;

pub trait Solver {
    fn part_one(self: &Self) -> impl Display;
    fn part_two(self: &Self) -> impl Display;
}
