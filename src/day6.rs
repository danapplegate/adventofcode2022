use crate::solvable::Solvable;
use std::collections::HashSet;
use std::fs;
use std::{error::Error, path::PathBuf};

pub struct Solution {
    filepath: PathBuf,
}

fn process_input(input: String, window_size: usize) -> usize {
    let input_slice = &input.chars().collect::<Vec<char>>()[..];
    for (i, window) in input_slice.windows(window_size).enumerate() {
        let mut set: HashSet<char> = HashSet::new();
        for c in window {
            set.insert(*c);
        }
        if set.len() == window_size {
            return i + window_size;
        }
    }
    0
}

impl Solvable<6> for Solution {
    fn new(filename: &str) -> Self {
        Solution {
            filepath: Self::data_path().join(filename),
        }
    }

    fn answer1(&self) -> Result<String, Box<dyn Error>> {
        let input = fs::read_to_string(&self.filepath)?;
        Ok(process_input(input, 4).to_string())
    }

    fn answer2(&self) -> Result<String, Box<dyn Error>> {
        let input = fs::read_to_string(&self.filepath)?;
        Ok(process_input(input, 14).to_string())
    }
}

#[allow(unused_imports)]
mod test {
    use super::*;

    #[test]
    fn it_gives_the_right_first_answer_for_test1() -> Result<(), Box<dyn Error>> {
        let solution = Solution::new("test1.txt");
        assert_eq!(solution.answer1()?, "7");
        Ok(())
    }

    #[test]
    fn it_gives_the_right_first_answer_for_test2() -> Result<(), Box<dyn Error>> {
        let solution = Solution::new("test2.txt");
        assert_eq!(solution.answer1()?, "5");
        Ok(())
    }

    #[test]
    fn it_gives_the_right_first_answer_for_test3() -> Result<(), Box<dyn Error>> {
        let solution = Solution::new("test3.txt");
        assert_eq!(solution.answer1()?, "6");
        Ok(())
    }

    #[test]
    fn it_gives_the_right_first_answer_for_test4() -> Result<(), Box<dyn Error>> {
        let solution = Solution::new("test4.txt");
        assert_eq!(solution.answer1()?, "10");
        Ok(())
    }

    #[test]
    fn it_gives_the_right_first_answer_for_test5() -> Result<(), Box<dyn Error>> {
        let solution = Solution::new("test5.txt");
        assert_eq!(solution.answer1()?, "11");
        Ok(())
    }

    #[test]
    fn it_gives_the_right_second_answer_for_test1() -> Result<(), Box<dyn Error>> {
        let solution = Solution::new("test1.txt");
        assert_eq!(solution.answer2()?, "19");
        Ok(())
    }

    #[test]
    fn it_gives_the_right_second_answer_for_test2() -> Result<(), Box<dyn Error>> {
        let solution = Solution::new("test2.txt");
        assert_eq!(solution.answer2()?, "23");
        Ok(())
    }

    #[test]
    fn it_gives_the_right_second_answer_for_test3() -> Result<(), Box<dyn Error>> {
        let solution = Solution::new("test3.txt");
        assert_eq!(solution.answer2()?, "23");
        Ok(())
    }

    #[test]
    fn it_gives_the_right_second_answer_for_test4() -> Result<(), Box<dyn Error>> {
        let solution = Solution::new("test4.txt");
        assert_eq!(solution.answer2()?, "29");
        Ok(())
    }

    #[test]
    fn it_gives_the_right_second_answer_for_test5() -> Result<(), Box<dyn Error>> {
        let solution = Solution::new("test5.txt");
        assert_eq!(solution.answer2()?, "26");
        Ok(())
    }
}
