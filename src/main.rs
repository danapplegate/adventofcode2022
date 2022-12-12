use adventofcode2022::{day11, solvable::Solvable};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let solution = day11::Solution::new("input.txt");
    println!("{}", solution.answer1()?);
    println!("{}", solution.answer2()?);
    Ok(())
}
