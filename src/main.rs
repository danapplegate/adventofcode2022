#[allow(unused_imports)]
use adventofcode2022::{day1, day2, day3, day4, day5, day6, day7, solvable::Solvable};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let solution = day7::Solution::new("input.txt");
    println!("{}", solution.answer1()?);
    println!("{}", solution.answer2()?);
    Ok(())
}
