use std::cmp::Ordering;
use std::error::Error;
use std::fs;
use std::path::PathBuf;

use crate::solvable::Solvable;

pub struct Solution {
    filepath: PathBuf,
}

#[derive(Eq, PartialEq)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl From<char> for Shape {
    fn from(c: char) -> Self {
        match c {
            'A' | 'X' => Self::Rock,
            'B' | 'Y' => Self::Paper,
            'C' | 'Z' => Self::Scissors,
            _ => panic!("Unrecognized character \"{}\"", c),
        }
    }
}

impl Shape {
    fn score(&self) -> u32 {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }
}
enum Outcome {
    Lose,
    Draw,
    Win,
}

impl From<char> for Outcome {
    fn from(c: char) -> Self {
        match c {
            'X' => Self::Lose,
            'Y' => Self::Draw,
            'Z' => Self::Win,
            _ => panic!("Unrecognized Outcome character \"{}\"", c),
        }
    }
}

impl Ord for Shape {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::Rock, Self::Rock)
            | (Self::Paper, Self::Paper)
            | (Self::Scissors, Self::Scissors) => Ordering::Equal,
            (Self::Rock, Self::Scissors)
            | (Self::Paper, Self::Rock)
            | (Self::Scissors, Self::Paper) => Ordering::Greater,
            (Self::Rock, Self::Paper)
            | (Self::Paper, Self::Scissors)
            | (Self::Scissors, Self::Rock) => Ordering::Less,
        }
    }
}

impl PartialOrd for Shape {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Solution {
    fn score_from_lines<F>(&self, score_from_chars: F) -> Result<u32, Box<dyn Error>>
    where
        F: Fn(char, char) -> u32,
    {
        let input = fs::read_to_string(&self.filepath)?;
        let mut score = 0;
        for line in input.lines() {
            let chars = line.chars().into_iter().collect::<Vec<char>>();
            score += score_from_chars(chars[0], chars[2]);
        }
        Ok(score)
    }
}

impl Solvable<2> for Solution {
    fn new(filename: &str) -> Self {
        Solution {
            filepath: Self::data_path().join(filename),
        }
    }

    fn answer1(&self) -> Result<String, Box<dyn Error>> {
        let total_score = self.score_from_lines(|c1, c2| {
            let mut score = 0;
            let (opponent_shape, my_shape): (Shape, Shape) = (c1.into(), c2.into());
            if my_shape > opponent_shape {
                score += 6;
            } else if my_shape == opponent_shape {
                score += 3;
            }
            score += my_shape.score();
            score
        })?;

        Ok(format!("{}", total_score))
    }

    fn answer2(&self) -> Result<String, Box<dyn Error>> {
        let total_score = self.score_from_lines(|c1, c2| {
            let mut score = 0;
            let (opponent_shape, outcome): (Shape, Outcome) = (c1.into(), c2.into());
            score += match outcome {
                Outcome::Win => 6,
                Outcome::Draw => 3,
                Outcome::Lose => 0,
            };
            score += match (outcome, opponent_shape) {
                (Outcome::Draw, my_shape) => my_shape.score(),
                (Outcome::Lose, Shape::Rock) | (Outcome::Win, Shape::Paper) => {
                    Shape::Scissors.score()
                }
                (Outcome::Lose, Shape::Paper) | (Outcome::Win, Shape::Scissors) => {
                    Shape::Rock.score()
                }
                (Outcome::Lose, Shape::Scissors) | (Outcome::Win, Shape::Rock) => {
                    Shape::Paper.score()
                }
            };
            score
        })?;
        Ok(format!("{}", total_score))
    }
}

#[allow(unused_imports)]
mod test {
    use super::*;

    #[test]
    fn gives_the_right_answer_for_test1() -> Result<(), Box<dyn Error>> {
        let solution = Solution::new("test1.txt");
        assert_eq!(solution.answer1()?, "15");
        Ok(())
    }

    #[test]
    fn gives_the_right_second_answer_for_test1() -> Result<(), Box<dyn Error>> {
        let solution = Solution::new("test1.txt");
        assert_eq!(solution.answer2()?, "12");
        Ok(())
    }
}
