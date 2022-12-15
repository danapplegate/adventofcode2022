use adventofcode2022::{day15, solvable::Solvable};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut solution = day15::Solution::new("input.txt");
    solution.set_row_to_inspect(2_000_000);
    solution.set_limit(4_000_000);
    println!("{}", solution.answer1()?);
    println!("{}", solution.answer2()?);
    Ok(())
}
