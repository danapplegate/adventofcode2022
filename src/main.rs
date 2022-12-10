use adventofcode2022::{day9, solvable::Solvable};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let solution = day9::Solution::new("input.txt");
    println!("{}", solution.answer1()?);
    println!("{}", solution.answer2()?);
    Ok(())
}
