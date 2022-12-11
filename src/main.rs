use adventofcode2022::{day10, solvable::Solvable};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let solution = day10::Solution::new("input.txt");
    println!("{}", solution.answer1()?);
    println!("{}", solution.answer2()?);
    Ok(())
}
