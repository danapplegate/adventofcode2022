use std::collections::HashSet;
use std::error::Error;
use std::fs;
use std::path::PathBuf;

use crate::solvable::Solvable;

pub struct Solution {
    filepath: PathBuf,
}

fn char_to_priority(c: &char) -> u32 {
    if c.is_ascii_lowercase() {
        *c as u32 - 96
    } else {
        *c as u32 - 38
    }
}

impl Solvable<3> for Solution {
    fn new(filename: &str) -> Self {
        Solution {
            filepath: Self::data_path().join(filename),
        }
    }

    fn answer1(&self) -> Result<String, Box<dyn Error>> {
        let input = fs::read_to_string(&self.filepath)?;
        let mut sum = 0;
        for line in input.lines() {
            let len = line.len();
            let (first, second) = line.split_at(len / 2);
            let first_set: HashSet<char> = first.chars().collect();
            let second_set: HashSet<char> = second.chars().collect();
            let common_chars = first_set.intersection(&second_set).collect::<Vec<&char>>();
            let common_char = common_chars.first().unwrap();
            sum += char_to_priority(*common_char);
        }
        Ok(format!("{}", sum))
    }

    fn answer2(&self) -> Result<String, Box<dyn Error>> {
        let input = fs::read_to_string(&self.filepath)?;
        let mut lines = input.lines().into_iter();
        let mut sum = 0;
        while let Some(first) = lines.next() {
            let second = lines.next().unwrap().chars().collect::<HashSet<char>>();
            let third = lines.next().unwrap().chars().collect::<HashSet<char>>();
            let common = first
                .chars()
                .filter(|c| second.contains(c) && third.contains(c))
                .collect::<Vec<char>>();
            sum += char_to_priority(common.first().unwrap());
        }
        Ok(format!("{}", sum))
    }
}

#[allow(unused_imports)]
mod test {
    use super::*;

    #[test]
    fn it_gives_the_right_first_answer_for_test1() -> Result<(), Box<dyn Error>> {
        let solution = Solution::new("test1.txt");
        assert_eq!(solution.answer1()?, "157");
        Ok(())
    }

    #[test]
    fn it_gives_the_right_second_answer_for_test1() -> Result<(), Box<dyn Error>> {
        let solution = Solution::new("test1.txt");
        assert_eq!(solution.answer2()?, "70");
        Ok(())
    }
}
