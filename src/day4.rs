use crate::solvable::Solvable;
use std::fs;
use std::ops::RangeInclusive;
use std::{error::Error, path::PathBuf};

pub struct Solution {
    filepath: PathBuf,
}

fn str_to_range(s: &str) -> Result<RangeInclusive<usize>, Box<dyn Error>> {
    let split: Vec<&str> = s.split('-').collect();
    let start: usize = str::parse::<usize>(split.first().unwrap())?;
    let end: usize = str::parse::<usize>(split.get(1).unwrap())?;
    Ok(RangeInclusive::new(start, end))
}

impl Solution {
    fn get_pairs(
        &self,
    ) -> Result<Vec<(RangeInclusive<usize>, RangeInclusive<usize>)>, Box<dyn Error>> {
        let input = fs::read_to_string(&self.filepath)?;
        Ok(input
            .lines()
            .map(|line| {
                let range_inputs: Vec<&str> = line.split(',').collect();
                let first = *range_inputs.first().unwrap();
                let second = *range_inputs.get(1).unwrap();
                (
                    str_to_range(first).expect(&format!("Error parsing first range of {}", first)),
                    str_to_range(second)
                        .expect(&format!("Error parsing second range of {}", second)),
                )
            })
            .collect())
    }
}

impl Solvable<4> for Solution {
    fn new(filename: &str) -> Self {
        Solution {
            filepath: Self::data_path().join(filename),
        }
    }

    fn answer1(&self) -> Result<String, Box<dyn Error>> {
        let mut num_overlapping = 0;
        let pairs = self.get_pairs()?;
        for pair in pairs {
            let (first, second) = pair;
            if (first.start() <= second.start() && first.end() >= second.end())
                || (second.start() <= first.start() && second.end() >= first.end())
            {
                num_overlapping += 1;
            }
        }
        Ok(format!("{}", num_overlapping))
    }

    fn answer2(&self) -> Result<String, Box<dyn Error>> {
        let mut num_overlapping = 0;
        let pairs = self.get_pairs()?;
        for pair in pairs {
            let (first, second) = pair;
            if (first.start() <= second.end() && first.end() >= second.start())
                || (second.start() <= first.end() && second.end() >= first.start())
            {
                num_overlapping += 1;
            }
        }
        Ok(format!("{}", num_overlapping))
    }
}

#[allow(unused_imports)]
mod test {
    use super::*;

    #[test]
    fn gives_the_right_answer_for_test1() -> Result<(), Box<dyn Error>> {
        let solution = Solution::new("test1.txt");
        assert_eq!(solution.answer1()?, "2");
        Ok(())
    }

    #[test]
    fn gives_the_right_second_answer_for_test1() -> Result<(), Box<dyn Error>> {
        let solution = Solution::new("test1.txt");
        assert_eq!(solution.answer2()?, "4");
        Ok(())
    }
}
