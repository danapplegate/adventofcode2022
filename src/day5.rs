use crate::solvable::Solvable;
use std::collections::VecDeque;
use std::fs;
use std::{error::Error, path::PathBuf};

pub struct Solution {
    filepath: PathBuf,
}

fn get_input_parts(input: &str) -> Result<(String, String), Box<dyn Error>> {
    let parts: Vec<&str> = input.split("\n\n").collect();
    Ok((
        String::from(*parts.first().unwrap()),
        String::from(*parts.get(1).unwrap()),
    ))
}

fn build_crate_stacks(input: &str) -> Result<Vec<Vec<char>>, Box<dyn Error>> {
    let stack_numbers = input.lines().last().unwrap();
    let num_stacks = stack_numbers
        .split_ascii_whitespace()
        .collect::<Vec<&str>>()
        .len();
    let mut stacks = vec![];
    for _ in 0..num_stacks {
        stacks.push(Vec::new());
    }
    for line in input.lines().rev().skip(1) {
        for i in 0..num_stacks {
            let start_idx = i * 4;
            let end_idx = start_idx + 3;
            if let ['[', c, ']'] = line[start_idx..end_idx]
                .chars()
                .take(3)
                .collect::<Vec<char>>()[..]
            {
                stacks[i].push(c);
            }
        }
    }
    Ok(stacks)
}

impl Solvable<5> for Solution {
    fn new(filename: &str) -> Self {
        Solution {
            filepath: Self::data_path().join(filename),
        }
    }

    fn answer1(&self) -> Result<String, Box<dyn Error>> {
        let input = fs::read_to_string(&self.filepath)?;
        let (stack_input, instructions) = get_input_parts(&input)?;
        let mut stacks = build_crate_stacks(&stack_input).unwrap();
        for line in instructions.lines() {
            // move 1 from 2 to 1
            let tokens: Vec<&str> = line.split_ascii_whitespace().collect();
            let count = str::parse::<usize>(tokens[1])?;
            let src = str::parse::<usize>(tokens[3])? - 1;
            let dst = str::parse::<usize>(tokens[5])? - 1;

            let src_stack = stacks.get_mut(src).unwrap();
            let mut temp = vec![];
            for _ in 0..count {
                let c = src_stack.pop().unwrap();
                temp.push(c);
            }
            stacks.get_mut(dst).unwrap().extend(temp);
        }

        let mut tops = String::new();
        stacks
            .iter()
            .map(|s| s.last().unwrap())
            .for_each(|c| tops.push(*c));

        Ok(String::from(tops))
    }

    fn answer2(&self) -> Result<String, Box<dyn Error>> {
        let input = fs::read_to_string(&self.filepath)?;
        let (stack_input, instructions) = get_input_parts(&input)?;
        let mut stacks = build_crate_stacks(&stack_input).unwrap();
        for line in instructions.lines() {
            // move 1 from 2 to 1
            let tokens: Vec<&str> = line.split_ascii_whitespace().collect();
            let count = str::parse::<usize>(tokens[1])?;
            let src = str::parse::<usize>(tokens[3])? - 1;
            let dst = str::parse::<usize>(tokens[5])? - 1;

            let src_stack = stacks.get_mut(src).unwrap();
            let mut temp: VecDeque<char> = VecDeque::new();
            for _ in 0..count {
                let c = src_stack.pop().unwrap();
                temp.push_front(c);
            }
            stacks.get_mut(dst).unwrap().extend(temp);
        }

        let mut tops = String::new();
        stacks
            .iter()
            .map(|s| s.last().unwrap())
            .for_each(|c| tops.push(*c));

        Ok(String::from(tops))
    }
}

#[allow(unused_imports)]
mod test {
    use super::*;

    #[test]
    fn gives_the_right_answer_for_test1() -> Result<(), Box<dyn Error>> {
        let solution = Solution::new("test1.txt");
        assert_eq!(solution.answer1()?, "CMZ");
        Ok(())
    }

    #[test]
    fn gives_the_right_second_answer_for_test1() -> Result<(), Box<dyn Error>> {
        let solution = Solution::new("test1.txt");
        assert_eq!(solution.answer2()?, "MCD");
        Ok(())
    }
}
