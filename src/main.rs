#[allow(unused_imports)]
use adventofcode2022::{day1, day2, solvable::Solvable};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let solution = day2::Solution::new("input.txt");
    println!("{}", solution.answer1()?);
    println!("{}", solution.answer2()?);
    Ok(())
}
