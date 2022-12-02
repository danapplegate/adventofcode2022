use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let solution = day1::Solution::new("data/1/1.txt");
    println!("{}", solution.answer1()?);
    println!("{}", solution.answer2()?);
    Ok(())
}

mod day1 {
    use std::fs;
    use std::{error::Error, path::PathBuf};

    pub struct Solution {
        filepath: PathBuf,
    }

    impl Solution {
        pub fn new(filepath: impl Into<PathBuf>) -> Self {
            Solution {
                filepath: filepath.into(),
            }
        }

        pub fn answer1(&self) -> Result<String, Box<dyn Error>> {
            let max = self.get_elves()?.into_iter().max().ok_or("Empty input")?;

            Ok(format!("{}", max))
        }

        pub fn answer2(&self) -> Result<String, Box<dyn Error>> {
            let mut elves = self.get_elves()?;
            elves.sort_by(|a, b| b.partial_cmp(a).unwrap());
            let sum = elves[..3].into_iter().sum::<u32>();
            Ok(sum.to_string())
        }

        fn get_elves(&self) -> Result<Vec<u32>, Box<dyn Error>> {
            let input = fs::read_to_string(&self.filepath)?;
            let mut elves: Vec<u32> = Vec::new();
            let mut elf: u32 = 0;
            for line in input.lines() {
                if line == "" {
                    elves.push(elf);
                    elf = 0;
                } else {
                    elf += line.parse::<u32>()?;
                }
            }

            // At end of file, push the final elf
            elves.push(elf);

            Ok(elves)
        }
    }

    #[allow(unused_imports)]
    mod test {
        use super::*;

        #[test]
        fn gives_the_right_answer_for_test1() -> Result<(), Box<dyn Error>> {
            let solution = Solution::new("data/1/test1.txt");
            assert_eq!(solution.answer1()?, "24000");
            Ok(())
        }

        #[test]
        fn gives_the_right_second_answer_for_test1() -> Result<(), Box<dyn Error>> {
            let solution = Solution::new("data/1/test1.txt");
            assert_eq!(solution.answer2()?, "45000");
            Ok(())
        }
    }
}
